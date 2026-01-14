use crate::data::types::PositionSnapshot;

#[derive(Debug, Clone)]
pub struct PositionState {
    pub positions: Vec<PositionSnapshot>,
}

impl PositionState {
    pub fn apply_snapshot(&mut self, _snap: PositionSnapshot) {
        // Update in-memory positions with latest snapshot data.
    }
}
