use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    start: DateTime<Utc>,
    ticks: u64,
}

impl Clock {
    pub fn new(start: DateTime<Utc>) -> Self {
        Clock { start, ticks: 0 }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.start + Duration::seconds(self.ticks as i64)
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}
