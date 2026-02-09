import React, { useCallback } from 'react';
import type { EvseStateRecord, OcppTraceRecord } from '../types';

interface Props {
  onEvseDataLoaded: (data: EvseStateRecord[]) => void;
  onTraceDataLoaded: (data: OcppTraceRecord[]) => void;
}

export const ArtifactLoader: React.FC<Props> = ({ onEvseDataLoaded, onTraceDataLoaded }) => {
  const handleFileChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      if (!content) return;

      const lines = content.split('\n');
      const evseRecords: EvseStateRecord[] = [];
      const traceRecords: OcppTraceRecord[] = [];

      // Simple sniffing based on filename
      const isTrace = file.name.includes('ocpp_trace');
      const isEvse = file.name.includes('evse_states');

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
  }, [onEvseDataLoaded, onTraceDataLoaded]);

  return (
    <div style={{ 
      border: '2px dashed #ccc', 
      padding: '20px', 
      textAlign: 'center', 
      margin: '20px',
      borderRadius: '8px'
    }}>
      <h3>Upload Artifacts</h3>
      <p>Upload <code>evse_states.jsonl</code> or <code>ocpp_trace.jsonl</code></p>
      <input 
        type="file" 
        accept=".jsonl" 
        onChange={handleFileChange} 
      />
    </div>
  );
};
