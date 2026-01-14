use crate::infra::{TimestampMs, TICK_AMT, TICK_PRICE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

pub type Price = i64;
pub type Volume = i64;

#[derive(Debug, Clone)]
pub struct BestBidAskTicks {
    // Best bid/ask in ticks. Fill in your own representation.
    pub bid: Price,
    pub ask: Price,
}

#[derive(Debug, Clone)]
pub struct OrderBookL2 {
    // Fast L2 book representation (vectorized levels, price->size maps, etc.).
    // Keep it cache-friendly and avoid allocations in the hot path.
    // Implementations should use integer comparisons on Price/Volume.
}

#[derive(Debug, Clone)]
pub struct TradePrint {
    pub px_ticks: Price,
    pub qty_lots: Volume,
    pub is_buy: bool,
    pub ts_ms: TimestampMs,
}

#[derive(Debug, Clone)]
pub struct PositionSnapshot {
    pub symbol: String,
    pub qty_lots: Volume,
    pub entry_px_ticks: Price,
    pub ts_ms: TimestampMs,
}

#[derive(Debug, Clone)]
pub enum MarketEvent {
    OrderBookUpdate {
        symbol: String,
        book: OrderBookL2,
        bbo: BestBidAskTicks,
        ts_ms: TimestampMs,
    },
    Trade {
        symbol: String,
        trade: TradePrint,
    },
}

#[derive(Debug, Clone)]
pub enum AccountEvent {
    BalanceUpdate {
        asset: String,
        free: f64,
        locked: f64,
        ts_ms: TimestampMs,
    },
    PositionUpdate(PositionSnapshot),
    OrderUpdate {
        client_id: u64,
        exchange_id: u64,
        status: String,
        filled_qty: Volume,
        ts_ms: TimestampMs,
    },
}

#[derive(Debug, Clone)]
pub enum WsEvent {
    Market(MarketEvent),
    Account(AccountEvent),
    Pong,
    Reconnected,
}

pub fn price_to_ticks(raw_price: f64) -> Price {
    // Round to nearest tick to handle 4.999999 -> 5.
    (raw_price / TICK_PRICE).round() as Price
}

pub fn ticks_to_price(ticks: Price) -> f64 {
    // Ensure output is a multiple of tick size.
    (ticks as f64) * TICK_PRICE
}

pub fn amount_to_ticks(raw_amount: f64) -> Volume {
    // Round to nearest tick to handle float errors.
    (raw_amount / TICK_AMT).round() as Volume
}

pub fn ticks_to_amount(ticks: Volume) -> f64 {
    // Ensure output is a multiple of tick amount.
    (ticks as f64) * TICK_AMT
}
