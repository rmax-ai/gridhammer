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

  const limits = useMemo(() => {
      return evseData.filter(r => r.enforced_limit);
  }, [evseData]);

  return (
    <div className="profile-viewer">
      <h3 style={{ marginBottom: '20px' }}>Profile & Enforcement History</h3>
      
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '30px' }}>
        <div>
            <h4 style={{ fontSize: '14px', color: '#64748b', marginBottom: '15px', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                Received Profiles
            </h4>
            {profiles.length === 0 ? <p style={{ color: '#94a3b8', fontSize: '14px' }}>No SetChargingProfile messages found.</p> : (
                <div style={{ maxHeight: '500px', overflowY: 'auto', paddingRight: '10px' }}>
                    {profiles.map((p, i) => (
                        <div key={i} style={{ marginBottom: '15px', padding: '12px', background: '#f8fafc', border: '1px solid #e2e8f0', borderRadius: '6px' }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                                <span style={{ fontWeight: 700, fontSize: '13px' }}>Tick {p.tick}</span>
                                <span style={{ fontSize: '11px', background: '#dbeafe', color: '#1e40af', padding: '2px 6px', borderRadius: '10px' }}>ID {p.profile.id}</span>
                            </div>
                            <div style={{ fontSize: '12px', color: '#475569', marginBottom: '8px' }}>
                                <strong>Purpose:</strong> {p.profile.chargingProfilePurpose} | <strong>Stack:</strong> {p.profile.stackLevel}
                            </div>
                            <details>
                                <summary style={{ fontSize: '11px', cursor: 'pointer', color: '#2563eb', fontWeight: 500 }}>View Payload</summary>
                                <pre style={{ fontSize: '10px', marginTop: '8px', padding: '8px', background: '#0f172a', color: '#cbd5e1', borderRadius: '4px' }}>
                                    {JSON.stringify(p.profile, null, 2)}
                                </pre>
                            </details>
                        </div>
                    ))}
                </div>
            )}
        </div>

        <div>
            <h4 style={{ fontSize: '14px', color: '#64748b', marginBottom: '15px', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                Enforcement Log
            </h4>
             {limits.length === 0 ? <p style={{ color: '#94a3b8', fontSize: '14px' }}>No enforcement data found.</p> : (
                <div style={{ maxHeight: '500px', overflowY: 'auto', paddingRight: '10px' }}>
                    {limits.map((l, i) => (
                        <div key={i} style={{ marginBottom: '8px', padding: '10px', background: '#f8fafc', borderBottom: '2px solid #e2e8f0', display: 'flex', alignItems: 'center', gap: '15px' }}>
                            <div style={{ width: '60px', fontWeight: 700, fontSize: '12px', color: '#64748b' }}>T {l.tick}</div>
                            <div style={{ flex: 1 }}>
                                <div style={{ fontSize: '13px', fontWeight: 600 }}>
                                    EVSE {l.evse_id}: <span style={{ color: '#2563eb' }}>{l.enforced_limit?.limit.toFixed(1)} {l.enforced_limit?.unit}</span>
                                </div>
                                <div style={{ fontSize: '11px', color: '#94a3b8' }}>
                                    {l.enforced_limit?.reason} {l.enforced_limit?.should_suspend ? '• SUSPEND' : ''}
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
             )}
        </div>
      </div>
    </div>
  );
};
