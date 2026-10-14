# QuantWeaverRL staged implementation and acceptance

Each phase is a separate, bounded engineering outcome. **Do not infer completion from this plan.** Source/owners are pinned by the admitted work item. Tests are selected by Soodles Test Manager; avoid full-suite runs without scope.

| Phase | Smallest working deliverable | Required independent evidence | Deferred until observed need |
| --- | --- | --- | --- |
| **P0 Provenance & deterministic baseline** | Rust Cargo workspace + Python uv workspace; integer/Decimal domain types; self-generated L2/events; input-time and license manifest; CI for selected tests | Same seed gives same event/receipt hash, fixed schemas, no secrets, all 4 rights categories recorded | Kubernetes, GPU Trainer, second API framework |
| **P1 Trading correctness** | Rust market-book validation, account-scoped risk writer, deterministic paper exchange, receipts and readback | ACK-loss after accepted order, idempotent duplicate delivery/fills, partial two-leg arbitrage, cancel/fill race, concurrent reserve, crash/replay | Real orders, broker credentials, profitability claim |
| **P2 Measured high-concurrency backend** | Rust Tokio + Axum/Tower; add NATS JetStream + PostgreSQL/SQLx and outbox only when callers/observations justify; OTel | 100/500/1000 incoming RPS workload, **completed** work/sec, p50/p95/p99, queue lag, DB saturation, overload/rejection, correctness unchanged | Default microservice mesh, 50K sustained RPS claims |
| **P3 Three-market adapters and arena** | Binance & Hyperliquid public read-only data, TWSE/MOPS disclosures, disabled Cathay brokerage seam; optional OKX/Bybit and demo contracts | Sequence/freshness/refusal tests, exchange-specific calendars, point-in-time as-of cutoff, provenance and fair paper-execution comparisons | Live funding, actual exchange orders, nonexistent Cathay sandbox |
| **P4 Quant + memory supply-chain alpha** | Qlib/RD-Agent(Q) optional research, time-valid HBM/DRAM supply-chain graph and economic surprise, herding/price impact arena | Rank IC, CAR, Brier/ECE, matched homogeneous/diverse LLM controls, net executable backtest after full costs, CVaR and blind walk-forward | Claiming crowd behavior proves real-world LLM causation or stable profits |
| **P5 NeoLab RL Environment** | One Rust Financial Oracle; adapter PoCs for **AgentEnv vs Karotte** and portable OpenEnv/NeMo Gym interface only where needed | Same seed/state/outcome under adapters, hidden grader isolation, reward tampering/egress/OOM controls, completed valid episodes/sec, canonical traces | Running multiple sandbox frameworks by default |
| **P6 Model / RL training** | Rules + Jev/DeepSeek API baselines, Qwen3.5 Decision SFT/Calibration in Colab, versioned weights and manifests; RL only with compatible model/token trainer | Same hidden holdout, F1/Accuracy/ECE/Brier, logprob/token-mask integrity where required, safety, cost per valid decision, learning gain beyond SFT | Calling SFT “RL”, assuming MLX or hosted API supports online gradients |
| **P7 Soodles RSI harness hill climb** | Use *existing* Session/Thinking Route/Noodle/Test/Schema owner; single causal atom; fixed Brain A/B controls; scoped prompt_delta and Golden Replay | Frozen baseline/treatment, actual Session decisions (canonical <=7 P-class), no correctness regression, independent owner/consumer readback | Parallel Soodles clone, self-reported RSI gain |
| **P8 Distributed reference** | Add a second real node; K3s, Ray, Temporal, Kafka/Flink, NeMo RL/VeRL or ClickHouse only for evidenced bottlenecks | Matched load, scaling efficiency, network-partition/worker failover, telemetry, costs, stable domain invariants | Claiming Mac/Colab is high-availability production deployment |

## P0 → P1 prerequisite chain

- Begin with self-generated synthetic data and own redacted traces. Verify dataset publication rights; never copy restricted Binance, Hyperliquid or exchange feeds into public fixtures.
- Implement one stable `event_id`, `client_order_id`, `account_id`, and exact quantity/price representation.
- Confirm the deterministic Oracle before Agent or RL policy controls the action.
- Run fault controls before concurrency optimization. Persist owner receipts before state transitions are reported complete.

## P2 workload acceptance contract

A successful request is a durably accepted or explicitly rejected action with a traceable receipt; **202/queued alone is not completed task throughput**. Overload may return a typed 429/503 and retry-after policy. Queue wait is separately measured from handler latency; end-to-end latency includes admission through terminal state. Use at-least-once messaging semantics; deduplicate at the effect boundary. DB consistency uses a transaction/reservation owner, not optimistic multi-worker assumptions.

## P4 alpha hypothesis tests

Compare spot cross-exchange, spot/perpetual basis and inventory rebalancing on fixed executable depth/time-aligned quotes. Include delayed second-leg prices, maker/taker schedules, funding, transfer/rebalance cost, borrow/margin risk and non-fill. Track opportunity frequency, spread lifetime, realized net P&L, drawdown, CVaR, turnover and capacity. Synthetic success is correctness evidence, not forward profitability evidence.

## P5/P6 NeoLab and RL validity

One Rust financial oracle must own order/ledger/reward truth; AgentEnv and Karotte are alternate environment adapters, not parallel oracles. Reward must be based on state oracle, not just a model grader. Penalize duplicate effects and safety violations as hard failures. Keep training, calibration and blind test splits disjoint by scenario family and market time; evaluate out-of-distribution faults. SFT is the initial budget baseline, not a claim that online RL training occurred. Before policy-gradient/GRPO, validate original token IDs, logprobs, masks, policy version and trainer compatibility. If rules already solve the problem, keep rules. Never let training output automatically update an active policy.

## P7 two-brain decision accounting

Brain A proposes the smallest observed causal atom. Brain B receives a clean frozen requirement, baseline and candidate delta, and proposes a disconfirming control. A Soodles owner chooses selected verification; independent evidence accepts/rejects/inconclusive. Prompt changes must be evidence-bound; measure each actual Session choice or guess at the same execution start and completion boundary and report separate token/time/test costs. P-class per-machine decision bound is <=7; rely on Soodles' canonical calculator, not manual counting.

## Resource budgets

MacBook M1 Pro 16 GB: run selected Rust/Python services, DuckDB and local paper simulations only as needed; keep only required services running. Benchmark memory and swap and do not co-host full LLM training. Colab 200 CU/month: initial planning envelope 40 CU baseline, 80 CU candidate SFT, 40 CU eval/rollouts, 20 CU ablation, 20 CU reserve; **planning only**, update with observed CU/hour. Colab is disposable, not a persistent daemon or database.

## Storage and agent harness handoffs

See [data ownership](data-lake.md), [runtime architecture](architecture.md), and [P-class/CLI transition contract](contracts/pclass-cli-domain-state-machines.md). PostgreSQL owns operational account state, Parquet/DuckDB owns historical analytics, DuckLake is optional when snapshot/concurrent metadata needs are observed. A saved Soodles prompt is not proof of consumer behavior; only its selected native agent observation and Schema Manager feedback can support an Agent behavior claim.

## Outcome ledger

Every phase must end with one of `PASS / FAIL / INCONCLUSIVE / NOT_RUN`, linking test report, exact source head, fixture hash, oracle version, runtime/CI, costs and authoritative continuation. A phase plan is not an acceptance receipt.