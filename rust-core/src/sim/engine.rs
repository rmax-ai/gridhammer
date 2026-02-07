use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::clock::Clock;
use super::rng::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
    clock: Clock,
    rng: Rng,
}

impl Engine {
    pub fn new(start: DateTime<Utc>, seed: u64) -> Self {
        Engine {
            clock: Clock::new(start),
            rng: Rng::new(seed),
        }
    }

    pub fn tick(&mut self) {
        self.clock.tick();
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn rng(&mut self) -> &mut Rng {
        &mut self.rng
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_clock_advances() {
        let start = Utc::now();
        let mut engine = Engine::new(start, 42);
        assert_eq!(engine.clock().ticks(), 0);
        engine.tick();
        assert_eq!(engine.clock().ticks(), 1);
    }

    #[test]
    fn test_rng_deterministic() {
        let start = Utc::now();
        let mut engine1 = Engine::new(start, 42);
        let mut engine2 = Engine::new(start, 42);
        let val1: u32 = engine1.rng().generate();
        let val2: u32 = engine2.rng().generate();
        assert_eq!(val1, val2);
    }
}
