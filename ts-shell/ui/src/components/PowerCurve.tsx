import React, { useMemo, useState } from 'react';
import type { PowerSeriesRecord } from '../types';

interface Props {
  data: PowerSeriesRecord[];
}

const COLORS = [
  '#3b82f6', '#f59e0b', '#10b981', '#ef4444', '#8b5cf6', '#b45309', '#06b6d4', '#ec4899'
];

export const PowerCurve: React.FC<Props> = ({ data }) => {
  const [hoveredPoint, setHoveredPoint] = useState<{ tick: number, values: Record<number, number> } | null>(null);

  const groupedData = useMemo(() => {
    const groups: Record<number, PowerSeriesRecord[]> = {};
    data.forEach(r => {
      if (!groups[r.evse_id]) groups[r.evse_id] = [];
      groups[r.evse_id].push(r);
    });
    Object.values(groups).forEach(g => g.sort((a, b) => a.tick - b.tick));
    return groups;
  }, [data]);

  const evseIds = useMemo(() => Object.keys(groupedData).map(Number).sort((a, b) => a - b), [groupedData]);

  const allTicks = useMemo(() => {
      const ticks = Array.from(new Set(data.map(r => r.tick))).sort((a, b) => a - b);
      return ticks;
  }, [data]);

  const maxTick = allTicks[allTicks.length - 1] || 0;
  const minTick = allTicks[0] || 0;
  const maxPower = useMemo(() => Math.max(...data.map(r => r.power_w), 1000), [data]);

  const width = 1000;
  const height = 400;
  const padding = 50;
  const chartWidth = width - padding * 2;
  const chartHeight = height - padding * 2;

  const tickRange = maxTick - minTick || 1;
  const powerRange = maxPower * 1.1; // Add 10% headroom

  const getX = (tick: number) => padding + ((tick - minTick) / tickRange) * chartWidth;
  const getY = (power: number) => height - padding - (power / powerRange) * chartHeight;

  const handleMouseMove = (e: React.MouseEvent<SVGSVGElement>) => {
      const svg = e.currentTarget;
      const rect = svg.getBoundingClientRect();
      const x = e.clientX - rect.left;
      
      if (x < padding || x > width - padding) {
          setHoveredPoint(null);
          return;
      }

      const ratio = (x - padding) / chartWidth;
      const targetTick = minTick + ratio * tickRange;
      
      // Find closest tick
      let closestTick = allTicks[0];
      let minDiff = Math.abs(targetTick - closestTick);
      
      for (const t of allTicks) {
          const diff = Math.abs(targetTick - t);
          if (diff < minDiff) {
              minDiff = diff;
              closestTick = t;
          }
      }

      const values: Record<number, number> = {};
      evseIds.forEach(id => {
          const record = groupedData[id].find(r => r.tick === closestTick);
          if (record) values[id] = record.power_w;
      });

      setHoveredPoint({ tick: closestTick, values });
  };

  return (
    <div className="power-curve-container">
      <h3>Power Curves (kW)</h3>
      <div style={{ position: 'relative' }}>
        <svg 
            width="100%" 
            viewBox={`0 0 ${width} ${height}`} 
            onMouseMove={handleMouseMove}
            onMouseLeave={() => setHoveredPoint(null)}
            style={{ overflow: 'visible' }}
        >
            {/* Grid Lines */}
            {[0, 0.25, 0.5, 0.75, 1].map(p => {
                const y = getY(p * maxPower);
                return (
                    <g key={`y-${p}`}>
                        <line x1={padding} y1={y} x2={width - padding} y2={y} stroke="var(--border-color, #e2e8f0)" strokeDasharray="4 4" />
                        <text x={padding - 10} y={y} textAnchor="end" dominantBaseline="middle" style={{ fontSize: '12px', fill: '#64748b' }}>
                            {Math.round(p * maxPower / 1000)}k
                        </text>
                    </g>
                );
            })}

            {/* Lines */}
            {evseIds.map((evseId) => {
                const points = groupedData[evseId].map(r => `${getX(r.tick)},${getY(r.power_w)}`).join(' ');
                return (
                    <polyline
                        key={evseId}
                        points={points}
                        fill="none"
                        stroke={COLORS[evseId % COLORS.length]}
                        strokeWidth="2.5"
                        strokeLinejoin="round"
                        strokeLinecap="round"
                        opacity="0.8"
                    />
                );
            })}

            {/* Hover Line */}
            {hoveredPoint && (
                <g>
                    <line 
                        x1={getX(hoveredPoint.tick)} 
                        y1={padding} 
                        x2={getX(hoveredPoint.tick)} 
                        y2={height - padding} 
                        stroke="#64748b" 
                        strokeWidth="1"
                    />
                    {evseIds.map(id => {
                        const val = hoveredPoint.values[id];
                        if (val === undefined) return null;
                        return (
                            <circle 
                                key={id}
                                cx={getX(hoveredPoint.tick)}
                                cy={getY(val)}
                                r="4"
                                fill={COLORS[id % COLORS.length]}
                                stroke="white"
                                strokeWidth="2"
                            />
                        );
                    })}
                </g>
            )}

            {/* Axes */}
            <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="#475569" strokeWidth="2" />
            <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="#475569" strokeWidth="2" />
        </svg>

        {/* Tooltip */}
        {hoveredPoint && (
            <div style={{
                position: 'absolute',
                left: `${getX(hoveredPoint.tick) + 20}px`,
                top: `${padding}px`,
                background: 'white',
                border: '1px solid #e2e8f0',
                padding: '8px',
                borderRadius: '4px',
                boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
                zIndex: 10,
                pointerEvents: 'none',
                minWidth: '120px'
            }}>
                <div style={{ fontWeight: 'bold', borderBottom: '1px solid #e2e8f0', marginBottom: '4px', fontSize: '12px' }}>
                    Tick: {hoveredPoint.tick}
                </div>
                {evseIds.map(id => (
                    <div key={id} style={{ display: 'flex', justifyContent: 'space-between', fontSize: '11px', gap: '10px' }}>
                        <span style={{ color: COLORS[id % COLORS.length] }}>EVSE {id}:</span>
                        <span>{(hoveredPoint.values[id] || 0).toFixed(1)}W</span>
                    </div>
                ))}
            </div>
        )}
      </div>

      <div style={{ marginTop: '20px', display: 'flex', gap: '15px', flexWrap: 'wrap', justifyContent: 'center' }}>
        {evseIds.map((evseId) => (
          <div key={evseId} style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '13px' }}>
            <div style={{ width: '12px', height: '12px', borderRadius: '2px', backgroundColor: COLORS[evseId % COLORS.length] }}></div>
            <span>EVSE {evseId}</span>
          </div>
        ))}
      </div>
    </div>
  );
};
