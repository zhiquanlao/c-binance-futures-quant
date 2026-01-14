use crate::data::types::{Price, Volume};
use crate::execution::oms::{ClientId, OrderHandle};

#[derive(Debug, Clone, Copy)]
pub struct ExecPolicy {
    pub post_only: bool,
    pub max_chase_ticks: i64,
    pub cancel_after_ms: u64,
    pub max_slippage_ticks: i64,
}

#[derive(Debug, Clone)]
pub struct OrderIntent {
    pub client: ClientId,
    pub side: u8,
    pub qty: Volume,
    pub px: Price,
    pub reduce_only: bool,
    pub ts_ms: u64,
}

#[derive(Debug, Clone)]
pub enum OrderCommand {
    Post { intent: OrderIntent },
    Cancel { handle: OrderHandle, ts_ms: u64 },
    Amend { handle: OrderHandle, new_px: Price, ts_ms: u64 },
    Market {
        client: ClientId,
        side: u8,
        qty: Volume,
        ts_ms: u64,
    },
    ReduceOnly {
        client: ClientId,
        side: u8,
        qty: Volume,
        ts_ms: u64,
    },
}

pub struct OrderRouter {
    pub policy: ExecPolicy,
}

impl OrderRouter {
    pub fn new(policy: ExecPolicy) -> Self {
        Self { policy }
    }

    pub fn to_post_only(&self, intent: OrderIntent) -> OrderCommand {
        // Convert a strategy intent into a post-only order command.
        OrderCommand::Post { intent }
    }

    pub fn maybe_chase(
        &self,
        _handle: OrderHandle,
        _new_px: Price,
        _ts_ms: u64,
    ) -> Option<OrderCommand> {
        // If price moved beyond threshold, return an amend/cancel command.
        None
    }
}
