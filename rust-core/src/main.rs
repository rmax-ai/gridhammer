use chrono::{DateTime, Utc};
use gridhammer_core::sim::engine::Engine;
use gridhammer_core::sim::tracer::{Direction, EvseStateRecord, OcppTraceRecord, Tracer};
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufWriter};

#[derive(Deserialize)]
struct RunConfig {
    seed: u64,
    start_time: DateTime<Utc>,
    evse_count: usize,
    ticks: Option<u64>,
    ocpp_trace_path: Option<String>,
    evse_state_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from stdin
    let config: RunConfig = serde_json::from_reader(io::stdin())?;

    let mut engine = Engine::new(config.start_time, config.seed, config.evse_count);

    // Setup output files if provided
    let mut ocpp_tracer = if let Some(path) = config.ocpp_trace_path {
        Some(Tracer::new(BufWriter::new(File::create(path)?)))
    } else {
        None
    };

    let mut evse_tracer = if let Some(path) = config.evse_state_path {
        Some(Tracer::new(BufWriter::new(File::create(path)?)))
    } else {
        None
    };

    // Run loop
    let ticks = config.ticks.unwrap_or(5);
    for _ in 1..=ticks {
        let messages = engine.tick();

        if let Some(tracer) = &mut ocpp_tracer {
            for msg in messages {
                let payload = serde_json::to_value(&msg)?;
                let record = OcppTraceRecord {
                    tick: engine.tick_count(),
                    timestamp: engine.clock().now().to_rfc3339(),
                    direction: Direction::Outbound,
                    action: "TransactionEvent".to_string(),
                    payload,
                };
                tracer.trace_ocpp_message(&record)?;
            }
        }

        if let Some(tracer) = &mut evse_tracer {
            let tick = engine.tick_count();
            let timestamp = engine.clock().now().to_rfc3339();
            for (i, evse) in engine.evses().iter().enumerate() {
                let active_transaction_id = evse
                    .transaction_manager
                    .current_transaction
                    .as_ref()
                    .map(|tx| tx.transaction_id.clone());

                let record = EvseStateRecord {
                    tick,
                    timestamp: timestamp.clone(),
                    evse_id: (i + 1) as u32,
                    state: evse.state,
                    active_transaction_id,
                    enforced_limit: evse.current_limit.clone(),
                };
                tracer.trace_evse_state(&record)?;
            }
        }
    }

    // Flush tracers
    if let Some(mut t) = ocpp_tracer {
        t.flush()?;
    }
    if let Some(mut t) = evse_tracer {
        t.flush()?;
    }

    Ok(())
}
