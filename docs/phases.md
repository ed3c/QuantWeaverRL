# QuantWeaverRL staged implementation and acceptance

Each phase is a separate, bounded engineering outcome. **Do not infer completion from this plan.** Source/owners are pinned by the admitted work item. Tests are selected by Soodles Test Manager; avoid full-suite runs without scope.

| Phase | Minimum runnable delivery | Acceptance evidence | Explicit non-goal |
|---|---|---|---|
| P0 License and reproducible baseline | Python project, deterministic synthetic L2/trade fixtures, config, CI unit test, provenance matrix, stable hashes | Clean bootstrap; same seed yields same event sequence; 4-asset license manifest | Broker credentials/live trading |
| P1 Paper trading correctness | Market replay -> signal -> per-account risk -> paper matching -> ledger/receipt + reconciliation | Duplicate intent, ACK loss, partial fill, cancel/fill race, kill/restart and unknown-write controls all pass | Profitable strategy |
| P2 Async backend concurrency | FastAPI admission, bounded NATS JetStream workers, PostgreSQL transaction/outbox, OTel, k6/Locust | Fixed load 100/500/1000 RPS; p99/lag/RSS/error and per-account invariants captured; overload is bounded; baseline/treatment compared | Multi-node HA claim |
| P3 Public market data and exchange adapters | Binance, OKX, Bybit read-only feed; book sequence recovery; isolated demo/testnet order adapter interfaces | Gaps/stale books rejected; API contract tests and stable paper order IDs; exchange data rights reviewed | Live wallet/funds |
| P4 Quant research and alpha verification | Qlib deterministic baseline; optional RD-Agent(Q) proposals; spot/spot and basis/funding paper models | Walk-forward, fee/latency/slippage/inventory-aware executed P&L vs naïve baselines; blind holdout, stress tests; avoid look-ahead | Guaranteed return |
| P5 Agent eval environment | Karotte isolated scenarios and hidden graders for recover/retry/reconcile/stop | Reproducible transcripts, anti-grader tampering, held-out failure suites, no trading gate violations | Karotte training weights |
| P6 Decision models and RL | Rules vs Jev/DeepSeek (API optional) vs Qwen3.5 Decision SFT on Colab; optional compatible RL trainer | Same held-out scenarios, accuracy/F1/ECE/Brier, safety, latency, cost/valid decision, checkpoint/provenance; show incremental RL gain or reject | Assuming hosted/API inputs freely redistributable |
| P7 Soodles RSI harness hill climb | Soodles-selected Issue atom, Brain A/B challenge, independent Test/Schema readback, scoped prompt_delta | Fixed baseline/treatment, fewer actual decisions/rework/cost with no lost hard gate; original owner terminal receipt and replay | Self-reported recursive improvement |
| P8 Distributed reference | Second real node, then K3s/Temporal/Ray/Flink only if bottleneck demands | Baseline on matched workload, horizontal scaling efficiency, partition/recovery, costs, no correctness regression | Claiming Mac/Colab is production cluster |

## P0 → P1 prerequisite chain

- Begin with publicly redistributable synthetic data and own traces; do not copy licensed market feeds to published fixtures.
- Implement one stable `event_id`, `client_order_id`, `account_id`, and exact quantity/price representation.
- Confirm the deterministic Oracle before Agent or RL policy controls the action.
- Run fault controls before concurrency optimization. Persist owner receipts before state transitions are reported complete.

## P2 workload acceptance contract

A successful request is a durably accepted or explicitly rejected action with a traceable receipt; **202/queued alone is not completed task throughput**. Overload may return a typed 429/503 and retry-after policy. Queue wait is separately measured from handler latency; end-to-end latency includes admission through terminal state. Use at-least-once messaging semantics; deduplicate at the effect boundary. DB consistency uses a transaction/reservation owner, not optimistic multi-worker assumptions.

## P4 alpha hypothesis tests

Compare spot cross-exchange, spot/perpetual basis and inventory rebalancing on fixed executable depth/time-aligned quotes. Include delayed second-leg prices, maker/taker schedules, funding, transfer/rebalance cost, borrow/margin risk and non-fill. Track opportunity frequency, spread lifetime, realized net P&L, drawdown, CVaR, turnover and capacity. Synthetic success is correctness evidence, not forward profitability evidence.

## P5/P6 RL validity

Reward must be based on state oracle, not just a model grader. Penalize duplicate effects and safety violations as hard failures. Keep training, calibration and blind test splits disjoint by scenario family and market time; evaluate out-of-distribution faults. SFT must precede a justified RL experiment; if rules already solve the problem, keep rules. Never let training output automatically update an active policy.

## P7 two-brain decision accounting

Brain A proposes the smallest observed causal atom. Brain B receives a clean frozen requirement, baseline and candidate delta, and proposes a disconfirming control. A Soodles owner chooses selected verification; independent evidence accepts/rejects/inconclusive. Prompt changes must be evidence-bound; measure each actual Session choice or guess at the same execution start and completion boundary and report separate token/time/test costs. P-class per-machine decision bound is <=7; rely on Soodles' canonical calculator, not manual counting.

## Resource budgets

MacBook M1 Pro 16 GB: keep only required services running. Benchmark memory and swap and do not co-host full LLM training. Colab 200 CU/month: initial planning envelope 40 CU baseline, 80 CU candidate SFT, 40 CU eval/rollouts, 20 CU ablation, 20 CU reserve; **planning only**, update with observed CU/hour. Colab is disposable, not a persistent daemon or database.

## Outcome ledger

Every phase must end with one of `PASS / FAIL / INCONCLUSIVE / NOT_RUN`, linking test report, exact source head, fixture hash, oracle version, runtime/CI, costs and authoritative continuation. A phase plan is not an acceptance receipt.