use crate::data::types::{AccountEvent, MarketEvent, WsEvent};
use crate::infra::{RingConsumer, RingProducer};

pub struct FeedDispatcher {
    pub market_tx: RingProducer<MarketEvent>,
    pub account_tx: RingProducer<AccountEvent>,
}

impl FeedDispatcher {
    pub fn dispatch_ws_event(&mut self, event: WsEvent) {
        // Route events into the correct ring buffer, drop if full.
        match event {
            WsEvent::Market(market) => {
                let _ = self.market_tx.push(market);
            }
            WsEvent::Account(account) => {
                let _ = self.account_tx.push(account);
            }
            WsEvent::Pong | WsEvent::Reconnected => {}
        }
    }
}

pub struct FeedRouter {
    pub market_rx: RingConsumer<MarketEvent>,
    pub account_rx: RingConsumer<AccountEvent>,
}

impl FeedRouter {
    pub fn poll_market(&mut self) -> Option<MarketEvent> {
        self.market_rx.pop().ok()
    }

    pub fn poll_account(&mut self) -> Option<AccountEvent> {
        self.account_rx.pop().ok()
    }
}
