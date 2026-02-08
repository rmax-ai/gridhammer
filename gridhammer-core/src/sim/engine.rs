use chrono::{DateTime, Utc};

use super::clock::Clock;
use super::rng::Rng;

pub struct Engine {
    clock: Clock,
    #[allow(dead_code)]
    rng: Rng,
    evse_count: usize,
}

impl Engine {
    pub fn new(start: DateTime<Utc>, seed: u64, evse_count: usize) -> Self {
        Self {
            clock: Clock::new(start),
            rng: Rng::new(seed),
            evse_count,
        }
    }

    pub fn tick(&mut self) {
        self.clock.tick();
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn evse_count(&self) -> usize {
        self.evse_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_engine_time_advances_deterministically() {
        let start = Utc::now();
        let mut engine = Engine::new(start, 42, 5);
        assert_eq!(engine.clock().now(), start);
        engine.tick();
        assert_eq!(engine.clock().now(), start + Duration::seconds(1));
        engine.tick();
        assert_eq!(engine.clock().now(), start + Duration::seconds(2));
    }
}
