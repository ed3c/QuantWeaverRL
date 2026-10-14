# QuantWeaverRL — zero-context Soodles handoff

Updated: **2026-10-14**. This compact records the current task intent and source locations; it is **not** evidence that a runtime, test or provider action succeeded. Always refresh the GitHub HEAD and owner next before continuing. It follows Soodles `.agents/skills/handoff/SKILL.md`; do not copy completed specs into this handoff.

## Where to resume

- **Target repository**: https://github.com/ed3c/QuantWeaverRL
- **Verified documentation baseline**: PR [#2](https://github.com/ed3c/QuantWeaverRL/pull/2) merged into `main` at `d3e1a37ae22a420c3256f5a6aff8c849c4e2db94`. Local and remote `main` matched this commit at continuation entry.
- **Current P0 Source work**: Issue [#3](https://github.com/ed3c/QuantWeaverRL/issues/3), branch `codex/p0-synthetic-replay`, in an App-managed isolated worktree. Main remains unchanged.
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

The documentation baseline is merged. At P0 continuation entry, `main` contains only documentation and has no Rust backend, domain CLI or CI workflow.

The current authorized Session selected Issue #3 for a finite Rust synthetic L2/trade replay. Soodles is pinned at `784f922c569e728bfdd34ed0f9b7bb2e2b7e21e2`. Its original `issue_admission.parse_contract` accepted the fresh GitHub Issue body. Its original `test_manager.target_scope` returned `ready` for the fixed external target binding, with `next=null` and `authorizes_landing=false`. Selected commands are `cargo fmt --all -- --check`, `cargo test --locked -p quantweaver-replay`, and `python3 scripts/verify_replay.py`. The selected Source preparation is not Native admission.

The external selection, real Issue readback, original Test Manager response and initial refusal/correction history are retained at `/Users/neon/.local/share/QuantWeaverRL/evidence/p0-20261014/`. These are machine-local references, not portable provider receipts.

The admitted Native path is not established. Generic admission requires a committed target workflow and actual carrier/provider evidence. Soodles' `contract-prepare` and first-Source native continuation do not currently support this generic target. Its AGENTS explicitly permits ordinary Source edits without an atom. Do not fabricate a Noodle writer, envelope, native outcome or alternate admission. Use the existing generic supervisor producer when its prerequisites exist.

The Rust replay is now implemented on the Source branch. The three selected local commands passed on Apple M1 Pro / 16 GiB with Rust 1.93.0. Five process-level Rust tests passed. The independent Python file/arithmetic observer checked four golden seeds, matched fresh 64-event runs, a different seed, debug/release at u64::MAX, and six planted corruption refusals. The saved report is `local-run-1/verification.json` under the external evidence directory. It records dirty source plus the compiled input digests; do not relabel it as clean exact-head CI acceptance.

For seed 42 / 64 events, both fresh runs produced event hash `7924a24b038c3f8891b6e118659e0b21d1bd6ef489666546dc219d49266c5004` and receipt hash `8322801b2962c409cf9abb7c0227b452a0902fda87d7d679c7314d747f277b5a`. The independently recomputed cumulative trade quote atoms were `7808445`. Seed 43 changed both hashes. A separate read-only Source observer recomputed actual line digests and integer amounts, confirming the matched and changed-seed cases.

The original `schema_manager.project_target_scope` returned `ready` with the same `next=null`. Its unknowns remain `target_command_execution` and `target_normal_use`; this projection does not validate the separate Source result. Native admission, native Agent behavior and Schema Manager behavior feedback remain `NOT_RUN`. The P0 slice has local Source/process evidence; the entire P0 phase remains incomplete. P1/P2, Mac backend benchmarks and Colab training remain `NOT_RUN`. Observation receipts must not be interpreted as order fills. See [replay contract and controls](p0-replay.md).

## Immediate next work (do not re-plan P0–P8)

1. Fresh-read Issue #3, the current Source branch/main, PR and checks. Documentation PR #2 is already merged; do not replay it. Preserve the existing Issue and external Test Manager selection.
2. Preserve the merged P-class guidance's unobserved native consumer acceptance. This Source slice changes no AGENTS or domain state-machine guidance and requests no new P-class comparison. Use the existing Soodles review-writing/behavior-feedback route when that separate requirement is selected; prose or replay PASS does not satisfy it.
3. Read Issue #3's Source PR and exact-head CI through the existing provider owner. The workflow checks out the PR head and runs the same selected controls. Preserve the original Test Manager binding and external evidence. Python uv/agent workloads and the paper-order Oracle remain later P0/P1 outcomes; this slice does not accept the whole phase. Establish actual generic Native admission only through its existing supervisor producer after prerequisites exist.
4. Implement **P1** account risk/reservation, paper order lifecycle, stable IDs and unknown-write/readback controls. Require duplicate, stale-book, ACK-loss, concurrent-account and partial-fill negative tests before optimizing throughput.
5. Implement **P2** bounded Rust admission API, measured concurrency, NATS/PostgreSQL only with concrete durable callers; record completed work, p99, queue lag, RSS, DB waits, overload and cost. Later phases P3–P8 are specified in [phases](phases.md); never declare them complete from documents.
6. Update the existing Soodles owner with source-bound accepted/rejected/inconclusive observations and exact continuation. Do not create duplicate scheduler or handcraft a Noodle admission.

## Suggested skills

Invoke the Skill tool for the selected **current** versions of `handoff`, Soodles `review-writing`, `issue-atom`, `test-manager`, `verify-soodles`/appropriate evals skill, `provider-readback`, and the original Noodle `poteto-mode` where admission selects it. Outer-loop `ponytail-review` and `waza-check` only for source-grounded design/review questions. Skill names are hints, not proof of installation.

## Completion and safety

Every phase needs authoritative `PASS | FAIL | INCONCLUSIVE | NOT_RUN` with source/ref, scenario and dataset hashes, selected controls, independent observation and effect receipt. Missing evidence is not a failure or a PASS. If a write is uncertain, the *original effect owner* performs fresh readback before another effect. Any handoff or documentation merge does not establish runnable trading/RL infrastructure.
