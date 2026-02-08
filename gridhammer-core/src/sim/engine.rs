use chrono::{DateTime, Utc};

use super::clock::Clock;
use super::rng::Rng;
use crate::evse::model::Evse;
use crate::evse::state::EvseEvent;
use crate::invariants::assertions::AssertionRegistry;
use crate::ocpp::messages::TransactionEventRequest;

pub struct Engine {
    clock: Clock,
    #[allow(dead_code)]
    rng: Rng,
    evses: Vec<Evse>,
    messages: Vec<TransactionEventRequest>,
    assertions: AssertionRegistry,
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
            messages: Vec::new(),
            assertions: AssertionRegistry::new(),
        }
    }

    pub fn tick(&mut self) -> Vec<TransactionEventRequest> {
        self.clock.tick();
        let now = self.clock.now();
        for evse in &mut self.evses {
            // TODO: handle error
            let _ = evse.tick_transaction(now);
        }
        self.messages.drain(..).collect()
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn evse_count(&self) -> usize {
        self.evses.len()
    }

    pub fn apply_event(
        &mut self,
        evse_id: u32,
        event: EvseEvent,
        transaction_id: Option<String>,
    ) -> Result<(), String> {
        let timestamp = self.clock.now();
        let evse = self
            .evses
            .get_mut((evse_id - 1) as usize)
            .ok_or(format!("Invalid evse_id: {}", evse_id))?;
        let (_, tx_req) = evse.handle_event(event, timestamp, transaction_id)?;
        if let Some(msg) = tx_req {
            self.messages.push(msg);
        }
        Ok(())
    }

    pub fn evses(&self) -> &[Evse] {
        &self.evses
    }

    pub fn assertions(&self) -> &AssertionRegistry {
        &self.assertions
    }

    pub fn tick_count(&self) -> u64 {
        self.clock.tick_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evse::state::{EvseEvent, EvseState};
    use crate::ocpp::messages::TransactionEventEnum;
    use chrono::Duration;

    #[test]
    fn test_engine_time_advances_deterministically() {
        let start = Utc::now();
        let mut engine = Engine::new(start, 42, 5);
        assert_eq!(engine.clock().now(), start);
        let _ = engine.tick();
        assert_eq!(engine.clock().now(), start + Duration::seconds(1));
        let _ = engine.tick();
        assert_eq!(engine.clock().now(), start + Duration::seconds(2));
    }

    #[test]
    fn test_transaction_event_lifecycle() {
        let start = Utc::now();
        let mut engine = Engine::new(start, 42, 1); // 1 EVSE

        let evse_id = 1;

        // Initial state: Available

        // PlugIn -> Preparing
        engine
            .apply_event(evse_id, EvseEvent::PlugIn, None)
            .unwrap();
        let messages = engine.tick();
        assert_eq!(messages.len(), 0); // No TransactionEvent

        // StartCharging -> Charging, emits Started
        engine
            .apply_event(evse_id, EvseEvent::StartCharging, Some("tx1".to_string()))
            .unwrap();
        let messages = engine.tick();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].event_type, TransactionEventEnum::Started);
        assert_eq!(messages[0].transaction_info.transaction_id, "tx1");
        assert_eq!(engine.evses()[0].state, EvseState::Charging);

        // StopCharging -> Available, emits Ended
        engine
            .apply_event(evse_id, EvseEvent::StopCharging, None)
            .unwrap();
        let messages = engine.tick();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].event_type, TransactionEventEnum::Ended);
        assert_eq!(messages[0].transaction_info.transaction_id, "tx1");
        assert_eq!(engine.evses()[0].state, EvseState::Available);
    }
}
