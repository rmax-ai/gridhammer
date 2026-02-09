import React, { useMemo } from 'react';
import type { EvseStateRecord, EvseState } from '../types';

interface Props {
  records: EvseStateRecord[];
}

const STATE_COLORS: Record<EvseState, string> = {
  "Available": '#4caf50', // Green
  "Preparing": '#2196f3', // Blue
  "Charging": '#ff9800',  // Orange
  "SuspendedEV": '#ffeb3b', // Yellow
  "SuspendedEVSE": '#f44336', // Redish
  "Faulted": '#b71c1c',   // Dark Red
  "Unavailable": '#9e9e9e', // Grey
  "Reserved": '#9c27b0',  // Purple
};

export const EvseTimeline: React.FC<Props> = ({ records }) => {
  const groupedRecords = useMemo(() => {
    const groups: Record<number, EvseStateRecord[]> = {};
    records.forEach(r => {
      if (!groups[r.evse_id]) groups[r.evse_id] = [];
      groups[r.evse_id].push(r);
    });
    // Sort each group by tick
    Object.values(groups).forEach(g => g.sort((a, b) => a.tick - b.tick));
    return groups;
  }, [records]);

  const evseIds = useMemo(() => Object.keys(groupedRecords).map(Number).sort((a, b) => a - b), [groupedRecords]);

  const maxTick = useMemo(() => Math.max(...records.map(r => r.tick), 0), [records]);
  const minTick = useMemo(() => Math.min(...records.map(r => r.tick), 0), [records]);
  const tickRange = maxTick - minTick || 1;

  const width = 1000;
  const rowHeight = 40;
  const padding = 20;
  const labelWidth = 100;
  const chartWidth = width - labelWidth - padding * 2;
  const height = evseIds.length * rowHeight + padding * 2;

  return (
    <div style={{ padding: '20px', overflowX: 'auto' }}>
      <h2>EVSE Timelines</h2>
      <svg width={width} height={height} style={{ border: '1px solid #ddd' }}>
        {evseIds.map((evseId, rowIndex) => {
          const evseRecords = groupedRecords[evseId];
          const y = padding + rowIndex * rowHeight;

          return (
            <g key={evseId}>
              {/* Label */}
              <text x={padding} y={y + rowHeight / 2} dominantBaseline="middle" style={{ fontSize: '12px' }}>
                EVSE {evseId}
              </text>

              {/* Row Background */}
              <rect 
                x={labelWidth} 
                y={y} 
                width={chartWidth} 
                height={rowHeight - 5} 
                fill="#f9f9f9" 
                stroke="#eee"
              />

              {/* State Segments */}
              {evseRecords.map((record, i) => {
                const nextTick = evseRecords[i + 1]?.tick ?? maxTick;
                const startX = labelWidth + ((record.tick - minTick) / tickRange) * chartWidth;
                const segmentWidth = ((nextTick - record.tick) / tickRange) * chartWidth;

                return (
                  <rect
                    key={i}
                    x={startX}
                    y={y + 5}
                    width={Math.max(segmentWidth, 1)}
                    height={rowHeight - 15}
                    fill={STATE_COLORS[record.state] || '#000'}
                  >
                    <title>
                      {`State: ${record.state}\nTick: ${record.tick}\nLimit: ${record.enforced_limit?.limit ?? 'N/A'} ${record.enforced_limit?.unit ?? ''}\nReason: ${record.enforced_limit?.reason ?? 'N/A'}`}
                    </title>
                  </rect>
                );
              })}
            </g>
          );
        })}

        {/* X-Axis ticks (simplified) */}
        <line x1={labelWidth} y1={height - padding} x2={labelWidth + chartWidth} y2={height - padding} stroke="#000" />
        {[0, 0.25, 0.5, 0.75, 1].map(p => {
          const x = labelWidth + p * chartWidth;
          const tickVal = Math.round(minTick + p * tickRange);
          return (
            <g key={p}>
              <line x1={x} y1={height - padding} x2={x} y2={height - padding + 5} stroke="#000" />
              <text x={x} y={height - padding + 15} textAnchor="middle" style={{ fontSize: '10px' }}>{tickVal}</text>
            </g>
          );
        })}
      </svg>
      
      <div style={{ marginTop: '20px', display: 'flex', gap: '10px', flexWrap: 'wrap' }}>
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
