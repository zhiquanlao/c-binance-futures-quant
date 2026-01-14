use crate::strategy::features::FeatureState;

#[derive(Debug, Clone)]
pub struct Signal {
    pub direction: i8,
    pub strength: f64,
    pub target_px_ticks: i64,
    pub expiry_ts_ms: u64,
}

pub struct SignalEngine {
    pub lgbm: Option<super::lgbm::LgbmModelHandle>,
    pub onnx: Option<super::onnx::OnnxSession>,
}

impl SignalEngine {
    pub fn new() -> Self {
        Self { lgbm: None, onnx: None }
    }

    pub fn build_signal(&self, _features: &FeatureState) -> Option<Signal> {
        // Combine feature set with model outputs to generate a signal.
        None
    }
}
