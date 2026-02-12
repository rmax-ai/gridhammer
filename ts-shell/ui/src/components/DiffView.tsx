import React from 'react';
import type { EvseStateRecord, PowerSeriesRecord, EvseState, OcppTraceRecord } from '../types';
import { ChargingProfileDiff } from './ChargingProfileDiff';

interface RunData {
  evseData: EvseStateRecord[] | null;
  powerData: PowerSeriesRecord[] | null;
  traceData: OcppTraceRecord[] | null;
  label: string;
}

interface Props {
  base: RunData;
  comparison: RunData;
}

const STATE_COLORS: Record<EvseState, string> = {
  "Available": '#10b981',
  "Preparing": '#3b82f6',
  "Charging": '#f59e0b',
  "SuspendedEV": '#eab308',
  "SuspendedEVSE": '#ef4444',
  "Faulted": '#991b1b',
  "Unavailable": '#64748b',
  "Reserved": '#8b5cf6',
};

const BASE_COLORS = ['#3b82f6', '#10b981', '#8b5cf6', '#06b6d4'];
const COMP_COLORS = ['#ef4444', '#f59e0b', '#ec4899', '#db2777'];

export const DiffView: React.FC<Props> = ({ base, comparison }) => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '40px' }}>
      <div style={{ padding: '0 20px' }}>
        <h2 style={{ marginBottom: '10px' }}>Comparison Analysis</h2>
        <p style={{ color: '#64748b', margin: 0 }}>Comparing <strong>{base.label}</strong> vs <strong>{comparison.label}</strong></p>
      </div>

      {(base.powerData || comparison.powerData) && (
        <div className="diff-section">
          <PowerCurveDiff 
            baseData={base.powerData || []} 
            compData={comparison.powerData || []} 
            baseLabel={base.label}
            compLabel={comparison.label}
          />
        </div>
      )}

      {(base.evseData || comparison.evseData) && (
        <div className="diff-section">
          <EvseTimelineDiff 
            baseData={base.evseData || []} 
            compData={comparison.evseData || []} 
            baseLabel={base.label}
            compLabel={comparison.label}
          />
        </div>
      )}

      {(base.traceData || comparison.traceData) && (
        <div className="diff-section">
          <ChargingProfileDiff 
            baseTrace={base.traceData || []} 
            compTrace={comparison.traceData || []} 
            baseLabel={base.label}
            compLabel={comparison.label}
          />
        </div>
      )}
    </div>
  );
};

