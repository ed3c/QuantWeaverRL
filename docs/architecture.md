# Proposed runtime architecture — Rust backend / Python Agent RL (not deployed)

## Goals and ownership

QuantWeaverRL is a **paper-only**, reproducible high-concurrency financial research and Agent RL laboratory. This document defines a target architecture, not a verified service. MacBook M1 Pro 16 GB runs scoped local workloads; Colab 200 CU/month is disposable GPU training compute. Do not claim multi-node HA, live execution, or profitable Alpha from this design.

Use **Rust** where deterministic market/order processing and bounded I/O matter, and **Python** where established Agent/RL libraries materially reduce implementation work. Retain one Financial Oracle and one Soodles workflow owner. Test a smaller in-memory baseline first and add each dependency only after a caller, failure or scale experiment justifies it.

| Plane | Preferred owner / technology | Reason and simplest fallback |
| --- | --- | --- |
| Market ingest | Rust **Tokio**, bounded channels, exchange-specific public WebSocket adapters | Preserve tick/order-book ordering, freshness, sequence-gap recovery. Start with deterministic synthetic feed and one Rust process. |
| API and admission | Rust **Axum + Tower** | Bounded concurrency, timeout, rejection, rate limits and structured errors. FastAPI is a valid Python reference, not a mandatory parallel API service. |
| Risk and simulated execution | Rust domain types, account-scoped single writer or proven transactional reservation, optional **NautilusTrader** adapter | Exact money arithmetic, deterministic paper fills, `UNKNOWN_WRITE` and external-effect reconciliation. Do not write a second trading oracle in Python. |
| Durable messaging and operational state | **NATS JetStream + PostgreSQL / SQLx** at P2 if warranted | At-least-once delivery and transactional outbox/idempotency. These are not synchronous dependencies of every market tick. |
| Research and agent env | Python **Qlib / RD-Agent(Q) / AgentEnv / Karotte**, OpenEnv or NeMo Gym when needed | Preserve task compatibility and avoid Rust rewrites of existing Agent/RL toolchains. One external sandbox backend per selected experiment. |
| Analytics / RL lake | **Parquet + DuckDB**, optional **DuckLake** when snapshot/concurrent catalog need is observed | Analytical files must not own account state. See [data lake contract](data-lake.md). |
| Inference and training | Jev/DeepSeek as optional cost/latency baselines; Qwen Decision SFT on **Colab/Unsloth**; MLX for compatible Mac local inference | Model serving does not imply support for token/logprob-faithful RL training. NeMo RL/VeRL require separately justified GPU resources. |
| Engineering and quality | Existing **Soodles Session/Thinking Route/Test Manager/Schema Manager** + Noodle worktree owner | Keep decisions, owner continuation, independent readback and Golden Replay. See [P-class / CLI boundary](contracts/pclass-cli-domain-state-machines.md). |

## Runtime separation

```text
REAL-TIME / PAPER DATA PLANE (Mac; Rust)
Synthetic Market Replay or Public Feeds
  -> Snapshot+Delta Sequence/Freshness Gate
  -> In-memory Order Book -> Strategy Signal/Executable-Depth Net Edge
  -> Account Risk Owner -> Paper Execution -> Stable Receipt/Reconciliation
           | (only necessary durable effects/analytics)
           v
  Async Outbox / NATS JetStream -> PostgreSQL Operational State
           |                         |
           +-> append-only Parquet <-+
                      |
OFFLINE RESEARCH / EVAL DATA PLANE (Mac; Python)
Frozen Parquet / DuckDB / PIT Supply-chain Features
  -> Qlib / RD-Agent(Q) Hypotheses + Multi-agent Arena
  -> Financial Domain Oracle (same Rust/Paper rules)
  -> selected AgentEnv OR Karotte Adapter (OpenEnv/NeMo reference)
  -> isolated Trajectories + Hidden Grader + Reward
  -> Frozen train/calibration/blind-holdout manifests
                      |
DISPOSABLE GPU PLANE (Colab)
  -> compatible Unsloth SFT / separately verified RL Trainer
  -> model candidate, checkpoint and metrics
                      |
INDEPENDENT ACCEPTANCE (Mac / existing Soodles)
  -> fixed-seed Golden Replay / Safety Oracle / Cost Evaluation
  -> Soodles Thinking Route -> scoped Issue correction
  -> only evidence-supported prompt_delta; no automatic live trading
```

