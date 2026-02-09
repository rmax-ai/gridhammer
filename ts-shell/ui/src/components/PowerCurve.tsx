import React, { useMemo } from 'react';
import type { PowerSeriesRecord } from '../types';

interface Props {
  data: PowerSeriesRecord[];
}

const COLORS = [
  '#2196f3', '#ff9800', '#4caf50', '#f44336', '#9c27b0', '#795548', '#009688', '#e91e63'
];

export const PowerCurve: React.FC<Props> = ({ data }) => {
  const groupedData = useMemo(() => {
    const groups: Record<number, PowerSeriesRecord[]> = {};
    data.forEach(r => {
      if (!groups[r.evse_id]) groups[r.evse_id] = [];
      groups[r.evse_id].push(r);
    });
    // Sort by tick
    Object.values(groups).forEach(g => g.sort((a, b) => a.tick - b.tick));
    return groups;
  }, [data]);

  const evseIds = useMemo(() => Object.keys(groupedData).map(Number).sort((a, b) => a - b), [groupedData]);

  const maxTick = useMemo(() => Math.max(...data.map(r => r.tick), 0), [data]);
  const minTick = useMemo(() => Math.min(...data.map(r => r.tick), 0), [data]);
  const maxPower = useMemo(() => Math.max(...data.map(r => r.power_w), 1000), [data]); // Default to 1000 if 0

  const width = 1000;
  const height = 400;
  const padding = 40;
  const chartWidth = width - padding * 2;
  const chartHeight = height - padding * 2;

  const tickRange = maxTick - minTick || 1;
  const powerRange = maxPower || 1;

  const getX = (tick: number) => padding + ((tick - minTick) / tickRange) * chartWidth;
  const getY = (power: number) => height - padding - (power / powerRange) * chartHeight;

  return (
    <div style={{ padding: '20px', overflowX: 'auto' }}>
      <h2>Power Curves</h2>
      <svg width={width} height={height} style={{ border: '1px solid #ddd' }}>
        {/* Y Axis Grid */}
        {[0, 0.25, 0.5, 0.75, 1].map(p => {
          const y = getY(p * powerRange);
          return (
             <g key={p}>
               <line x1={padding} y1={y} x2={width - padding} y2={y} stroke="#eee" />
               <text x={padding - 5} y={y} textAnchor="end" dominantBaseline="middle" style={{ fontSize: '10px' }}>
                 {Math.round(p * powerRange / 1000)}kW
               </text>
             </g>
          );
        })}

        {/* X Axis Grid */}
        {[0, 0.25, 0.5, 0.75, 1].map(p => {
            const x = getX(minTick + p * tickRange);
            return (
                <g key={p}>
                    <line x1={x} y1={padding} x2={x} y2={height - padding} stroke="#eee" />
                    <text x={x} y={height - padding + 15} textAnchor="middle" style={{ fontSize: '10px' }}>
                        {Math.round(minTick + p * tickRange)}
                    </text>
                </g>
            );
        })}

        {/* Axes */}
        <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="black" />
        <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="black" />

        {/* Lines */}
        {evseIds.map((evseId) => {
          const points = groupedData[evseId].map(r => `${getX(r.tick)},${getY(r.power_w)}`).join(' ');
          const color = COLORS[evseId % COLORS.length];
          return (
            <polyline
              key={evseId}
              points={points}
              fill="none"
              stroke={color}
              strokeWidth="2"
              opacity="0.8"
            />
          );
        })}
      </svg>

      <div style={{ marginTop: '20px', display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
        {evseIds.map((evseId) => (
          <div key={evseId} style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
            <div style={{ width: '12px', height: '12px', backgroundColor: COLORS[evseId % COLORS.length] }}></div>
            <span style={{ fontSize: '12px' }}>EVSE {evseId}</span>
          </div>
        ))}
      </div>
    </div>
  );
};
