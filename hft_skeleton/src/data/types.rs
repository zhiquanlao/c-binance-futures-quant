use crate::infra::TimestampMs;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

// pub type Price = i64;   // in ticks
// pub type Volume = i64;  // in base units or lots
pub type Price = f64; // in ticks
pub type Volume = f64; // in base units or lots
#[derive(Debug, Clone, Copy)]
pub struct BestBidAskTicks {
    pub ts_ms: u64,
    pub bid_px: f64,
    pub bid_amt: f64,
    pub ask_px: f64,
    pub ask_amt: f64,
}
impl BestBidAskTicks {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            ts_ms: 0,
            bid_px: 0.0,
            bid_amt: 0.0,
            ask_px: 0.0,
            ask_amt: 0.0,
        }
    }
    /// Order-Flow Imbalance (OFI) between `prev` -> `self` using your convention:
    /// Bid side:
    ///   +q_t if p_t > p_{t-1}
    ///   q_t - q_{t-1} if p_t == p_{t-1}
    ///   -q_t if p_t < p_{t-1}
    /// Ask side:
    ///   +q_t if p_t < p_{t-1}
    ///   q_t - q_{t-1} if p_t == p_{t-1}
    ///   -q_t if p_t > p_{t-1}
    /// Returns (of_bid, of_ask, ofi_total = of_bid - of_ask).
    #[inline(always)]
    pub fn ofi_against(&self, prev: &BestBidAskTicks) -> (f64, f64, f64) {
        const EPS: f64 = 1e-12;

        #[inline(always)]
        fn side_ofi(p_t: f64, q_t: f64, p_p: f64, q_p: f64, is_bid: bool) -> f64 {
            // Treat missing/invalid quotes as neutral
            if !(p_t.is_finite() && q_t.is_finite()) {
                return 0.0;
            }

            // No previous quote seen -> treat as arrival
            if !(p_p.is_finite() && q_p.is_finite()) {
                return q_t.max(0.0);
            }

            let better = if is_bid {
                p_t > p_p + EPS
            } else {
                p_t + EPS < p_p
            };
            let equal = (p_t - p_p).abs() <= EPS;

            if better {
                q_t
            } else if equal {
                q_t - q_p
            } else {
                -q_t
            }
        }

        let of_bid = side_ofi(self.bid_px, self.bid_amt, prev.bid_px, prev.bid_amt, true);
        let of_ask = side_ofi(self.ask_px, self.ask_amt, prev.ask_px, prev.ask_amt, false);
        (of_bid, of_ask, of_bid - of_ask)
    }

    /// Convenience: compute OFI series for a slice of ticks.
    /// Returns Vec of (ts_ms, of_bid, of_ask, ofi_total).
    #[inline]
    pub fn ofi_series(ticks: &[BestBidAskTicks]) -> Vec<(u64, f64, f64, f64)> {
        if ticks.len() <= 1 {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(ticks.len() - 1);
        for w in ticks.windows(2) {
            let (prev, curr) = (&w[0], &w[1]);
            let (ob, oa, ot) = curr.ofi_against(prev);
            out.push((curr.ts_ms, ob, oa, ot));
        }
        out
    }

    pub fn from_orderbook(ob: &OrderBookL2) -> Option<Self> {
        let (bid_px, bid_amt) = ob.best_bid_level()?;
        let (ask_px, ask_amt) = ob.best_ask_level()?;
        let ts_ms = ob.ts_ms;
        Some(Self {
            ts_ms,
            bid_px,
            bid_amt,
            ask_px,
            ask_amt,
        })
    }

    #[inline(always)]
    pub fn update_from_orderbook(&mut self, ob: &OrderBookL2) -> bool {
        if let Some((bid_px, bid_amt)) = ob.best_bid_level() {
            if let Some((ask_px, ask_amt)) = ob.best_ask_level() {
                self.ts_ms = ob.ts_ms;
                self.bid_px = bid_px;
                self.bid_amt = bid_amt;
                self.ask_px = ask_px;
                self.ask_amt = ask_amt;
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub fn mid_price(&self) -> Option<f64> {
        if self.bid_px.is_finite() && self.ask_px.is_finite() {
            Some(0.5 * (self.bid_px + self.ask_px))
        } else {
            None
        }
    }
}

#[inline(never)]
pub unsafe fn slice_assume_init_ref<T>(s: &[std::mem::MaybeUninit<T>]) -> &[T] {
    &*(s as *const [std::mem::MaybeUninit<T>] as *const [T])
}

#[derive(Debug, Clone, Copy)]
struct PriceLevel {
    price: Price,
    size: Volume,
}

impl PriceLevel {
    #[inline(always)]
    fn new(price: Price, size: Volume) -> Self {
        Self { price, size }
    }
}

pub const MAX_LEVELS: usize = 16; // reasonable for L2 book

pub struct OrderBookL2 {
    pub ts_ms: u64,
    bids: [MaybeUninit<PriceLevel>; MAX_LEVELS],
    asks: [MaybeUninit<PriceLevel>; MAX_LEVELS],
    bid_depth: usize,
    ask_depth: usize,
}

impl OrderBookL2 {
    pub fn new() -> Self {
        Self {
            // bids: unsafe { MaybeUninit::uninit().assume_init() },
            // asks: unsafe { MaybeUninit::uninit().assume_init() },
            ts_ms: 0,
            bids: [MaybeUninit::uninit(); MAX_LEVELS],
            asks: [MaybeUninit::uninit(); MAX_LEVELS],
            bid_depth: 0,
            ask_depth: 0,
        }
    }

    #[inline(always)]
    pub fn best_bid(&self) -> Option<Price> {
        if (self.bid_depth > 0) {
            Some(unsafe { self.bids[self.bid_depth - 1].assume_init_ref().price })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn best_ask(&self) -> Option<Price> {
        if (self.ask_depth > 0) {
            Some(unsafe { self.asks[self.ask_depth - 1].assume_init_ref().price })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn update_level(&mut self, side: Side, price: Price, size: Volume) {
        // println!("[update_level] side: {:?}, price: {}, size: {}", side, price, size);
        let (levels, depth) = match side {
            Side::Bid => (&mut self.bids, &mut self.bid_depth),
            Side::Ask => (&mut self.asks, &mut self.ask_depth),
        };

        for i in 0..*depth {
            let level = unsafe { levels[i].assume_init_mut() };
            if (level.price - price).abs() < PRICE_EPSILON {
                if size.abs() < TICK_AMOUNT / 100.0 {
                    // delete level
                    for j in i..(*depth - 1) {
                        levels[j] = levels[j + 1];
                    }
                    *depth -= 1;
                } else {
                    level.size = size;
                }
                return;
            }
        }

        // level not found
        if size.abs() < TICK_AMOUNT / 100.0 {
            return;
        }
        if *depth < MAX_LEVELS {
            // insert sorted (worst → best, best at end)
            let mut i = *depth;
            while i > 0 {
                let prev = unsafe { levels[i - 1].assume_init_ref() };
                let better = match side {
                    Side::Bid => prev.price > price + PRICE_EPSILON,
                    Side::Ask => prev.price < price - PRICE_EPSILON,
                };
                if better {
                    levels[i] = levels[i - 1];
                    i -= 1;
                } else {
                    break;
                }
            }
            levels[i] = MaybeUninit::new(PriceLevel::new(price, size));
            *depth += 1;
        } else {
            // Book is full: compare to worst at index 0
            let worst_px = unsafe { levels[0].assume_init_ref().price };
            let incoming_better = match side {
                Side::Bid => price > worst_px + PRICE_EPSILON,
                Side::Ask => price < worst_px - PRICE_EPSILON,
            };
            if !incoming_better {
                return; // keep existing top-16
            }
            // Remove worst: shift left one position
            for j in 0..(*depth - 1) {
                levels[j] = levels[j + 1];
            }
            // depth stays the same; now run your insertion block to place the new one
            let mut i = *depth - 1;
            while i > 0 {
                let prev = unsafe { levels[i - 1].assume_init_ref() };
                let prev_better = match side {
                    Side::Bid => prev.price > price + PRICE_EPSILON,
                    Side::Ask => prev.price < price - PRICE_EPSILON,
                };
                if prev_better {
                    levels[i] = levels[i - 1];
                    i -= 1;
                } else {
                    break;
                }
            }
            levels[i] = MaybeUninit::new(PriceLevel::new(price, size));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bid_depth == 0 && self.ask_depth == 0
    }
    pub fn clear(&mut self) {
        self.bid_depth = 0;
        self.ask_depth = 0;
        for level in &mut self.bids {
            *level = MaybeUninit::uninit();
        }
        for level in &mut self.asks {
            *level = MaybeUninit::uninit();
        }
    }
    pub fn levels_as_real(&self, side: Side) -> Vec<(f64, f64)> {
        let (arr, depth) = match side {
            Side::Bid => (&self.bids, self.bid_depth),
            Side::Ask => (&self.asks, self.ask_depth),
        };
        let lvls: &[PriceLevel] = unsafe { slice_assume_init_ref(&arr[..depth]) };
        // best at end → return best→worst
        lvls.iter()
            .rev()
            .map(|lvl| {
                // (lvl.price as f64 * TICK_PRICE, lvl.size as f64 * TICK_AMOUNT)
                (lvl.price as f64, lvl.size as f64)
            })
            .collect()
    }
    #[inline]
    fn top_raw(&self, side: Side) -> Option<(f64 /*px*/, f64 /*sz*/, usize /*idx*/)> {
        match side {
            Side::Bid => {
                if self.bid_depth == 0 {
                    return None;
                }
                let i = self.bid_depth - 1;
                let lvl = unsafe { self.bids.get_unchecked(i).assume_init_ref() };
                Some((lvl.price, lvl.size, i))
            }
            Side::Ask => {
                if self.ask_depth == 0 {
                    return None;
                }
                let i = self.ask_depth - 1;
                let lvl = unsafe { self.asks.get_unchecked(i).assume_init_ref() };
                Some((lvl.price, lvl.size, i))
            }
        }
    }

    #[inline]
    fn level(&self, side: Side, idx: usize) -> Option<(f64, f64)> {
        match side {
            Side::Bid => {
                if idx >= self.bid_depth {
                    return None;
                }
                let lvl = unsafe { self.bids.get_unchecked(idx).assume_init_ref() };
                Some((lvl.price, lvl.size))
            }
            Side::Ask => {
                if idx >= self.ask_depth {
                    return None;
                }
                let lvl = unsafe { self.asks.get_unchecked(idx).assume_init_ref() };
                Some((lvl.price, lvl.size))
            }
        }
    }
    pub fn best_bid_level(&self) -> Option<(f64, f64)> {
        if self.bid_depth == 0 {
            return None;
        }
        let lvl = unsafe {
            self.bids
                .get_unchecked(self.bid_depth - 1)
                .assume_init_ref()
        };
        Some((lvl.price, lvl.size))
    }
    pub fn best_ask_level(&self) -> Option<(f64, f64)> {
        if self.ask_depth == 0 {
            return None;
        }
        let lvl = unsafe {
            self.asks
                .get_unchecked(self.ask_depth - 1)
                .assume_init_ref()
        };
        Some((lvl.price, lvl.size))
    }
    // cumulative depth up to and including idx (0 is deepest kept)
    #[inline]
    pub fn cumulative_to(&self, side: Side, idx_inclusive: usize) -> f64 {
        let end = match side {
            Side::Bid => idx_inclusive.min(self.bid_depth.saturating_sub(1)),
            Side::Ask => idx_inclusive.min(self.ask_depth.saturating_sub(1)),
        };
        let mut acc = 0.0;
        for i in 0..=end {
            if let Some((_, s)) = self.level(side, i) {
                acc += s;
            }
        }
        acc
    }

    // first “wall” within `scan` deeper levels: sudden size jump vs two levels behind
    pub fn find_wall(&self, side: Side, scan: usize) -> Option<f64> {
        let Some((_best_px, _best_sz, top)) = self.top_raw(side) else {
            return None;
        };
        let start = top.saturating_sub(1);
        let end = start.saturating_sub(scan.min(start));
        let mut i = start;
        loop {
            if i < 2 {
                break;
            }
            let (p_i, s_i) = self.level(side, i).unwrap();
            let (_, s1) = self.level(side, i - 1).unwrap();
            let (_, s2) = self.level(side, i - 2).unwrap();
            if s_i > (s1 + s2) {
                return Some(p_i);
            }
            if i == end {
                break;
            }
            i -= 1;
        }
        None
    }

    // generic: top / (sum of top N) for either side; N>=1
    pub fn top_over_top_n(&self, side: Side, n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }
        let Some((_px0, s0, top)) = self.top_raw(side) else {
            return 0.0;
        };
        let mut sum = s0;
        let mut taken = 1usize;
        let mut i = top.saturating_sub(1);
        while taken < n
            && (match side {
                Side::Bid => i < self.bid_depth,
                Side::Ask => i < self.ask_depth,
            })
        {
            if let Some((_, s)) = self.level(side, i) {
                sum += s;
            }
            taken += 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        if sum > 0.0 { s0 / sum } else { 0.0 }
    }

    // top bid size / top ask size (for your ratio rule)
    pub fn top_ba_ratio(&self) -> f64 {
        let b = self.top_raw(Side::Bid).map(|(_, s, _)| s).unwrap_or(0.0);
        let a = self.top_raw(Side::Ask).map(|(_, s, _)| s).unwrap_or(0.0);
        if a > 0.0 { b / a } else { 0.0 }
    }
    #[inline]
    pub fn vwap_top_n(&self, side: Side, n: usize) -> Option<f64> {
        if n == 0 {
            return None;
        }
        let Some((_px0, s0, top)) = self.top_raw(side) else {
            return None;
        };
        let mut v_sum = s0 * _px0;
        let mut s_sum = s0;
        let mut taken = 1usize;
        let mut i = top.saturating_sub(1);
        while taken < n
            && (match side {
                Side::Bid => i < self.bid_depth,
                Side::Ask => i < self.ask_depth,
            })
        {
            if let Some((p, s)) = self.level(side, i) {
                v_sum += s * p;
                s_sum += s;
            }
            taken += 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        if s_sum > 1e-9 {
            Some(v_sum / s_sum)
        } else {
            None
        }
    }
    #[inline]
    pub fn obi_top_n(&self, n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }
        let bid_sum: f64 = self.cumulative_to(Side::Bid, n - 1);
        let ask_sum: f64 = self.cumulative_to(Side::Ask, n - 1);
        let denom = bid_sum + ask_sum;
        if denom > 1e-9 {
            (bid_sum - ask_sum) / denom
        } else {
            0.0
        }
    }
    #[inline]
    pub fn microprice_and_effspread_top_n(
        &self,
        n: usize,
    ) -> Option<(
        f64, /*micro_N*/
        f64, /*eff_spread_N*/
        f64, /*mid*/
    )> {
        // Use your existing best_* if VWAP_N not available:
        let (bid_vwap, ask_vwap) =
            match (self.vwap_top_n(Side::Bid, n), self.vwap_top_n(Side::Ask, n)) {
                (Some(b), Some(a)) => (b, a),
                _ => {
                    // fallback to top level mid
                    let (bid, ask) = (self.best_bid()?, self.best_ask()?);
                    return Some((
                        (bid + ask) * 0.5,
                        (ask - bid).max(TICK_PRICE),
                        (bid + ask) * 0.5,
                    ));
                }
            };
        let eff = (ask_vwap - bid_vwap).max(TICK_PRICE);
        let obi = self.obi_top_n(n); // (ΣB-ΣA)/(ΣB+ΣA); implement if not present
        let micro_n = 0.5 * (ask_vwap + bid_vwap) + 0.5 * eff * obi;
        let mid = 0.5 * (self.best_bid()? + self.best_ask()?);
        Some((micro_n, eff, mid))
    }
    /// Weighted OrderBook Imbalance over top-N:
    /// weights(i) = 1 - i/N for i=0..N-1 (level 0 has weight 1.0).
    /// Returns (WB - WA) / (WB + WA), or 0.0 if denominator ~ 0.
    #[inline]
    pub fn weighted_obi_top_n(&self, n: usize) -> f64 {
        const EPS: f64 = 1e-12;
        if n == 0 {
            return 0.0;
        }

        let wb = self.weighted_sum_volume(Side::Bid, n);
        let wa = self.weighted_sum_volume(Side::Ask, n);
        let denom = wb + wa;
        if denom.abs() < EPS {
            0.0
        } else {
            (wb - wa) / denom
        }
    }

    /// Weighted pressure over top-N on a single side:
    /// sum_{i=0..N-1} price_i * size_i * (1 - i/N).
    #[inline]
    pub fn weighted_pressure_side_top_n(&self, side: Side, n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }
        let depth = match side {
            Side::Bid => self.bid_depth,
            Side::Ask => self.ask_depth,
        };
        let upto = n.min(depth);
        if upto == 0 {
            return 0.0;
        }

        let nf = n as f64;
        let mut acc = 0.0;
        for i in 0..upto {
            if let Some((px, sz)) = self.level(side, depth - i - 1) {
                let w = 1.0 - (i as f64) / nf;
                acc += px * sz * w;
            } else {
                break;
            }
        }
        acc
    }

    /// Weighted pressure imbalance over top-N:
    /// (PB - PA) / (PB + PA), 0.0 if denominator ~ 0
    #[inline]
    pub fn weighted_pressure_imbalance_top_n(&self, n: usize) -> f64 {
        const EPS: f64 = 1e-12;
        let pb = self.weighted_pressure_side_top_n(Side::Bid, n);
        let pa = self.weighted_pressure_side_top_n(Side::Ask, n);
        let denom = pb + pa;
        if denom.abs() < EPS {
            0.0
        } else {
            (pb - pa) / denom
        }
    }

    /// Distance of top-N VWAPs from the MID price.
    /// Returns (mid - vwap_bid_top_n, vwap_ask_top_n - mid).
    /// None if mid or either VWAP is unavailable.
    #[inline]
    pub fn vwap_distance_from_mid_top_n(&self, n: usize) -> Option<(f64, f64)> {
        let mid = (self.best_bid()? + self.best_ask()?) * 0.5;
        let vwap_b = self.vwap_top_n(Side::Bid, n)?;
        let vwap_a = self.vwap_top_n(Side::Ask, n)?;
        Some((mid - vwap_b, vwap_a - mid))
    }

    /// Weighted sum of volumes over top-N with weights 1 - i/N.
    #[inline]
    fn weighted_sum_volume(&self, side: Side, n: usize) -> f64 {
        let depth = match side {
            Side::Bid => self.bid_depth,
            Side::Ask => self.ask_depth,
        };
        let upto = n.min(depth);
        if upto == 0 || n == 0 {
            return 0.0;
        }

        let nf = n as f64;
        let mut acc = 0.0;
        for i in 0..upto {
            if let Some((_px, sz)) = self.level(side, depth - i - 1) {
                let w = 1.0 - (i as f64) / nf;
                acc += sz * w;
            } else {
                break;
            }
        }
        acc
    }
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
