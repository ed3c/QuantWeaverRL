# QuantWeaverRL — Session entry

QuantWeaverRL is a reproducible **paper-only** quantitative trading and agent improvement laboratory. It targets MacBook M1 Pro 16 GB plus optional Google Colab GPU (200 CU/month). The intended product is a phased, observable workload for backend concurrency, agent infrastructure, quant research, and RL policy evaluation. These are requirements, not claims of implementation.

## Select the existing work and its owner

Read this file, `docs/architecture.md`, `docs/phases.md`, and the selected contracts before implementing a phase. Relevant contracts: [domain P-class/CLI](docs/contracts/pclass-cli-domain-state-machines.md) and [data lake](docs/data-lake.md). The compact [handoff](docs/handoff.md) locates the latest continuation; it never replaces current source or an owner receipt. Inspect the source, tests, CI, and saved receipts first. Only implement the smallest unmet acceptance outcome. Do not create parallel schedulers, reviewers, managers, or a second Soodles harness.

Soodles owns **Issue execution, Thinking Route, Session decisions, Test Manager selection, Schema Manager feedback, and delivery**. Noodle owns authorized processes and worktrees. QuantWeaverRL owns **market adapters, paper execution, research workloads, environment tasks, data contracts, and workload evidence**. Use the actual selected Soodles owner and returned continuation. A link to Soodles is not evidence that its native runtime ran. Session can make judgments and prepare identity/authorization under current user authority; it cannot invent provider credentials, native process evidence, or completed external writes.

When using Soodles, read its *current committed* `AGENTS.md`, selected skill and `contracts/system-v1.md` at the selected ref. Do not copy its internal control plane into this repository.

## Stack selection — evidence before infrastructure

Target data plane: **Rust Tokio + Axum/Tower + account-scoped Rust paper-execution domain**. Use fixed-point quantities and one Financial Oracle. Begin with a single-process synthetic replay and in-memory execution baseline. Introduce **NATS JetStream / PostgreSQL + SQLx** when a real durable or concurrent caller exists. Keep Rust market-data hot path free of synchronous LLM, Agent sandbox, DuckDB and remote DB calls. Do not migrate a Python feature to Rust solely because Rust is faster in unrelated benchmarks.

Target offline plane: **Python Qlib / RD-Agent(Q), AgentEnv or Karotte**, with NeMo Gym / OpenEnv as interoperability references. **MLX** is an optional compatible Mac inference backend. **Colab Unsloth** is an ephemeral candidate trainer, not a persistent task server. Token-faithful policy-gradient training requires verified token IDs/logprobs/masks; ordinary Agent traces do not prove RL-training compatibility.

Store operational orders/ledger/receipts in **PostgreSQL** when durability is needed. Store time-stamped raw events, supply-chain evidence, financial features and RL trajectories as **Parquet/JSONL**; query with **DuckDB**. Add DuckLake or distributed streaming only when observed readers, writers or data volume require it. Keep code/data/model/trace license and time visibility independently checked.

Three target paper market tracks: **Binance crypto**, **Hyperliquid perps**, **TWSE/MOPS Taiwan equities**. Cathay Securities is a *future, disabled* broker integration candidate; a public retail trade/sandbox API is not proven. No implied real orders or fabricated provider capability.

## P-class and CLI inference ownership

P-class identifies the requirement, semantic judgment, counterexample and acceptance boundary. **Domain CLI / validators**, once implemented, own fact-determined state transitions. Soodles' existing CLI continues to own workflow selection, Issue/PR/CI, admission, Test Manager, Schema Manager and delivery. No QuantWeaverRL clone of those owners.

For a selected domain state machine: consume producer-validated `state`, `evidence_refs`, `status`, `next` and `missing_inputs`. Session performs only remaining semantic decisions; it never reconstructs IDs, command arguments or digests from prose. A descriptive example in a contract is **not executable argv**. If the CLI is not implemented, mark `NOT_IMPLEMENTED`, do the source-level work under the admitted owner, and do not claim runtime execution.

For `UNKNOWN_WRITE`, the **original effect owner** must provide fresh readback before another effect. A forwarded owner or agent name is not a new identity/auth barrier; the present Session consumes the saved continuation. Missing real credentials, native runtime observation or provider truth remain explicit unmet prerequisites.

The [domain state-machine contract](docs/contracts/pclass-cli-domain-state-machines.md) is the single description of proposed market/order/episode/dataset/policy states and their discriminating tests. Preserve Soodles' canonical per-machine seven-actual-decision rule and incomplete-capture semantics; do not invent a second counter. Do not split or rename decisions to hide their cost.

## Shortest implementation route

