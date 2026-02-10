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
    <div className="container">
      <header>
        <h1>GridHammer Visualization</h1>
        <div style={{ margin: '10px 0' }}>
            <label>
                <input 
                    type="checkbox" 
                    checked={isDiffMode} 
                    onChange={(e) => {
                        setIsDiffMode(e.target.checked);
                        reset();
                    }} 
                />
                Diff Mode
            </label>
        </div>
      </header>
      
      <main>
        {!isDiffMode ? (
          !hasBaseData ? (
            <ArtifactLoader 
              onEvseDataLoaded={(d) => setBaseRun(prev => ({ ...prev, evseData: d }))}
              onTraceDataLoaded={(d) => setBaseRun(prev => ({ ...prev, traceData: d }))}
              onPowerDataLoaded={(d) => setBaseRun(prev => ({ ...prev, powerData: d }))}
            />
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
              <div style={{ padding: '10px' }}>
                  <button onClick={reset} style={{ margin: '10px' }}>
                    Load Another Run
                  </button>
                  <DataSummary run={baseRun} />
              </div>

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
          )
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', gap: '20px', justifyContent: 'center' }}>
                <div>
                    <h3>Base Run</h3>
                    <ArtifactLoader 
                        onEvseDataLoaded={(d) => setBaseRun(prev => ({ ...prev, evseData: d }))}
                        onTraceDataLoaded={(d) => setBaseRun(prev => ({ ...prev, traceData: d }))}
                        onPowerDataLoaded={(d) => setBaseRun(prev => ({ ...prev, powerData: d }))}
                    />
                    <DataSummary run={baseRun} />
                </div>
                <div>
                    <h3>Comparison Run</h3>
                    <ArtifactLoader 
                        onEvseDataLoaded={(d) => setCompRun(prev => ({ ...prev, evseData: d }))}
                        onTraceDataLoaded={(d) => setCompRun(prev => ({ ...prev, traceData: d }))}
                        onPowerDataLoaded={(d) => setCompRun(prev => ({ ...prev, powerData: d }))}
                    />
                    <DataSummary run={compRun} />
                </div>
            </div>
            
            {(hasBaseData || hasCompData) && (
                <button onClick={reset} style={{ alignSelf: 'center' }}>Reset Both</button>
            )}

            {hasBaseData && hasCompData && (
                <DiffView 
                    base={{ ...baseRun, label: 'Base' }} 
                    comparison={{ ...compRun, label: 'Comp' }} 
                />
            )}
          </div>
        )}
      </main>
    </div>
  )
}

function DataSummary({ run }: { run: RunData }) {
    const { evseData, traceData, powerData } = run;
    if (!evseData && !traceData && !powerData) return null;
    return (
        <div style={{ fontSize: '0.8em', color: '#666' }}>
            {evseData ? `EVSE States (${evseData.length})` : ''} 
            {evseData && (traceData || powerData) ? ' + ' : ''}
            {traceData ? `OCPP Trace (${traceData.length})` : ''}
            {(evseData || traceData) && powerData ? ' + ' : ''}
            {powerData ? `Power Series (${powerData.length})` : ''}
        </div>
    );
}

export default App
