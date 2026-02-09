import { useState } from 'react'
import './App.css'
import { ArtifactLoader } from './components/ArtifactLoader'
import { EvseTimeline } from './components/EvseTimeline'
import { ProfileViewer } from './components/ProfileViewer'
import type { EvseStateRecord, OcppTraceRecord } from './types'

function App() {
  const [evseData, setEvseData] = useState<EvseStateRecord[] | null>(null)
  const [traceData, setTraceData] = useState<OcppTraceRecord[] | null>(null)

  const hasData = evseData || traceData;

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
          />
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ padding: '10px' }}>
                <button 
                  onClick={() => { setEvseData(null); setTraceData(null); }} 
                  style={{ margin: '10px' }}
                >
                  Load Another Run
                </button>
                <div style={{ fontSize: '0.9em', color: '#666' }}>
                    Loaded: {evseData ? `EVSE States (${evseData.length})` : ''} 
                    {evseData && traceData ? ' + ' : ''}
                    {traceData ? `OCPP Trace (${traceData.length})` : ''}
                </div>
            </div>

            {evseData && (
                <section>
                    <h2>EVSE Timelines</h2>
                    <EvseTimeline records={evseData} />
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
