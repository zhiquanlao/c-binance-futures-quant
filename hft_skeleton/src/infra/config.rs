use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub symbols: Vec<String>,
    pub binance_spot_ws: String,
    pub binance_usdt_ws: String,
    pub binance_usdc_ws: String,
    pub user_stream_ws: String,
    pub max_inflight_orders: usize,
    pub risk_limits: RiskLimitsConfig,
}

pub const TICK_PRICE: f64 = 0.01;
pub const TICK_AMT: f64 = 0.001;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitsConfig {
    pub max_position_notional: f64,
    pub max_order_rate_per_sec: u32,
    pub max_daily_loss: f64,
}

impl RuntimeConfig {
    pub fn load_from_path(_path: &str) -> anyhow::Result<Self> {
        // Load config from a file (toml/json/yaml), validate defaults, and return.
        todo!("config loading stub")
    }
}