const PowerCurveDiff: React.FC<{ 
  baseData: PowerSeriesRecord[], 
  compData: PowerSeriesRecord[],
  baseLabel: string,
  compLabel: string
}> = ({ baseData, compData, baseLabel, compLabel }) => {
  const allData = [...baseData, ...compData];
  const maxTick = Math.max(...allData.map(r => r.tick), 0);
  const minTick = Math.min(...allData.map(r => r.tick), 0);
  const maxPower = Math.max(...allData.map(r => r.power_w), 1000);

  const width = 1000;
  const height = 400;
  const padding = 50;
  const chartWidth = width - padding * 2;
  const chartHeight = height - padding * 2;

  const tickRange = maxTick - minTick || 1;
  const powerRange = maxPower * 1.1;

  const getX = (tick: number) => padding + ((tick - minTick) / tickRange) * chartWidth;
  const getY = (power: number) => height - padding - (power / powerRange) * chartHeight;

  const renderLines = (data: PowerSeriesRecord[], colors: string[], isDashed: boolean) => {
    const grouped: Record<number, PowerSeriesRecord[]> = {};
    data.forEach(r => {
      if (!grouped[r.evse_id]) grouped[r.evse_id] = [];
      grouped[r.evse_id].push(r);
    });

    return Object.entries(grouped).map(([evseId, records]) => {
      records.sort((a, b) => a.tick - b.tick);
      const points = records.map(r => `${getX(r.tick)},${getY(r.power_w)}`).join(' ');
      const color = colors[Number(evseId) % colors.length];
      return (
        <polyline
          key={evseId}
          points={points}
          fill="none"
          stroke={color}
          strokeWidth="2"
          strokeDasharray={isDashed ? "5,5" : "0"}
          opacity="0.8"
        />
      );
    });
  };

  return (
    <div>
      <h3 style={{ marginBottom: '20px' }}>Power Curve Comparison</h3>
      <div style={{ display: 'flex', gap: '30px', marginBottom: '15px', padding: '10px', background: '#f8fafc', borderRadius: '6px' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <div style={{ width: '24px', height: '3px', backgroundColor: '#3b82f6' }}></div>
          <span style={{ fontSize: '14px', fontWeight: 500 }}>{baseLabel} (Solid)</span>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <div style={{ width: '24px', height: '3px', borderBottom: '3px dashed #ef4444' }}></div>
          <span style={{ fontSize: '14px', fontWeight: 500 }}>{compLabel} (Dashed)</span>
        </div>
      </div>
      <svg width="100%" viewBox={`0 0 ${width} ${height}`} style={{ overflow: 'visible' }}>
        {[0, 0.25, 0.5, 0.75, 1].map(p => {
          const y = getY(p * maxPower);
          return (
            <g key={p}>
              <line x1={padding} y1={y} x2={width - padding} y2={y} stroke="#e2e8f0" strokeDasharray="4 4" />
              <text x={padding - 10} y={y} textAnchor="end" dominantBaseline="middle" style={{ fontSize: '12px', fill: '#64748b' }}>
                {Math.round(p * maxPower / 1000)}k
              </text>
            </g>
          );
        })}
        
        <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="#475569" strokeWidth="2" />
        <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="#475569" strokeWidth="2" />
        
        {renderLines(baseData, BASE_COLORS, false)}
        {renderLines(compData, COMP_COLORS, true)}
      </svg>
    </div>
  );
};

const EvseTimelineDiff: React.FC<{ 
    baseData: EvseStateRecord[], 
    compData: EvseStateRecord[],
    baseLabel: string,
    compLabel: string
}> = ({ baseData, compData, baseLabel, compLabel }) => {
  const allRecords = [...baseData, ...compData];
  const evseIds = Array.from(new Set(allRecords.map(r => r.evse_id))).sort((a, b) => a - b);
  const maxTick = Math.max(...allRecords.map(r => r.tick), 0);
  const minTick = Math.min(...allRecords.map(r => r.tick), 0);
  const tickRange = maxTick - minTick || 1;

  const width = 1000;
  const rowHeight = 24;
  const padding = 20;
  const labelWidth = 140;
  const chartWidth = width - labelWidth - padding * 2;
  const height = evseIds.length * (rowHeight * 2 + 15) + padding * 2;

  const groupRecords = (data: EvseStateRecord[]) => {
      const grouped: Record<number, EvseStateRecord[]> = {};
      data.forEach(r => {
          if (!grouped[r.evse_id]) grouped[r.evse_id] = [];
          grouped[r.evse_id].push(r);
      });
      Object.values(grouped).forEach(g => g.sort((a, b) => a.tick - b.tick));
      return grouped;
  };

  const baseGrouped = groupRecords(baseData);
  const compGrouped = groupRecords(compData);

  return (
    <div>
      <h3 style={{ marginBottom: '20px' }}>Timeline Comparison</h3>
      <svg width="100%" viewBox={`0 0 ${width} ${height}`} style={{ overflow: 'visible' }}>
        {evseIds.map((evseId, idx) => {
          const yBase = padding + idx * (rowHeight * 2 + 15);
          const yComp = yBase + rowHeight;

          const renderRow = (records: EvseStateRecord[] | undefined, y: number, label: string, isBase: boolean) => (
            <g>
              <text x={padding} y={y + rowHeight / 2} dominantBaseline="middle" style={{ fontSize: '11px', fontWeight: 500, fill: isBase ? '#2563eb' : '#db2777' }}>
                EVSE {evseId} ({label})
              </text>
              <rect x={labelWidth} y={y + 2} width={chartWidth} height={rowHeight - 4} fill="#f8fafc" rx="2" />
              {records?.map((record, i) => {
                const nextTick = records[i + 1]?.tick ?? maxTick;
                const startX = labelWidth + ((record.tick - minTick) / tickRange) * chartWidth;
                const segmentWidth = ((nextTick - record.tick) / tickRange) * chartWidth;
                return (
                  <rect
                    key={i}
                    x={startX}
                    y={y + 4}
                    width={Math.max(segmentWidth, 1)}
                    height={rowHeight - 8}
                    fill={STATE_COLORS[record.state] || '#000'}
                    rx="1"
                    opacity="0.9"
                  />
                );
              })}
            </g>
          );

          return (
            <g key={evseId}>
              {renderRow(baseGrouped[evseId], yBase, baseLabel, true)}
              {renderRow(compGrouped[evseId], yComp, compLabel, false)}
              <line x1={padding} y1={yComp + rowHeight + 6} x2={width - padding} y2={yComp + rowHeight + 6} stroke="#f1f5f9" strokeWidth="1" />
            </g>
          );
        })}
      </svg>
      <div style={{ marginTop: '10px', display: 'flex', gap: '10px', flexWrap: 'wrap', justifyContent: 'center' }}>
        {Object.entries(STATE_COLORS).map(([state, color]) => (
          <div key={state} style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
            <div style={{ width: '10px', height: '10px', borderRadius: '2px', backgroundColor: color }}></div>
            <span style={{ fontSize: '11px', color: '#64748b' }}>{state}</span>
          </div>
        ))}
      </div>
    </div>
  );
};
