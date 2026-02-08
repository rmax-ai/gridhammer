use chrono::{DateTime, Utc};

use super::clock::Clock;
use super::rng::Rng;
use crate::evse::model::Evse;

pub struct Engine {
    clock: Clock,
    #[allow(dead_code)]
    rng: Rng,
    evses: Vec<Evse>,
}

impl Engine {
    pub fn new(start: DateTime<Utc>, seed: u64, evse_count: usize) -> Self {
        let mut evses = Vec::with_capacity(evse_count);
        for i in 0..evse_count {
            evses.push(Evse::new((i + 1) as u32));
        }

        Self {
            clock: Clock::new(start),
            rng: Rng::new(seed),
            evses,
        }
    }

    pub fn tick(&mut self) {
        self.clock.tick();
        // TODO: In the future, we will process events here or trigger Evse::tick_transaction
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn evse_count(&self) -> usize {
        self.evses.len()
    }

    pub fn evses(&self) -> &[Evse] {
        &self.evses
    }

    pub fn tick_count(&self) -> u64 {
        self.clock.tick_count()
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
