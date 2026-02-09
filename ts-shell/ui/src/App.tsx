import { useState } from 'react'
import './App.css'
import { ArtifactLoader } from './components/ArtifactLoader'
import { EvseTimeline } from './components/EvseTimeline'
import { ProfileViewer } from './components/ProfileViewer'
import { PowerCurve } from './components/PowerCurve'
import type { EvseStateRecord, OcppTraceRecord, PowerSeriesRecord } from './types'

function App() {
  const [evseData, setEvseData] = useState<EvseStateRecord[] | null>(null)
  const [traceData, setTraceData] = useState<OcppTraceRecord[] | null>(null)
  const [powerData, setPowerData] = useState<PowerSeriesRecord[] | null>(null)

  const hasData = evseData || traceData || powerData;

  return (
    <div className="container">
      <header>
        <h1>GridHammer Visualization</h1>
      </header>
      
      <main>
        {!hasData ? (
          <ArtifactLoader 
            onEvseDataLoaded={setEvseData}
            onTraceDataLoaded={setTraceData}
            onPowerDataLoaded={setPowerData}
          />
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ padding: '10px' }}>
                <button 
                  onClick={() => { setEvseData(null); setTraceData(null); setPowerData(null); }} 
                  style={{ margin: '10px' }}
                >
                  Load Another Run
                </button>
                <div style={{ fontSize: '0.9em', color: '#666' }}>
                    Loaded: {evseData ? `EVSE States (${evseData.length})` : ''} 
                    {evseData && (traceData || powerData) ? ' + ' : ''}
                    {traceData ? `OCPP Trace (${traceData.length})` : ''}
                    {(evseData || traceData) && powerData ? ' + ' : ''}
                    {powerData ? `Power Series (${powerData.length})` : ''}
                </div>
            </div>

            {evseData && (
                <section>
                    <EvseTimeline records={evseData} />
                </section>
            )}

            {powerData && (
                <section>
                    <PowerCurve data={powerData} />
                </section>
            )}

            {traceData && (
                <section>
                    <ProfileViewer traceData={traceData} evseData={evseData || []} />
                </section>
            )}
          </div>
        )}
      </main>
    </div>
  )
}

export default App
