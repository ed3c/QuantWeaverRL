# Proposed runtime architecture (not deployed)

## Goals and ownership

QuantWeaverRL is an end-to-end **paper-only**, high-concurrency agentic quant experiment. It is not an exchange, broker, custody service, or autonomous real-money trading bot. This design optimizes correctness, repeatability, measurable concurrency, then optional distributed scaling.

- **Market-data boundary**: Binance/OKX/Bybit public feeds (subject to access/terms), normalized snapshots, sequence gaps and freshness. Sourced data must be licensed separately. Initial tests use synthetic data.
- **Execution boundary**: bounded FastAPI admission -> NATS JetStream durable delivery -> deterministic opportunity detection -> account-scoped Risk Gate -> Paper Exchange -> PostgreSQL ledger/receipts. Implement a smaller synchronous in-memory baseline first; add messaging/DB only to meet a demonstrated need.
- **Research boundary**: Qlib baseline; RD-Agent(Q) optional proposal/evaluation loop. No research agent sends orders directly.
- **RL boundary**: Karotte tasks and isolated mock-exchange failure episodes -> rollout transcripts/graded outcomes -> validated train/calibration/held-out splits -> optional Colab Unsloth Qwen Decision SFT -> local independent evaluation. Later RL needs a separately verified trainer/decision-head interface.
- **Improvement boundary**: Soodles existing Thinking Route (Brain A minimal hypothesis, Brain B independent counterexample), Noodle issue-atom execution, Test Manager focused evidence, Schema Manager readback, bounded prompt_delta. QuantWeaverRL must not re-implement Soodles authority.

## Runtime data contracts

```text
Synthetic Replay / Public WS
    -> MarketEvent(exchange,symbol,seq,event_time,receive_time,book,source_revision)
    -> Sequence/Freshness Gate
    -> Opportunity(intent_id, fees, depth, inventory, hypothesis_version)
    -> Risk Owner(account_id, limits, open_orders, exposure)
    -> OrderIntent(stable client_order_id, scenario_id, policy_version)
    -> Paper Exchange(accepted/rejected/partial/filled/cancelled/unknown)
    -> ExecutionReceipt(exchange_order_id?, execution_id?, cumulative_fill, observation_time)
    -> Ledger + Reconciliation + Immutable Event/Trace Ref
                         |
                         +-> Karotte Scenario (faults, market regimes, competition)
                              -> rollout/grades
                              -> Train/Calibration/Blind Test
                              -> Colab candidate adapter
                              -> Independent holdout + safety gates
                              -> Policy registry (candidate / accepted / rejected)
```

In a CEX-CEX arbitrage opportunity, two independent venues cannot supply atomic two-leg execution. Simulate failed or partial second legs, hedge/rebalance inventory, queue lag, order-book drift and API disconnection. A positive quoted spread is not realized alpha. Track net executed edge after fees, slippage, funding, borrow/rebalance costs and inventory/venue risk.

## State and correctness

Proposed order transitions: `CREATED -> RISK_REJECTED | ADMITTED -> SUBMITTING -> ACCEPTED | REJECTED | UNKNOWN_WRITE -> PARTIALLY_FILLED | FILLED | CANCELLED`. This is not a universal exchange FIX state machine. Store execution reports separately and reconcile out-of-order/duplicate events by stable identifiers and monotonic observed fill, using versioned CAS/transactions. Never derive final external success from a timed-out HTTP request. Preserve `UNKNOWN_WRITE` until fresh owner readback.

Crash tests: duplicate NATS delivery, ACK loss after accepted submit, risk owner concurrent reservations, kill worker at side-effect boundary, DB restart, delayed/duplicated fills, quote staleness, malformed payloads, partial second-leg fills and reconnect sequence gaps. A paper engine must support fault injection and deterministic replay.

## High concurrency: acceptance versus aspiration

Measure (not promise): 100/500/1000 incoming requests per second, 1/2/4 process workers, bounded concurrency and queue length, completed requests/sec, p50/p95/p99, peak RSS, DB pool saturation, backlog recovery and duplicate side effects (must be zero for tested injected scenarios). Fix the arrival distribution, dataset hash, network and CPU limits. Compare baseline with exactly one treatment change. No 50k-RPS or horizontal-HA claim from a single Mac. For Kubernetes/K3s, Ray, Temporal, Kafka/Flink or vLLM, require a measured bottleneck and a second-node comparison first.

## Portable evidence & commercial usability

Keep code/data/model/traces rights independently pinned. Synthetic fixtures and self-generated, redacted traces are the initial redistributable assets. Verify upstream model weights and trading data licenses before release. Keep PII, keys and provider outputs subject to their service terms. Publish reproducible container images, SBOM, dependency versions, test commands, scenario seed and observed metrics; do not label unexecuted stages verified.

## Implementation policy

Small runnable vertical slices beat a speculative full stack. No direct live-trading adapter in initial phases. Paper trading is the only write side effect. External demo/testnet adapters are contract-test candidates and must remain explicitly disabled by default.