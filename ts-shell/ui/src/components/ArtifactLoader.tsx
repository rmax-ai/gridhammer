import React, { useCallback, useState } from 'react';
import type { EvseStateRecord, OcppTraceRecord, PowerSeriesRecord } from '../types';

interface Props {
  onEvseDataLoaded: (data: EvseStateRecord[]) => void;
  onTraceDataLoaded: (data: OcppTraceRecord[]) => void;
  onPowerDataLoaded: (data: PowerSeriesRecord[]) => void;
}

export const ArtifactLoader: React.FC<Props> = ({ onEvseDataLoaded, onTraceDataLoaded, onPowerDataLoaded }) => {
  const [isDragging, setIsDragging] = useState(false);
  const [loading, setLoading] = useState(false);

  const processFile = useCallback((file: File) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      if (!content) return;

      const lines = content.split('\n').filter(l => l.trim().length > 0);
      
      const isTrace = file.name.includes('ocpp_trace');
      const isEvse = file.name.includes('evse_states');
      const isPower = file.name.includes('power_series');

      if (isPower || file.name.endsWith('.csv')) {
        const header = lines[0].split(',').map(h => h.trim());
        const powerRecords: PowerSeriesRecord[] = [];
        
        for (let i = 1; i < lines.length; i++) {
            const values = lines[i].split(',');
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const record: any = {};
            header.forEach((h, idx) => {
                const val = values[idx]?.trim();
                if (['tick', 'evse_id', 'power_w', 'limit_w'].includes(h)) {
                    record[h] = val ? Number(val) : undefined;
                } else {
                    record[h] = val;
                }
            });
            if (record.tick !== undefined && record.evse_id !== undefined) {
                powerRecords.push(record as unknown as PowerSeriesRecord);
            }
        }
        if (powerRecords.length > 0) onPowerDataLoaded(powerRecords);
        return;
      }

      const evseRecords: EvseStateRecord[] = [];
      const traceRecords: OcppTraceRecord[] = [];

      for (const line of lines) {
        try {
            const json = JSON.parse(line);
            if (isEvse || ('state' in json && 'evse_id' in json)) {
                evseRecords.push(json as EvseStateRecord);
            } else if (isTrace || ('action' in json && 'direction' in json)) {
                traceRecords.push(json as OcppTraceRecord);
            }
        } catch (error) {
            console.warn('Skipping invalid JSON line', error);
        }
      }

      if (evseRecords.length > 0) onEvseDataLoaded(evseRecords);
      if (traceRecords.length > 0) onTraceDataLoaded(traceRecords);
    };
    reader.readAsText(file);
  }, [onEvseDataLoaded, onTraceDataLoaded, onPowerDataLoaded]);

  const handleFiles = useCallback((files: FileList | null) => {
    if (!files) return;
    setLoading(true);
    Array.from(files).forEach(processFile);
    setLoading(false);
  }, [processFile]);

  const onDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  }, []);

  const onDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  }, []);

  const onDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    handleFiles(e.dataTransfer.files);
  }, [handleFiles]);

  return (
    <div 
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
      style={{ 
        border: `2px dashed ${isDragging ? '#2563eb' : '#cbd5e1'}`, 
        padding: '40px 20px', 
        textAlign: 'center', 
        borderRadius: '12px',
        backgroundColor: isDragging ? '#eff6ff' : '#f8fafc',
        transition: 'all 0.2s',
        cursor: 'pointer'
      }}
      onClick={() => document.getElementById('fileInput')?.click()}
    >
      <input 
        id="fileInput"
        type="file" 
        multiple
        accept=".jsonl,.csv" 
        style={{ display: 'none' }}
        onChange={(e) => handleFiles(e.target.files)} 
      />
      <div style={{ fontSize: '40px', marginBottom: '10px' }}>📦</div>
      <h3 style={{ margin: '0 0 10px 0' }}>{loading ? 'Processing...' : 'Drop artifacts here'}</h3>
      <p style={{ margin: 0, color: '#64748b', fontSize: '14px' }}>
        Select or drop <code>evse_states.jsonl</code>, <code>ocpp_trace.jsonl</code>, or <code>power_series.csv</code>
      </p>
      <button className="primary" style={{ marginTop: '20px' }}>Select Files</button>
    </div>
  );
};
