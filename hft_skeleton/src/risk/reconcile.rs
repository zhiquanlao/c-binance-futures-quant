use crate::data::binance_rest::RestSnapshot;
use crate::data::types::PositionSnapshot;

pub struct Reconciler;

impl Reconciler {
    pub fn reconcile_positions(_ws: &[PositionSnapshot], _rest: &RestSnapshot) -> Vec<PositionSnapshot> {
        // Compare multiple sources and choose the freshest data by timestamp.
        Vec::new()
    }
}
