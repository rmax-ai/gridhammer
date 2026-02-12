import React, { useMemo, useState } from 'react';
import type { OcppTraceRecord } from '../types';

interface SetChargingProfilePayload {
  chargingProfile: {
    id: number;
    stackLevel: number;
    chargingProfilePurpose: string;
    [key: string]: unknown;
  };
}

interface Props {
  baseTrace: OcppTraceRecord[];
  compTrace: OcppTraceRecord[];
  baseLabel: string;
  compLabel: string;
}

interface ProfileRecord {
    tick: number;
    id: number;
    profile: SetChargingProfilePayload['chargingProfile'];
}

export const ChargingProfileDiff: React.FC<Props> = ({ baseTrace, compTrace, baseLabel, compLabel }) => {
    const [showUnchanged, setShowUnchanged] = useState(false);

    const extractProfiles = (trace: OcppTraceRecord[]): ProfileRecord[] => {
        return trace
            .filter(r => r.action === 'SetChargingProfile' && r.direction === 'inbound')
            .map(r => {
                const payload = r.payload as SetChargingProfilePayload;
                return {
                    tick: r.tick,
                    id: payload.chargingProfile.id,
                    profile: payload.chargingProfile
                };
            });
    };

    const baseProfiles = useMemo(() => extractProfiles(baseTrace), [baseTrace]);
    const compProfiles = useMemo(() => extractProfiles(compTrace), [compTrace]);

    const diff = useMemo(() => {
        const added: ProfileRecord[] = [];
        const removed: ProfileRecord[] = [];
        const modified: { base: ProfileRecord, comp: ProfileRecord }[] = [];
        const unchanged: ProfileRecord[] = [];

        const baseMap = new Map<string, ProfileRecord>();
        baseProfiles.forEach(p => baseMap.set(`${p.tick}-${p.id}`, p));

        const compMap = new Map<string, ProfileRecord>();
        compProfiles.forEach(p => compMap.set(`${p.tick}-${p.id}`, p));

        baseMap.forEach((baseP, key) => {
            const compP = compMap.get(key);
            if (!compP) {
                removed.push(baseP);
            } else {
                if (JSON.stringify(baseP.profile) !== JSON.stringify(compP.profile)) {
                    modified.push({ base: baseP, comp: compP });
                } else {
                    unchanged.push(baseP);
                }
            }
        });

        compMap.forEach((compP, key) => {
            if (!baseMap.has(key)) {
                added.push(compP);
            }
        });

        return { 
            added: added.sort((a, b) => a.tick - b.tick), 
            removed: removed.sort((a, b) => a.tick - b.tick), 
            modified: modified.sort((a, b) => a.base.tick - b.base.tick), 
            unchanged: unchanged.sort((a, b) => a.tick - b.tick) 
        };
    }, [baseProfiles, compProfiles]);

    const renderProfile = (p: ProfileRecord) => (
        <pre style={{ fontSize: '0.75em', background: '#f8fafc', padding: '10px', borderRadius: '4px', overflowX: 'auto', border: '1px solid #e2e8f0', margin: '5px 0' }}>
            {JSON.stringify(p.profile, null, 2)}
        </pre>
    );

    const hasDiff = diff.added.length > 0 || diff.removed.length > 0 || diff.modified.length > 0;

    return (
        <div className="profile-diff-container">
            <h3 style={{ marginBottom: '20px' }}>Charging Profile Diff</h3>
            
            {!hasDiff && <p style={{ color: '#64748b' }}>No differences in charging profiles found.</p>}

            <div style={{ display: 'flex', flexDirection: 'column', gap: '1.5rem' }}>
                {diff.removed.length > 0 && (
                    <div className="diff-group">
                        <h4 style={{ color: '#ef4444', fontSize: '14px', marginBottom: '10px' }}>Removed in {compLabel}</h4>
                        {diff.removed.map(p => (
                            <div key={`${p.tick}-${p.id}`} style={{ marginBottom: '10px', padding: '12px', borderLeft: '4px solid #ef4444', backgroundColor: '#fef2f2', borderRadius: '4px' }}>
                                <div style={{ fontSize: '13px', fontWeight: 600 }}>Tick {p.tick} (ID {p.id})</div>
                                {renderProfile(p)}
                            </div>
                        ))}
                    </div>
                )}

                {diff.added.length > 0 && (
                    <div className="diff-group">
                        <h4 style={{ color: '#10b981', fontSize: '14px', marginBottom: '10px' }}>Added in {compLabel}</h4>
                        {diff.added.map(p => (
                            <div key={`${p.tick}-${p.id}`} style={{ marginBottom: '10px', padding: '12px', borderLeft: '4px solid #10b981', backgroundColor: '#ecfdf5', borderRadius: '4px' }}>
                                <div style={{ fontSize: '13px', fontWeight: 600 }}>Tick {p.tick} (ID {p.id})</div>
                                {renderProfile(p)}
                            </div>
                        ))}
                    </div>
                )}

                {diff.modified.length > 0 && (
                    <div className="diff-group">
                        <h4 style={{ color: '#f59e0b', fontSize: '14px', marginBottom: '10px' }}>Modified Profiles</h4>
                        {diff.modified.map(({ base, comp }) => (
                            <div key={`${base.tick}-${base.id}`} style={{ marginBottom: '15px', padding: '12px', borderLeft: '4px solid #f59e0b', backgroundColor: '#fffbeb', borderRadius: '4px' }}>
                                <div style={{ fontSize: '13px', fontWeight: 600, marginBottom: '10px' }}>Tick {base.tick} (ID {base.id})</div>
                                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '15px' }}>
                                    <div>
                                        <div style={{ fontSize: '11px', fontWeight: 600, color: '#92400e', marginBottom: '4px' }}>{baseLabel}</div>
                                        {renderProfile(base)}
                                    </div>
                                    <div>
                                        <div style={{ fontSize: '11px', fontWeight: 600, color: '#92400e', marginBottom: '4px' }}>{compLabel}</div>
                                        {renderProfile(comp)}
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                )}

                <div style={{ borderTop: '1px solid #e2e8f0', paddingTop: '1rem' }}>
                    <label style={{ fontSize: '13px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '8px', color: '#64748b' }}>
                        <input 
                            type="checkbox" 
                            checked={showUnchanged} 
                            onChange={e => setShowUnchanged(e.target.checked)} 
                        />
                        Show Unchanged Profiles ({diff.unchanged.length})
                    </label>
                    {showUnchanged && diff.unchanged.length > 0 && (
                        <div style={{ marginTop: '10px', display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                            {diff.unchanged.map(p => (
                                <div key={`${p.tick}-${p.id}`} style={{ padding: '4px 8px', fontSize: '11px', color: '#475569', border: '1px solid #e2e8f0', borderRadius: '4px', background: '#f8fafc' }}>
                                    T{p.tick}: ID{p.id}
                                </div>
                            ))}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};
