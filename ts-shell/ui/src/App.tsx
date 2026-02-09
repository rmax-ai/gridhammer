import { useState } from 'react'
import './App.css'
import { ArtifactLoader } from './components/ArtifactLoader'
import { EvseTimeline } from './components/EvseTimeline'
import type { EvseStateRecord } from './types'

function App() {
  const [data, setData] = useState<EvseStateRecord[] | null>(null)

  return (
    <div className="container">
      <header>
        <h1>GridHammer: EVSE Timelines</h1>
      </header>
      
      <main>
        {!data ? (
          <ArtifactLoader onDataLoaded={setData} />
        ) : (
          <>
            <button 
              onClick={() => setData(null)} 
              style={{ margin: '20px' }}
            >
              Load Another File
            </button>
            <EvseTimeline records={data} />
          </>
        )}
      </main>
    </div>
  )
}

export default App
