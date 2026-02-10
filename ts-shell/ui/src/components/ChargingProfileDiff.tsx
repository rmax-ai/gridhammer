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
        <pre style={{ fontSize: '0.8em', background: '#f5f5f5', padding: '5px', borderRadius: '4px', overflowX: 'auto' }}>
            {JSON.stringify(p.profile, null, 2)}
        </pre>
    );

    const hasDiff = diff.added.length > 0 || diff.removed.length > 0 || diff.modified.length > 0;

    return (
        <div style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px', width: '1000px' }}>
            <h2>Charging Profile Diff</h2>
            
            {!hasDiff && <p>No differences in charging profiles found.</p>}

            <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
                {diff.removed.length > 0 && (
                    <section>
                        <h3 style={{ color: '#d32f2f' }}>Removed in {compLabel}</h3>
                        {diff.removed.map(p => (
                            <div key={`${p.tick}-${p.id}`} style={{ marginBottom: '10px', padding: '10px', border: '1px solid #ffcdd2', backgroundColor: '#ffebee', borderRadius: '4px' }}>
                                <strong>Tick {p.tick} (ID {p.id})</strong>
                                {renderProfile(p)}
                            </div>
                        ))}
                    </section>
                )}

                {diff.added.length > 0 && (
                    <section>
                        <h3 style={{ color: '#388e3c' }}>Added in {compLabel}</h3>
                        {diff.added.map(p => (
                            <div key={`${p.tick}-${p.id}`} style={{ marginBottom: '10px', padding: '10px', border: '1px solid #c8e6c9', backgroundColor: '#e8f5e9', borderRadius: '4px' }}>
                                <strong>Tick {p.tick} (ID {p.id})</strong>
                                {renderProfile(p)}
                            </div>
                        ))}
                    </section>
                )}

                {diff.modified.length > 0 && (
                    <section>
                        <h3 style={{ color: '#f57c00' }}>Modified</h3>
                        {diff.modified.map(({ base, comp }) => (
                            <div key={`${base.tick}-${base.id}`} style={{ marginBottom: '10px', padding: '10px', border: '1px solid #ffe0b2', backgroundColor: '#fff3e0', borderRadius: '4px' }}>
                                <strong>Tick {base.tick} (ID {base.id})</strong>
                                <div style={{ display: 'flex', gap: '10px' }}>
                                    <div style={{ flex: 1 }}>
                                        <h4 style={{ margin: '5px 0' }}>{baseLabel}</h4>
                                        {renderProfile(base)}
                                    </div>
                                    <div style={{ flex: 1 }}>
                                        <h4 style={{ margin: '5px 0' }}>{compLabel}</h4>
                                        {renderProfile(comp)}
                                    </div>
                                </div>
                            </div>
                        ))}
                    </section>
                )}

                <section>
                    <label style={{ fontSize: '0.9em', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '5px' }}>
                        <input 
                            type="checkbox" 
                            checked={showUnchanged} 
                            onChange={e => setShowUnchanged(e.target.checked)} 
                        />
                        Show Unchanged ({diff.unchanged.length})
                    </label>
                    {showUnchanged && diff.unchanged.length > 0 && (
                        <div style={{ marginTop: '10px', display: 'flex', flexWrap: 'wrap', gap: '5px' }}>
                            {diff.unchanged.map(p => (
                                <div key={`${p.tick}-${p.id}`} style={{ padding: '2px 5px', fontSize: '0.8em', color: '#666', border: '1px solid #eee' }}>
                                    T{p.tick}:ID{p.id}
                                </div>
                            ))}
                        </div>
                    )}
                </section>
            </div>
        </div>
    );
};
