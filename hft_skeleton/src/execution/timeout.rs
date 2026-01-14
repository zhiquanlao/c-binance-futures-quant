use crate::execution::oms::{OrderHandle, OmsStatus};

#[derive(Debug, Clone)]
pub struct TimeoutRule {
    pub max_age_ms: u64,
}

pub struct TimeoutEngine {
    pub rule: TimeoutRule,
}

impl TimeoutEngine {
    pub fn new(rule: TimeoutRule) -> Self {
        Self { rule }
    }

    pub fn expired_orders(&self, _now_ms: u64, _statuses: &[(OrderHandle, OmsStatus, u64)]) -> Vec<OrderHandle> {
        // Return handles that exceeded max_age_ms and are still live/pending.
        Vec::new()
    }
}
