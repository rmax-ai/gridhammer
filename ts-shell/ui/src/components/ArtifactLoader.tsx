import React, { useCallback } from 'react';
import type { EvseStateRecord } from '../types';

interface Props {
  onDataLoaded: (data: EvseStateRecord[]) => void;
}

export const ArtifactLoader: React.FC<Props> = ({ onDataLoaded }) => {
  const handleFileChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      if (!content) return;

      const lines = content.split('\n');
      const records: EvseStateRecord[] = [];

      for (const line of lines) {
        if (line.trim()) {
          try {
            records.push(JSON.parse(line));
          } catch (err) {
            console.error('Failed to parse line:', line, err);
          }
        }
      }

      onDataLoaded(records);
    };
    reader.readAsText(file);
  }, [onDataLoaded]);

  return (
    <div style={{ 
      border: '2px dashed #ccc', 
      padding: '20px', 
      textAlign: 'center', 
      margin: '20px',
      borderRadius: '8px'
    }}>
      <h3>Upload evse_states.jsonl</h3>
      <input 
        type="file" 
        accept=".jsonl" 
        onChange={handleFileChange} 
      />
      <p>Drag and drop or click to select the artifact file.</p>
    </div>
  );
};
