use common::tokio::time::Instant;
use std::time::Duration;

use super::super::TIMER_M;

#[derive(Debug, Clone)]
pub struct Completed {
    pub entered_at: Instant,
}

impl Completed {
    pub fn should_terminate(&self) -> bool {
        self.entered_at.elapsed() > Duration::from_millis(TIMER_M)
    }
}