Three independent performance metrics: **market events/sec**, **durably completed API/jobs/sec** and **valid agent episodes/sec**. No two should be substituted for each other. Reject invalid book data rather than dropping deltas silently. Isolate CPU-bound tasks from Tokio's async executor. External broker/demo adapters are opt-in read/contract checks, not live fund access.

## Financial markets and fairness

- **Binance**: public Spot/Futures books, trades and funding; simulation/testnet APIs only when separately configured and permitted. Confirm historical market-data commercial and redistribution rights.
- **Hyperliquid**: public L2, funding and chain-related events; use local paper matching as the independent oracle. Testnet is not proof of live fill quality.
- **Taiwan equities / Cathay Securities**: TWSE/MOPS open disclosures and calendars, supply-chain time graph. Cathay brokerage API/sandbox capability is unverified; keep a disabled broker adapter interface, never invent order access. Sinotrade Shioaji or other supported simulation APIs are *optional* external adapter comparisons, not replacements for domain truth.
- **Arena**: Rules, multiple hosted/local LLMs and Qwen policies run on the **same** frozen information-time cutoff, fees, initial capital, exposure limits and market dynamics. If testing herding, participant orders must actually affect the simulated book. Separate Binance, Hyperliquid and Taiwan market metrics.

## Mathematical research claims

Measure supply-chain surprise relative to point-in-time forecasts, lagged information propagation with time-valid graph edges, event-study abnormal return/CAR, cross-sectional rank IC, calibrated directional probabilities (Brier/ECE), conditional agent action correlation, exposure concentration, price impact and signal decay. For simulated trading measure executable net return **after** fees/spread/slippage/borrow/funding/FX/rebalancing, max drawdown, CVaR, turnover and capital capacity. Pre-register baseline hypotheses and account for repeated factor searches and backtest overfitting. A correlated LLM decision is not proof that an LLM caused public-market herding.

For Phase detail see [phase plan](phases.md); for storage ownership see [data lake](data-lake.md). For state transitions see [P-class / CLI contract](contracts/pclass-cli-domain-state-machines.md).

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

In a CEX-CEX arbitrage opportunity, two independent venues cannot supply atomic two-leg execution. Simulate failed or partial second legs, hedge/rebalance inventory, queue lag, order-book drift and API disconnection. A positive quoted spread is not realized alpha. Public-market trading performance and matched fairness cannot be proved by synthetic paper outcomes. Track net executed edge after fees, slippage, funding, borrow/rebalance costs and inventory/venue risk.

## State and correctness

Proposed order transitions: `CREATED -> RISK_REJECTED | ADMITTED -> SUBMITTING -> ACCEPTED | REJECTED | UNKNOWN_WRITE -> PARTIALLY_FILLED | FILLED | CANCELLED`. This is not a universal exchange FIX state machine. Store execution reports separately and reconcile out-of-order/duplicate events by stable identifiers and monotonic observed fill, using versioned CAS/transactions. Never derive final external success from a timed-out HTTP request. Preserve `UNKNOWN_WRITE` until fresh owner readback.

Crash tests: duplicate NATS delivery, ACK loss after accepted submit, risk owner concurrent reservations, kill worker at side-effect boundary, DB restart, delayed/duplicated fills, quote staleness, malformed payloads, partial second-leg fills and reconnect sequence gaps. A paper engine must support fault injection and deterministic replay.

## High concurrency: acceptance versus aspiration

Measure (not promise): 100/500/1000 incoming requests per second, 1/2/4 process workers, bounded concurrency and queue length, completed requests/sec, p50/p95/p99, peak RSS, DB pool saturation, backlog recovery and duplicate side effects (must be zero for tested injected scenarios). Fix the arrival distribution, dataset hash, network and CPU limits. Compare baseline with exactly one treatment change. No 50k-RPS or horizontal-HA claim from a single Mac. For Kubernetes/K3s, Ray, Temporal, Kafka/Flink or vLLM, require a measured bottleneck and a second-node comparison first.

## Portable evidence & commercial usability

Keep code/data/model/traces rights independently pinned. Synthetic fixtures and self-generated, redacted traces are the initial redistributable assets. Verify upstream model weights and trading data licenses before release. Keep PII, keys and provider outputs subject to their service terms. Publish reproducible container images, SBOM, dependency versions, test commands, scenario seed and observed metrics; do not label unexecuted stages verified.

## Implementation policy

Small runnable vertical slices beat a speculative full stack. No direct live-trading adapter in initial phases. Paper trading is the only write side effect. External demo/testnet adapters are contract-test candidates and must remain explicitly disabled by default.