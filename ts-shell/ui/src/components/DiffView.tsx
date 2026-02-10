import React from 'react';
import type { EvseStateRecord, PowerSeriesRecord, EvseState } from '../types';

interface RunData {
  evseData: EvseStateRecord[] | null;
  powerData: PowerSeriesRecord[] | null;
  label: string;
}

interface Props {
  base: RunData;
  comparison: RunData;
}

const STATE_COLORS: Record<EvseState, string> = {
  "Available": '#4caf50',
  "Preparing": '#2196f3',
  "Charging": '#ff9800',
  "SuspendedEV": '#ffeb3b',
  "SuspendedEVSE": '#f44336',
  "Faulted": '#b71c1c',
  "Unavailable": '#9e9e9e',
  "Reserved": '#9c27b0',
};

const BASE_COLORS = ['#2196f3', '#4caf50', '#9c27b0', '#009688'];
const COMP_COLORS = ['#f44336', '#ff9800', '#795548', '#e91e63'];

export const DiffView: React.FC<Props> = ({ base, comparison }) => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '40px' }}>
      {(base.powerData || comparison.powerData) && (
        <PowerCurveDiff 
          baseData={base.powerData || []} 
          compData={comparison.powerData || []} 
          baseLabel={base.label}
          compLabel={comparison.label}
        />
      )}
      {(base.evseData || comparison.evseData) && (
        <EvseTimelineDiff 
          baseData={base.evseData || []} 
          compData={comparison.evseData || []} 
          baseLabel={base.label}
          compLabel={comparison.label}
        />
      )}
      <div style={{ marginTop: '20px', display: 'flex', gap: '10px', flexWrap: 'wrap', justifyContent: 'center' }}>
        {Object.entries(STATE_COLORS).map(([state, color]) => (
          <div key={state} style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
            <div style={{ width: '12px', height: '12px', backgroundColor: color }}></div>
            <span style={{ fontSize: '12px' }}>{state}</span>
          </div>
        ))}
      </div>
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
  const padding = 40;
  const chartWidth = width - padding * 2;
  const chartHeight = height - padding * 2;

  const tickRange = maxTick - minTick || 1;
  const powerRange = maxPower || 1;

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
      <h2>Power Curve Diff</h2>
      <div style={{ display: 'flex', gap: '20px', marginBottom: '10px' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
          <div style={{ width: '20px', height: '2px', backgroundColor: '#333' }}></div>
          <span>{baseLabel} (Solid)</span>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
          <div style={{ width: '20px', height: '2px', borderBottom: '2px dashed #333' }}></div>
          <span>{compLabel} (Dashed)</span>
        </div>
      </div>
      <svg width={width} height={height} style={{ border: '1px solid #ddd' }}>
        {/* Grids and Axes (simplified from PowerCurve) */}
        <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="black" />
        <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="black" />
        
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
  const rowHeight = 30;
  const padding = 20;
  const labelWidth = 150;
  const chartWidth = width - labelWidth - padding * 2;
  const height = evseIds.length * (rowHeight * 2 + 10) + padding * 2;

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
      <h2>EVSE Timeline Diff</h2>
      <svg width={width} height={height} style={{ border: '1px solid #ddd' }}>
        {evseIds.map((evseId, idx) => {
          const yBase = padding + idx * (rowHeight * 2 + 10);
          const yComp = yBase + rowHeight;

          const renderRow = (records: EvseStateRecord[] | undefined, y: number, label: string) => (
            <g>
              <text x={padding} y={y + rowHeight / 2} dominantBaseline="middle" style={{ fontSize: '10px' }}>
                EVSE {evseId} ({label})
              </text>
              <rect x={labelWidth} y={y} width={chartWidth} height={rowHeight - 2} fill="#f9f9f9" stroke="#eee" />
              {records?.map((record, i) => {
                const nextTick = records[i + 1]?.tick ?? maxTick;
                const startX = labelWidth + ((record.tick - minTick) / tickRange) * chartWidth;
                const segmentWidth = ((nextTick - record.tick) / tickRange) * chartWidth;
                return (
                  <rect
                    key={i}
                    x={startX}
                    y={y + 2}
                    width={Math.max(segmentWidth, 1)}
                    height={rowHeight - 6}
                    fill={STATE_COLORS[record.state] || '#000'}
                  >
                    <title>{`${record.state} @ ${record.tick}`}</title>
                  </rect>
                );
              })}
            </g>
          );

          return (
            <g key={evseId}>
              {renderRow(baseGrouped[evseId], yBase, baseLabel)}
              {renderRow(compGrouped[evseId], yComp, compLabel)}
              <line x1={padding} y1={yComp + rowHeight + 4} x2={width - padding} y2={yComp + rowHeight + 4} stroke="#eee" />
            </g>
          );
        })}
      </svg>
    </div>
  );
};
