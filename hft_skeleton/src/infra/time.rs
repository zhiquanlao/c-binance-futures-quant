use std::time::{Duration, Instant};

pub type TimestampMs = u64;

#[derive(Debug, Clone)]
pub struct Clock {
    start: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }

    pub fn now_ms(&self) -> TimestampMs {
        self.start.elapsed().as_millis() as TimestampMs
    }

    pub fn sleep_ms(&self, ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
    }
}
