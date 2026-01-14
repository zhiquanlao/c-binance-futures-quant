pub mod data;
pub mod execution;
pub mod infra;
pub mod risk;
pub mod strategy;

// Re-export core types for easier access by downstream apps.
pub use data::types::{AccountEvent, MarketEvent, WsEvent};
pub use execution::oms::{ClientId, ExchId, Oms, OmsEvent, OrderEntry, OrderHandle};
pub use strategy::signals::Signal;
