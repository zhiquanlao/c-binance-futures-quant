# HFT Skeleton (Rust)

This directory contains a **skeleton-only** Rust layout for the HFT system design.
All methods intentionally stub behavior with comments rather than full implementations.

## Module map

- `data/`: websocket + REST ingestion, normalized into `MarketEvent` and `AccountEvent`.
- `strategy/`: feature computation + LGBM/ONNX model stubs, producing `Signal`.
- `execution/`: OMS state machine + order routing + timeout logic.
- `risk/`: pre-trade checks + reconciliation across multiple account sources.
- `infra/`: lock-free ring buffers, clock utilities, config loading.

## Communication channels (lock-free)

- **Data threads** push into `Ring<MarketEvent>` and `Ring<AccountEvent>`.
- **Strategy thread** consumes `MarketEvent` and pushes `OrderIntent`.
- **Risk thread** consumes `AccountEvent` and publishes `RiskDecision`.
- **Execution thread** consumes `OrderIntent`, consults `RiskDecision`, and emits `OrderCommand`.
- **OMS thread** consumes exchange acks/reports and updates order state.

All channels are intended to be `rtrb` SPSC rings to avoid locks and minimize latency.
