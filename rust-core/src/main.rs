use chrono::{DateTime, Utc};
use gridhammer_core::sim::engine::Engine;
use gridhammer_core::sim::tracer::Tracer;
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
        engine.tick();

        // TODO: Retrieve events from engine and trace them
        // For now, we just tick deterministic time
        // In the future:
        // let events = engine.collect_events();
        // for event in events { ... }
    }

    // Flush tracers
    if let Some(t) = ocpp_tracer.as_mut() {
        t.flush()?;
    }
    if let Some(t) = evse_tracer.as_mut() {
        t.flush()?;
    }

    Ok(())
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
        engine.tick();

        // TODO: Retrieve events from engine and trace them
        // For now, we just tick deterministic time
        // In the future:
        // let events = engine.collect_events();
        // for event in events { ... }
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
