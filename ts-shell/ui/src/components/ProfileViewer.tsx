import React, { useMemo } from 'react';
import type { EvseStateRecord, OcppTraceRecord } from '../types';

interface Props {
  traceData: OcppTraceRecord[];
  evseData: EvseStateRecord[];
}

interface SetChargingProfilePayload {
  chargingProfile: {
    id: number;
    stackLevel: number;
    chargingProfilePurpose: string;
    [key: string]: unknown;
  };
}

export const ProfileViewer: React.FC<Props> = ({ traceData, evseData }) => {
  const profiles = useMemo(() => {
    return traceData
      .filter(r => r.action === 'SetChargingProfile' && r.direction === 'inbound')
      .map(r => {
        const payload = r.payload as SetChargingProfilePayload;
        return {
            tick: r.tick,
            timestamp: r.timestamp,
            profile: payload.chargingProfile
        };
      });
  }, [traceData]);

  // Derive enforced limits
  const limits = useMemo(() => {
      return evseData.filter(r => r.enforced_limit);
  }, [evseData]);

  return (
    <div style={{ padding: '20px' }}>
      <h2>Profile Application History</h2>
      
      <div style={{ display: 'flex', gap: '20px' }}>
        <div style={{ flex: 1 }}>
            <h3>Received Profiles (Inbound)</h3>
            {profiles.length === 0 ? <p>No SetChargingProfile messages found.</p> : (
                <ul style={{ maxHeight: '400px', overflowY: 'auto', listStyle: 'none', padding: 0 }}>
                    {profiles.map((p, i) => (
                        <li key={i} style={{ marginBottom: '10px', padding: '10px', border: '1px solid #eee', borderRadius: '4px' }}>
                            <strong>Tick {p.tick}</strong>: ID {p.profile.id} (Stack {p.profile.stackLevel})
                            <br/>
                            Purpose: {p.profile.chargingProfilePurpose}
                            <br/>
                            <details>
                                <summary>Raw Payload</summary>
                                <pre style={{ fontSize: '0.8em', overflowX: 'auto' }}>{JSON.stringify(p.profile, null, 2)}</pre>
                            </details>
                        </li>
                    ))}
                </ul>
            )}
        </div>

        <div style={{ flex: 1 }}>
            <h3>Enforced Limits (EVSE Output)</h3>
             {limits.length === 0 ? <p>No enforcement data found.</p> : (
                <ul style={{ maxHeight: '400px', overflowY: 'auto', listStyle: 'none', padding: 0 }}>
                    {limits.map((l, i) => (
                        <li key={i} style={{ marginBottom: '5px', padding: '5px', borderBottom: '1px solid #f0f0f0' }}>
                            <strong>Tick {l.tick}</strong> (EVSE {l.evse_id}): 
                            {' '}{l.enforced_limit?.limit.toFixed(1)} {l.enforced_limit?.unit}
                            <br/>
                            <span style={{ fontSize: '0.9em', color: '#666' }}>
                                Reason: {l.enforced_limit?.reason} 
                                {l.enforced_limit?.should_suspend ? ' (SUSPEND)' : ''}
                            </span>
                        </li>
                    ))}
                </ul>
             )}
        </div>
      </div>
    </div>
  );
};
