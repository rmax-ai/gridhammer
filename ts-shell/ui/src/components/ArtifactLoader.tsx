import React, { useCallback } from 'react';
import type { EvseStateRecord, OcppTraceRecord, PowerSeriesRecord } from '../types';

interface Props {
  onEvseDataLoaded: (data: EvseStateRecord[]) => void;
  onTraceDataLoaded: (data: OcppTraceRecord[]) => void;
  onPowerDataLoaded: (data: PowerSeriesRecord[]) => void;
}

export const ArtifactLoader: React.FC<Props> = ({ onEvseDataLoaded, onTraceDataLoaded, onPowerDataLoaded }) => {
  const handleFileChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      if (!content) return;

      const lines = content.split('\n').filter(l => l.trim().length > 0);
      
      const isTrace = file.name.includes('ocpp_trace');
      const isEvse = file.name.includes('evse_states');
      const isPower = file.name.includes('power_series');

      if (isPower || file.name.endsWith('.csv')) {
        // CSV Parsing
        const header = lines[0].split(',').map(h => h.trim());
        const powerRecords: PowerSeriesRecord[] = [];
        
        for (let i = 1; i < lines.length; i++) {
            const values = lines[i].split(',');
            const record: Record<string, string | number | undefined> = {};
            header.forEach((h, idx) => {
                const val = values[idx]?.trim();
                if (h === 'tick' || h === 'evse_id' || h === 'power_w' || h === 'limit_w') {
                    record[h] = val ? Number(val) : undefined;
                } else {
                    record[h] = val;
                }
            });
            
            // Basic validation
            if (record.tick !== undefined && record.evse_id !== undefined && record.timestamp !== undefined && record.power_w !== undefined) {
                powerRecords.push(record as unknown as PowerSeriesRecord);
            }
        }
        
        if (powerRecords.length > 0) {
            onPowerDataLoaded(powerRecords);
        }
        return;
      }

      // JSONL Parsing
      const evseRecords: EvseStateRecord[] = [];
      const traceRecords: OcppTraceRecord[] = [];

      for (const line of lines) {
        if (line.trim()) {
          try {
            const json = JSON.parse(line);
            if (isEvse) {
                evseRecords.push(json as EvseStateRecord);
            } else if (isTrace) {
                traceRecords.push(json as OcppTraceRecord);
            } else {
                // Fallback sniffing based on fields
                if ('state' in json && 'evse_id' in json) {
                    evseRecords.push(json as EvseStateRecord);
                } else if ('action' in json && 'direction' in json) {
                    traceRecords.push(json as OcppTraceRecord);
                }
            }
          } catch (err) {
            console.error('Failed to parse line:', line, err);
          }
        }
      }

      if (evseRecords.length > 0) {
        onEvseDataLoaded(evseRecords);
      }
      if (traceRecords.length > 0) {
        onTraceDataLoaded(traceRecords);
      }
    };
    reader.readAsText(file);
  }, [onEvseDataLoaded, onTraceDataLoaded, onPowerDataLoaded]);

  return (
    <div style={{ 
      border: '2px dashed #ccc', 
      padding: '20px', 
      textAlign: 'center', 
      margin: '20px',
      borderRadius: '8px'
    }}>
      <h3>Upload Artifacts</h3>
      <p>Upload <code>evse_states.jsonl</code>, <code>ocpp_trace.jsonl</code>, or <code>power_series.csv</code></p>
      <input 
        type="file" 
        accept=".jsonl,.csv" 
        onChange={handleFileChange} 
      />
    </div>
  );
};