1. Read the selected phase's observable acceptance and discover existing code, ownership, tests and evidence.
2. Establish a fixed source commit, dataset, scenario seed, hardware, dependency versions and baseline. Reuse valid evidence.
3. Brain A proposes **one minimal causal change**. Brain B independently challenges the premise with an explicit counterexample and discriminating control. Both are proposals, not verdicts.
4. Implement only an observed barrier in the current admitted Issue atom. Prefer deterministic schema/CLI/state-machine code over repeated prompt decisions.
5. Let Soodles Test Manager select focused controls. Evaluate identical baseline and treatment workloads with an independent observer; record `PASS`, `FAIL`, `INCONCLUSIVE`, or `NOT_RUN`.
6. Route verified feedback to the existing Schema Manager and consume its returned next action. Keep the original owner, selected identity, effects, and evidence references.
7. Save a bounded **prompt_delta** only when behavior evidence supports changing instructions. Preserve original requirements and check new Agent behavior separately. Do not claim that prose review, local tests, or a candidate PR authorizes landing.

No unbounded two-brain debate. Count every Session path choice/guess per *single state-machine execution* with Soodles' canonical calculator; maximum seven for P-class guidance. Do not reset counts at a Session boundary or disguise them by renaming steps. Move fact-determined choices into their existing owner. If coverage or decision count cannot be established, mark it unknown.

## Safety and runtime authority

- DEFAULT: public market-data read + deterministic paper execution. No live orders, real funds, withdrawals, margin, API write keys or address signing. Enabling a real-money mode requires a separate explicit scope and independently verified controls; it is outside these phases.
- Keep credentials out of Git, transcripts, generated datasets and telemetry. Reject secrets in records.
- `MarketEvent -> normalized sequence/freshness -> Opportunity -> Risk Gate -> OrderIntent -> Paper Exchange -> Receipt -> Reconciliation` is the proposed execution path. Agents/LLMs/RL do not bypass the deterministic Risk Gate.
- Preserve `UNKNOWN_WRITE`. On missing ACK, query the same paper exchange using the stable `client_order_id`; do not blindly retry a side effect. Duplicate delivery must not create a second order or fill.
- All money/quantities use integer minor units or `Decimal` with explicit tick/lot scales, never binary floats for ledger accounting. Use one logical account risk writer or proven transaction coordination.
- Trading data, model weights, code dependencies and traces each need separately recorded license/provenance, redistribution permission, revision and hash. Repository licenses do not license third-party market data. Use self-generated synthetic data as initial publishable fixtures.

## Runtime environments

**Mac 16 GB**: Rust/Python source and tests, bounded local market replay, paper risk/execution, optional NATS/PostgreSQL services, DuckDB/Parquet, selected AgentEnv or Karotte evaluation and independent local evals. Keep the simplest running process topology that satisfies current acceptance. Start only the selected services. Measure memory and reject OOM-driven overcommit.

**Colab 200 CU/month**: optional, disposable GPU training/evaluation worker for compatible Qwen Decision candidates. Export versioned datasets/checkpoints/metrics to durable storage. Never use Colab as a 24/7 exchange service or persistent database. Do not assume Karotte sandbox VM capabilities are available in Colab. Jev/DeepSeek are optional API benchmarks, not mandatory live dependencies; model and service commercial terms need review.

## Evidence and acceptance

Record `source_sha, scenario_hash, dataset_hash, policy_hash, config_hash, seed, runtime, timestamps, request_id, event_id, account_id, client_order_id, trace_id, test_scope, observer, result, receipt` where applicable. Distinguish simulated market fills from exchange demo API acknowledgments. Store immutable receipts outside disposable worktrees; redact personal and API data.

Performance claims require completed tasks/sec, fixed arrival rates, queue age/lag, p50/p95/p99, rejection rate, DB waits, peak memory, failure-recovery results and cost per valid completion. A single Mac proves local concurrency, **not** multi-node HA or production capacity. Paper P&L does not establish profitable live alpha.

For model improvements, compare fixed deterministic rules, hosted decisions when available, Qwen SFT and later RL under the **same** workload, holdout, cost and safety gates. The independent Oracle must distinguish held-out predictive gain from new market risk and multiple-testing bias. An LLM herd is not presumed to cause real market moves. Report invalid actions, calibration and adverse scenarios. Karotte creates isolated tasks/rollout traces and grading; it is not a trainer. Reward improvement alone is not production acceptance.

Proceed phase-by-phase per `docs/phases.md`. Do not skip failure controls in order to report higher throughput. An empty test selection is not a pass. State precisely which behavior was observed and which remains `NOT_RUN` or `INCONCLUSIVE`.

## Writing and prompt_delta

Engineering instructions should keep actors, conditions, state, effect and causal justification explicit. Use the existing Soodles `review-writing` procedure when this file or engineering docs are revised in an admitted task. For P-class changes use its scoped Agent behavior feedback and Schema Manager readback. Preserve exact IDs, original evidence, status and constraints. A `prompt_delta` is the smallest necessary behavioral adjustment with: `failure_ref, frozen_baseline, observed_barrier, delta, counterexample, independent_eval_ref, cost_delta, retained_hard_gates, outcome`. If no supported behavior defect exists, do not modify the prompt.

Source reference: [Soodles](https://github.com/ed3c/soodles) (requires authorized repository access).