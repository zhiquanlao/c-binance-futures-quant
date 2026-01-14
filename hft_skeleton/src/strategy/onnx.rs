#[derive(Debug)]
pub struct OnnxSession;

impl OnnxSession {
    pub fn load(_path: &str) -> anyhow::Result<Self> {
        // Initialize onnx-ort session for inference.
        todo!("onnx load stub")
    }

    pub fn predict(&self, _features: &[f32]) -> Vec<f32> {
        // Run ONNX inference and return output vector.
        Vec::new()
    }
}
