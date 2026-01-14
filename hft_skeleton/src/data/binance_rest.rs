use crate::data::types::PositionSnapshot;
use crate::infra::TimestampMs;

#[derive(Debug, Clone)]
pub struct RestSnapshot {
    pub positions: Vec<PositionSnapshot>,
    pub balance_usdt: f64,
    pub ts_ms: TimestampMs,
}

pub struct RestClient;

impl RestClient {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch_position_risk(&self) -> anyhow::Result<RestSnapshot> {
        // Call /fapi/v2/positionRisk and build a snapshot.
        todo!("rest fetch stub")
    }

    pub fn fetch_account(&self) -> anyhow::Result<RestSnapshot> {
        // Call /fapi/v2/account and build a snapshot.
        todo!("rest fetch stub")
    }
}
