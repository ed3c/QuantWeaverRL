# QuantWeaverRL — zero-context Soodles handoff

Updated: **2026-10-14**. This compact records the current task intent and source locations; it is **not** evidence that a runtime, test or provider action succeeded. Always refresh the GitHub HEAD and owner next before continuing. It follows Soodles `.agents/skills/handoff/SKILL.md`; do not copy completed specs into this handoff.

## Where to resume

- **Target repository**: https://github.com/ed3c/QuantWeaverRL
- **Current documentation branch for this handoff**: `docs/rust-agentrl-runtime-and-handoff` (refresh state before any action).
- **Previous merged docs PR**: https://github.com/ed3c/QuantWeaverRL/pull/1, merged in main commit `7bd63d599c5fd619ac045806602b694405c2d632`.
- **Selected instructions/specs**: [AGENTS.md](../AGENTS.md), [architecture](architecture.md), [P0–P8 phases](phases.md), [domain P-class/CLI states](contracts/pclass-cli-domain-state-machines.md), [RL lakehouse](data-lake.md).
- **Original engineering owner**: https://github.com/ed3c/soodles. Refresh its `AGENTS.md`, `contracts/system-v1.md`, selected skills and owner receipt. Do **not** re-create its Issue/PR, manager, identity or admission state machines inside QuantWeaverRL.

## The objective

Build a **paper-only**, reproducible, commercially permissioned (rights checked separately for code, data, model, traces) financial Agentic RL platform. Prove distributed-backend *principles* locally before claiming distributed deployment: executable correctness, bounded concurrency, durable state, high-quality point-in-time data, financial signal research, multi-agent LLM herding stress tests, robust RL environments and independently verified harness improvement.

Current user's hardware: MacBook M1 Pro 16 GB + Colab 200 compute units/month. Mac runs synthetic replay, Rust backend and small Python agent environments; Colab is optional disposable SFT / verified-compatible RL training. Do not start a persistent cloud server or treat 200 CU as guaranteed GPU hours.

## Decisions now represented in documents

**Backend:** Rust Tokio + Axum/Tower + account-scoped paper execution; NATS JetStream / PostgreSQL SQLx only after the minimal caller demonstrates need. Operational ledger and exact risk state never belong to DuckDB. Data-lake baseline is Parquet + DuckDB, optionally DuckLake snapshots later. P0 Rust Cargo + Python uv; keep one canonical financial oracle.

**Markets:** Public Binance and Hyperliquid, Taiwan equities via TWSE/MOPS; Cathay Securities remains a disabled, unverified brokerage API seam. Optional OKX/Bybit Demo read/contract tests. Separate per-market paper arena tracks; same capital, timestamps and fee/risk controls for compared agents. No live orders, real funds, withdrawal keys or money.

**Research:** Memory/HBM/DRAM supply-chain point-in-time graph and disclosure surprise; rank IC, event-study CAR, proper probability scores, conditional herding correlation, portfolio concentration, impact and net risk-adjusted returns. No unproved assertion that same-direction real trades were caused by LLMs.

**Agent RL:** AgentEnv vs Karotte *alternate adapters* to the same domain oracle and hidden grader. OpenEnv / NeMo Gym are interoperability and training-contract references. Use Qlib/RD-Agent(Q) research, Rules vs optional Jev/DeepSeek/Qwen baseline; Colab Unsloth SFT first. Claim RL only with trainer-appropriate token/logprob/mask provenance and actual weight updates plus held-out gain. MLX on Mac is an optional compatible inference backend, not NVIDIA RL.

**Soodles:** Brain A proposes one observed causal atom; Brain B challenges with an independent counterexample. Test Manager chooses scoped controls, independent observer checks actual behavior, Schema Manager returns selected next. Only verified defects justify `prompt_delta`. A single P-class machine permits at most seven actual Session choices/guesses, counted using Soodles' calculator, not a new one.

## State at this handoff

During this task, current `main` was read as documentation-only; it had no executable Rust / Python CLI and no open PR before this documentation branch. This branch changes P-class guidance, domain state-machine *specification*, staged architecture, data ownership and this compact. **None of the proposed domain CLI transitions is implemented by a real caller.** The source-level prose may be reviewed, but no fresh Soodles native Agent behavior, CI, Mac hardware or Colab evaluation was observed here. Keep these at `NOT_RUN` or `UNKNOWN` until verified.

## Immediate next work (do not re-plan P0–P8)

1. Fresh-read latest branch, main, open PRs, commits, checks, files and Soodles returned owner continuation. Reconcile the documentation PR first; if accepted, retain the verified main readback.
2. Use Soodles review-writing on saved P-class AGENTS and domain CLI contract (STE reference and source fidelity). Select **fresh consumer behavior** eval only if required by Soodles, then consume Schema Manager feedback. Do not call a prose review an Agent behavior PASS.
3. Start **P0** runnable vertical slice in the current selected Issue atom: Rust Cargo workspace, Python uv only where called, deterministic synthetic books/trades, time/rights schema, exact-money types, simple paper oracle and focused CI.
4. Implement **P1** account risk/reservation, paper order lifecycle, stable IDs and unknown-write/readback controls. Require duplicate, stale-book, ACK-loss, concurrent-account and partial-fill negative tests before optimizing throughput.
5. Implement **P2** bounded Rust admission API, measured concurrency, NATS/PostgreSQL only with concrete durable callers; record completed work, p99, queue lag, RSS, DB waits, overload and cost. Later phases P3–P8 are specified in [phases](phases.md); never declare them complete from documents.
6. Update the existing Soodles owner with source-bound accepted/rejected/inconclusive observations and exact continuation. Do not create duplicate scheduler or handcraft a Noodle admission.

## Suggested skills

Invoke the Skill tool for the selected **current** versions of `handoff`, Soodles `review-writing`, `issue-atom`, `test-manager`, `verify-soodles`/appropriate evals skill, `provider-readback`, and the original Noodle `poteto-mode` where admission selects it. Outer-loop `ponytail-review` and `waza-check` only for source-grounded design/review questions. Skill names are hints, not proof of installation.

## Completion and safety

Every phase needs authoritative `PASS | FAIL | INCONCLUSIVE | NOT_RUN` with source/ref, scenario and dataset hashes, selected controls, independent observation and effect receipt. Missing evidence is not a failure or a PASS. If a write is uncertain, the *original effect owner* performs fresh readback before another effect. Any handoff or documentation merge does not establish runnable trading/RL infrastructure.
