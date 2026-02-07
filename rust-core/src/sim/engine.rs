use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;

use super::clock::Clock;
use super::rng::Rng;
use super::tracer::{EvseStateRecord, Tracer};
use crate::evse::model::Evse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    clock: Clock,
    rng: Rng,
    evses: Vec<Evse>,
}

impl Engine {
    pub fn new(start: DateTime<Utc>, seed: u64, evse_count: u32) -> Self {
        let mut evses = Vec::with_capacity(evse_count as usize);
        for id in 1..=evse_count {
            evses.push(Evse::new(id));
        }

        Engine {
            clock: Clock::new(start),
            rng: Rng::new(seed),
            evses,
        }
    }

    pub fn tick(&mut self) {
        self.clock.tick();
        // Here we would also tick EVSEs if they had time-dependent logic
        // for evse in &mut self.evses {
        //     evse.tick_transaction().ok();
        // }
    }

    pub fn emit_state_trace<W: Write>(&self, tracer: &mut Tracer<W>) -> std::io::Result<()> {
        let timestamp = self.clock.now().to_rfc3339();
        let tick = self.clock.ticks();

        for evse in &self.evses {
            let record = EvseStateRecord {
                tick,
                timestamp: timestamp.clone(),
                evse_id: evse.id,
                state: evse.state,
            };
            tracer.trace_evse_state(&record)?;
        }
        Ok(())
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn rng(&mut self) -> &mut Rng {
        &mut self.rng
    }

    pub fn evses_mut(&mut self) -> &mut Vec<Evse> {
        &mut self.evses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_clock_advances() {
        let start = Utc::now();
        let mut engine = Engine::new(start, 42, 1);
        assert_eq!(engine.clock().ticks(), 0);
        engine.tick();
        assert_eq!(engine.clock().ticks(), 1);
    }

    #[test]
    fn test_rng_deterministic() {
        let start = Utc::now();
        let mut engine1 = Engine::new(start, 42, 1);
        let mut engine2 = Engine::new(start, 42, 1);
        let val1: u32 = engine1.rng().generate();
        let val2: u32 = engine2.rng().generate();
        assert_eq!(val1, val2);
    }

    #[test]
    fn test_artifact_emission() {
        let start = Utc::now();
        let mut engine = Engine::new(start, 42, 2);

        // Use a buffer as the writer
        let mut buffer = Vec::new();
        let mut tracer = Tracer::new(&mut buffer);

        // Tick once
        engine.tick();

        // Emit trace
        engine.emit_state_trace(&mut tracer).unwrap();

        let output = String::from_utf8(buffer).unwrap();

        // Should have 2 lines (one for each EVSE)
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 2);

        assert!(lines[0].contains("\"evse_id\":1"));
        assert!(lines[1].contains("\"evse_id\":2"));
        assert!(lines[0].contains("\"tick\":1"));
    }
}
