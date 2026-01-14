use crate::infra::TimestampMs;

#[derive(Debug, Clone)]
pub struct BestBidAskTicks {
    // Best bid/ask in ticks. Fill in your own representation.
    pub bid: i64,
    pub ask: i64,
}

#[derive(Debug, Clone)]
pub struct OrderBookL2 {
    // Fast L2 book representation (vectorized levels, price->size maps, etc.).
    // Keep it cache-friendly and avoid allocations in the hot path.
}

#[derive(Debug, Clone)]
pub struct TradePrint {
    pub px_ticks: i64,
    pub qty_lots: i64,
    pub is_buy: bool,
    pub ts_ms: TimestampMs,
}

#[derive(Debug, Clone)]
pub struct PositionSnapshot {
    pub symbol: String,
    pub qty_lots: i64,
    pub entry_px_ticks: i64,
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
        filled_qty: i64,
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
