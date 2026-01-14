use crate::execution::router::OrderIntent;

#[derive(Debug, Clone)]
pub struct RiskLimits {
    pub max_position_notional: f64,
    pub max_order_rate_per_sec: u32,
    pub max_daily_loss: f64,
}

#[derive(Debug, Clone)]
pub struct RiskState {
    pub pnl: f64,
    pub exposure_notional: f64,
    pub order_rate: u32,
}

#[derive(Debug, Clone)]
pub enum RiskDecision {
    Allow,
    ReduceOnly,
    Reject,
    Halt,
}

impl RiskLimits {
    pub fn check_pre_trade(&self, _state: &RiskState, _intent: &OrderIntent) -> RiskDecision {
        // Compare intent to limits (position, rate, loss) and return decision.
        RiskDecision::Allow
    }
}
