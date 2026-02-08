use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
pub struct Clock {
    start_time: DateTime<Utc>,
    tick_count: u64,
    tick_duration: Duration,
}

impl Clock {
    pub fn new(start: DateTime<Utc>) -> Self {
        Self {
            start_time: start,
            tick_count: 0,
            tick_duration: Duration::seconds(1),
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.start_time + self.tick_duration * self.tick_count as i32
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
}
