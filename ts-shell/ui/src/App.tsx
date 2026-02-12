import { useState } from 'react'
import './App.css'
import { ArtifactLoader } from './components/ArtifactLoader'
import { EvseTimeline } from './components/EvseTimeline'
import { ProfileViewer } from './components/ProfileViewer'
import { PowerCurve } from './components/PowerCurve'
import { DiffView } from './components/DiffView'
import type { EvseStateRecord, OcppTraceRecord, PowerSeriesRecord } from './types'

interface RunData {
  evseData: EvseStateRecord[] | null;
  traceData: OcppTraceRecord[] | null;
  powerData: PowerSeriesRecord[] | null;
}

const emptyRun = (): RunData => ({
  evseData: null,
  traceData: null,
  powerData: null,
});

function App() {
  const [isDiffMode, setIsDiffMode] = useState(false);
  const [baseRun, setBaseRun] = useState<RunData>(emptyRun());
  const [compRun, setCompRun] = useState<RunData>(emptyRun());

  const hasBaseData = baseRun.evseData || baseRun.traceData || baseRun.powerData;
  const hasCompData = compRun.evseData || compRun.traceData || compRun.powerData;

  const reset = () => {
    setBaseRun(emptyRun());
    setCompRun(emptyRun());
  };

  return (
    <div className="app-container">
      <header className="main-header">
        <div className="container" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '1rem 2rem' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <div style={{ fontSize: '24px' }}>🔨</div>
            <h1 style={{ margin: 0, fontSize: '1.5rem', letterSpacing: '-0.025em' }}>GridHammer <span style={{ fontWeight: 300, color: '#64748b' }}>Lens</span></h1>
          </div>
          <div style={{ display: 'flex', gap: '20px', alignItems: 'center' }}>
            <label style={{ display: 'flex', alignItems: 'center', gap: '8px', cursor: 'pointer', fontSize: '14px', fontWeight: 500 }}>
                <input 
                    type="checkbox" 
                    checked={isDiffMode} 
                    onChange={(e) => {
                        setIsDiffMode(e.target.checked);
                        reset();
                    }} 
                />
                Comparison Mode
            </label>
            {hasBaseData && <button onClick={reset}>Clear Data</button>}
          </div>
        </div>
      </header>
      
      <main className="container">
        {!isDiffMode ? (
          <div className="dashboard-grid">
            {!hasBaseData ? (
                <div style={{ gridColumn: '1 / -1', marginTop: '40px' }}>
                    <ArtifactLoader 
                        onEvseDataLoaded={(d) => setBaseRun(prev => ({ ...prev, evseData: d }))}
                        onTraceDataLoaded={(d) => setBaseRun(prev => ({ ...prev, traceData: d }))}
                        onPowerDataLoaded={(d) => setBaseRun(prev => ({ ...prev, powerData: d }))}
                    />
                </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '2rem' }}>
                <DataSummaryCard run={baseRun} />

                {baseRun.evseData && (
                    <section>
                        <EvseTimeline records={baseRun.evseData} />
                    </section>
                )}

                {baseRun.powerData && (
                    <section>
                        <PowerCurve data={baseRun.powerData} />
                    </section>
                )}

                {baseRun.traceData && (
                    <section>
                        <ProfileViewer traceData={baseRun.traceData} evseData={baseRun.evseData || []} />
                    </section>
                )}
              </div>
            )}
          </div>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '2rem' }}>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '2rem' }}>
                <div className="run-col">
                    <h3 style={{ marginBottom: '1rem', display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <span style={{ width: '8px', height: '8px', borderRadius: '50%', backgroundColor: '#2563eb' }}></span>
                        Base Run
                    </h3>
                    <ArtifactLoader 
                        onEvseDataLoaded={(d) => setBaseRun(prev => ({ ...prev, evseData: d }))}
                        onTraceDataLoaded={(d) => setBaseRun(prev => ({ ...prev, traceData: d }))}
                        onPowerDataLoaded={(d) => setBaseRun(prev => ({ ...prev, powerData: d }))}
                    />
                    <DataSummaryCard run={baseRun} compact />
                </div>
                <div className="run-col">
                    <h3 style={{ marginBottom: '1rem', display: 'flex', alignItems: 'center', gap: '8px' }}>
                         <span style={{ width: '8px', height: '8px', borderRadius: '50%', backgroundColor: '#db2777' }}></span>
                        Comparison Run
                    </h3>
                    <ArtifactLoader 
                        onEvseDataLoaded={(d) => setCompRun(prev => ({ ...prev, evseData: d }))}
                        onTraceDataLoaded={(d) => setCompRun(prev => ({ ...prev, traceData: d }))}
                        onPowerDataLoaded={(d) => setCompRun(prev => ({ ...prev, powerData: d }))}
                    />
                    <DataSummaryCard run={compRun} compact />
                </div>
            </div>

            {hasBaseData && hasCompData && (
                <section>
                    <DiffView 
                        base={{ ...baseRun, label: 'Base' }} 
                        comparison={{ ...compRun, label: 'Comp' }} 
                    />
                </section>
            )}
          </div>
        )}
      </main>
    </div>
  )
}

function DataSummaryCard({ run, compact }: { run: RunData, compact?: boolean }) {
    const { evseData, traceData, powerData } = run;
    if (!evseData && !traceData && !powerData) return null;
    
    return (
        <div style={{ 
            display: 'flex', 
            gap: '24px', 
            padding: compact ? '12px' : '20px', 
            background: compact ? 'transparent' : 'white',
            borderRadius: '8px',
            border: compact ? '1px solid #e2e8f0' : 'none',
            marginTop: '1rem'
        }}>
            <StatItem label="EVSE Records" value={evseData?.length ?? 0} color="#10b981" />
            <StatItem label="OCPP Messages" value={traceData?.length ?? 0} color="#3b82f6" />
            <StatItem label="Power Samples" value={powerData?.length ?? 0} color="#f59e0b" />
        </div>
    );
}

function StatItem({ label, value, color }: { label: string, value: number, color: string }) {
    return (
        <div>
            <div style={{ fontSize: '12px', color: '#64748b', fontWeight: 500, textTransform: 'uppercase' }}>{label}</div>
            <div style={{ fontSize: '20px', fontWeight: 700, color: value > 0 ? color : '#cbd5e1' }}>{value.toLocaleString()}</div>
        </div>
    );
}

export default App
