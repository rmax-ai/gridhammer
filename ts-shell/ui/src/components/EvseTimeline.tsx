import React, { useMemo, useState } from 'react';
import type { EvseStateRecord, EvseState } from '../types';

interface Props {
  records: EvseStateRecord[];
}

const STATE_COLORS: Record<EvseState, string> = {
  "Available": '#10b981', // Emerald 500
  "Preparing": '#3b82f6', // Blue 500
  "Charging": '#f59e0b',  // Amber 500
  "SuspendedEV": '#eab308', // Yellow 500
  "SuspendedEVSE": '#ef4444', // Red 500
  "Faulted": '#991b1b',   // Red 800
  "Unavailable": '#64748b', // Slate 500
  "Reserved": '#8b5cf6',  // Violet 500
};

export const EvseTimeline: React.FC<Props> = ({ records }) => {
  const [hoveredSegment, setHoveredSegment] = useState<EvseStateRecord | null>(null);

  const groupedRecords = useMemo(() => {
    const groups: Record<number, EvseStateRecord[]> = {};
    records.forEach(r => {
      if (!groups[r.evse_id]) groups[r.evse_id] = [];
      groups[r.evse_id].push(r);
    });
    Object.values(groups).forEach(g => g.sort((a, b) => a.tick - b.tick));
    return groups;
  }, [records]);

  const evseIds = useMemo(() => Object.keys(groupedRecords).map(Number).sort((a, b) => a - b), [groupedRecords]);
  const maxTick = useMemo(() => Math.max(...records.map(r => r.tick), 0), [records]);
  const minTick = useMemo(() => Math.min(...records.map(r => r.tick), 0), [records]);
  const tickRange = maxTick - minTick || 1;

  const width = 1000;
  const rowHeight = 45;
  const padding = 20;
  const labelWidth = 80;
  const chartWidth = width - labelWidth - padding * 2;
  const height = evseIds.length * rowHeight + padding * 2 + 30;

  return (
    <div className="timeline-container">
      <h3>EVSE State Timelines</h3>
      <div style={{ position: 'relative' }}>
        <svg width="100%" viewBox={`0 0 ${width} ${height}`} style={{ overflow: 'visible' }}>
            {evseIds.map((evseId, rowIndex) => {
            const evseRecords = groupedRecords[evseId];
            const y = padding + rowIndex * rowHeight;

            return (
                <g key={evseId}>
                <text x={padding} y={y + rowHeight / 2} dominantBaseline="middle" style={{ fontSize: '13px', fontWeight: 600, fill: '#475569' }}>
                    EVSE {evseId}
                </text>

                <rect 
                    x={labelWidth} 
                    y={y + 5} 
                    width={chartWidth} 
                    height={rowHeight - 10} 
                    fill="#f8fafc" 
                    rx="4"
                />

                {evseRecords.map((record, i) => {
                    const nextTick = evseRecords[i + 1]?.tick ?? maxTick;
                    const startX = labelWidth + ((record.tick - minTick) / tickRange) * chartWidth;
                    const segmentWidth = ((nextTick - record.tick) / tickRange) * chartWidth;

                    return (
                    <rect
                        key={i}
                        x={startX}
                        y={y + 8}
                        width={Math.max(segmentWidth, 1.5)}
                        height={rowHeight - 16}
                        fill={STATE_COLORS[record.state] || '#000'}
                        rx="2"
                        opacity={hoveredSegment === record ? 1 : 0.85}
                        onMouseEnter={() => setHoveredSegment(record)}
                        onMouseLeave={() => setHoveredSegment(null)}
                        style={{ cursor: 'pointer', transition: 'opacity 0.2s' }}
                    />
                    );
                })}
                </g>
            );
            })}

            {/* X-Axis */}
            <g transform={`translate(0, ${height - padding - 20})`}>
                <line x1={labelWidth} y1={0} x2={labelWidth + chartWidth} y2={0} stroke="#cbd5e1" strokeWidth="1" />
                {[0, 0.25, 0.5, 0.75, 1].map(p => {
                    const x = labelWidth + p * chartWidth;
                    const tickVal = Math.round(minTick + p * tickRange);
                    return (
                        <g key={p}>
                        <line x1={x} y1={0} x2={x} y2={5} stroke="#cbd5e1" />
                        <text x={x} y={18} textAnchor="middle" style={{ fontSize: '11px', fill: '#64748b' }}>{tickVal}</text>
                        </g>
                    );
                })}
                <text x={labelWidth + chartWidth / 2} y={35} textAnchor="middle" style={{ fontSize: '12px', fill: '#94a3b8' }}>Simulation Ticks</text>
            </g>
        </svg>

        {/* Hover Info */}
        {hoveredSegment && (
            <div style={{
                position: 'fixed',
                left: '50%',
                bottom: '20px',
                transform: 'translateX(-50%)',
                background: 'rgba(15, 23, 42, 0.9)',
                color: 'white',
                padding: '12px 20px',
                borderRadius: '8px',
                boxShadow: '0 10px 15px -3px rgba(0, 0, 0, 0.1)',
                zIndex: 100,
                display: 'flex',
                gap: '20px',
                fontSize: '14px',
                backdropFilter: 'blur(4px)'
            }}>
                <div><strong>EVSE {hoveredSegment.evse_id}</strong></div>
                <div>State: <span style={{ color: STATE_COLORS[hoveredSegment.state] }}>{hoveredSegment.state}</span></div>
                <div>Start Tick: {hoveredSegment.tick}</div>
                {hoveredSegment.enforced_limit && (
                    <div>Limit: {hoveredSegment.enforced_limit.limit.toFixed(1)} {hoveredSegment.enforced_limit.unit} ({hoveredSegment.enforced_limit.reason})</div>
                )}
            </div>
        )}
      </div>
      
      <div style={{ marginTop: '20px', display: 'flex', gap: '15px', flexWrap: 'wrap', justifyContent: 'center' }}>
        {Object.entries(STATE_COLORS).map(([state, color]) => (
          <div key={state} style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '12px' }}>
            <div style={{ width: '12px', height: '12px', borderRadius: '2px', backgroundColor: color }}></div>
            <span>{state}</span>
          </div>
        ))}
      </div>
    </div>
  );
};
