#[derive(Debug)]
pub struct LgbmModelHandle;

impl LgbmModelHandle {
    pub fn load_from_file(_path: &str) -> anyhow::Result<Self> {
        // Load LightGBM model through FFI and return a handle.
        todo!("lgbm load stub")
    }

    pub fn predict(&self, _features: &[f64]) -> f64 {
        // Run inference via LightGBM FFI and return a score.
        0.0
    }
}
