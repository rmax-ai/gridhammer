use chrono::{DateTime, Utc};
use gridhammer_core::sim::engine::Engine;
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct RunConfig {
    seed: u64,
    start_time: DateTime<Utc>,
    evse_count: usize,
    ticks: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from stdin
    let config: RunConfig = serde_json::from_reader(io::stdin())?;

    let mut engine = Engine::new(config.start_time, config.seed, config.evse_count);

    println!("Starting GridHammer Core...");
    println!("Clock start: {}", engine.clock().now());

    // Run loop
    let ticks = config.ticks.unwrap_or(5);
    for i in 1..=ticks {
        engine.tick();
        // In the future, we will emit JSON artifacts here
        println!("Tick {}: {}", i, engine.clock().now());
    }

    Ok(())
}
