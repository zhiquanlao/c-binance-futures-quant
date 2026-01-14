#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(pub u64);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExchId(pub u64);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderHandle(pub u32);

#[derive(Clone, Copy)]
pub enum OmsStatus {
    Dead = 0,
    PendingPost = 1,
    Live = 2,
    PendingCancel = 3,
}

use crate::data::types::{Price, Volume};

pub struct OrderEntry {
    pub inst: u32,
    pub side: u8,
    pub px: Price,
    pub qty: Volume,
    pub cum_filled: Volume,
    pub client: ClientId,
    pub exch: ExchId,
    pub has_exch: bool,
    pub status: OmsStatus,
    pub last_ts_ms: u64,
    pub last_seq: u64,
}

pub struct IdMap {
    keys: Vec<u64>,
    vals: Vec<OrderHandle>,
    states: Vec<u8>,
    mask: usize,
}

impl IdMap {
    pub fn get(&self, _key: u64) -> Option<OrderHandle> {
        // O(1) probe lookup into open addressing table.
        None
    }

    pub fn insert(&mut self, _key: u64, _h: OrderHandle) {
        // O(1) probe insert with tombstones.
    }

    pub fn remove(&mut self, _key: u64) -> Option<OrderHandle> {
        // Tombstone removal; return handle if found.
        None
    }
}

pub struct Oms {
    orders: Vec<OrderEntry>,
    free: Vec<OrderHandle>,
    by_client: IdMap,
    by_exch: IdMap,
    live_by_inst: Vec<Vec<OrderHandle>>,
}

pub enum OmsEvent {
    CmdAck(CmdAck),
    CmdReject(CmdReject),
    ExecReport(ExecReport),
    Timer(TimerEv),
}

#[derive(Clone, Copy)]
pub enum CmdKind {
    Post,
    Cancel,
    Amend,
}

pub struct CmdAck {
    pub client: ClientId,
    pub exch: Option<ExchId>,
    pub kind: CmdKind,
    pub status_hint: Option<ExchOrderStatus>,
    pub ts_ms: u64,
}

pub struct CmdReject {
    pub client: ClientId,
    pub kind: CmdKind,
    pub err_code: i32,
    pub ts_ms: u64,
}

#[derive(Clone, Copy)]
pub enum ExchExecType {
    New,
    Trade,
    Canceled,
    Expired,
    Amendment,
}

#[derive(Clone, Copy)]
pub enum ExchOrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Expired,
}

pub struct ExecReport {
    pub client: ClientId,
    pub exch: ExchId,
    pub exec_type: ExchExecType,
    pub order_status: ExchOrderStatus,
    pub cum_filled: Volume,
    pub last_fill: Volume,
    pub last_fill_px: Price,
    pub ts_ms: u64,
    pub seq: u64,
}

pub struct TimerEv {
    pub ts_ms: u64,
}

impl Oms {
    pub fn on_event(&mut self, _ev: OmsEvent) {
        // Dispatch events to update order state machine.
    }

    fn cleanup_dead(&mut self, _h: OrderHandle) {
        // Remove indices, return handle to free list, and clear per-instrument live lists.
    }
}
