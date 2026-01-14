use crate::data::types::{BestBidAskTicks, OrderBookL2, TradePrint};
use crate::infra::TimestampMs;

#[derive(Debug, Clone)]
pub struct FeatureState {
    pub last_ts_ms: TimestampMs,
    pub return_20k: f64,
    pub microprice: f64,
    pub order_imbalance: f64,
}

impl FeatureState {
    pub fn new() -> Self {
        Self {
            last_ts_ms: 0,
            return_20k: 0.0,
            microprice: 0.0,
            order_imbalance: 0.0,
        }
    }

    pub fn update_from_book(&mut self, _book: &OrderBookL2, _bbo: &BestBidAskTicks) {
        // Update features derived from orderbook state.
    }

    pub fn update_from_trade(&mut self, _trade: &TradePrint) {
        // Update features derived from trades (e.g., return window, flow).
    }
}
