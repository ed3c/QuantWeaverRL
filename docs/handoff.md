# QuantWeaverRL — zero-context engineering handoff

Generated: 2026-10-12. Purpose: resume the **existing** QuantWeaverRL project and implement its staged paper-trading/agent-RL infrastructure. This is a snapshot, not execution evidence or a replacement for current source.

## Exact continuation

- Repository: https://github.com/ed3c/QuantWeaverRL
- Existing PR: https://github.com/ed3c/QuantWeaverRL/pull/1
- PR branch: `docs/agentic-phased-runtime-contract`
- Original reviewed head before this handoff: `d80c59a8878a25a683cbf868c2b18e5772ef29f3`. **Refresh HEAD before every write/merge.**
- Existing documents: `AGENTS.md`, `docs/architecture.md`, `docs/phases.md`. Read them at the actual selected ref. Do not duplicate their specifications here.
- Soodles: https://github.com/ed3c/soodles (private, requires connected GitHub authorization). Read its current `AGENTS.md`, `contracts/system-v1.md`, and selected skills before operating its workflow.

## User outcome and constraints

Implement, **in phases P0–P8**, a commercially usable (code/data/model/trace rights verified separately), reproducible, paper-only end-to-end Agentic Quant Trading PoC demonstrating backend high concurrency, AI Engineering, Agent Infrastructure, Karotte RL environment and bounded Soodles RSI/harness hill climb. Workloads include synthetic market replay, exchange public-feed normalization, cross-exchange spot arbitrage, funding/basis research, deterministic risk/order simulation, fault recovery, Qlib/RD-Agent(Q) experiments, Jev/DeepSeek/Qwen Decision comparisons and Colab training. Do not promise profit or use real funds.

Hardware: MacBook M1 Pro 16 GB; Google Colab 200 compute units/month, disposable GPU runtime. Local Mac owns development, limited services, replay and evidence. Colab owns optional training/evals and must persist artifacts externally. No 24/7 Colab server or assumed Mac multi-node HA.

## Actual state at handoff

- PR #1 contains **documents only**, not a working trading runtime.
- `AGENTS.md` describes Soodles ownership, Brain A/B challenge, independent Test Manager/Schema Manager readback, bounded `prompt_delta`, seven-decision P-class bound, unknown-write preservation and paper-only controls.
- `docs/architecture.md` defines proposed lanes and state/data contracts.
- `docs/phases.md` defines staged acceptance P0 through P8.
- No Mac runtime execution, Colab training, independent Soodles native behavior verification, CI acceptance, or deployed service has been observed. Treat these as `NOT_RUN` until fresh evidence exists.
- At last read, PR #1 was Draft and open, with zero GitHub check runs on its then-current head. Refresh PR state; never infer checks from this snapshot.

## First actions for next Session

1. Fetch current `main`, open PRs, exact HEAD, saved files, CI/check runs, Soodles source and owner continuations. Reconcile any changes since this handoff. Do not create duplicate issues/PRs/schedulers.
2. Select the original Soodles owner route and run its required scoped P-class behavior feedback for `AGENTS.md` if not already covered by valid evidence. Use its `review-writing` and Test/Schema Manager paths. Keep missing checks `NOT_RUN`, do not substitute a prose review for native behavior.
3. Begin P0 with smallest runnable vertical slice: synthetic redistributable market events, stable IDs and hashes, license/provenance manifest, deterministic oracle and CI. Follow original admitted Issue atom and returned continuation.
4. P1 before throughput: account-scoped risk owner, exact decimal/integer accounting, paper exchange, idempotent order ID, execution receipts, duplicate delivery, ACK loss, partial fills, cancel/fill race, crash/restart, `UNKNOWN_WRITE` reconciliation.
5. P2 then add bounded FastAPI/NATS JetStream/PostgreSQL and fixed-load tests (100/500/1000 RPS targets, **not measured claims**), telemetry, overload control and DB contention evidence.
6. Only then P3 public exchange feeds and isolated demo/testnet contract adapters; P4 Qlib/RD-Agent(Q) alpha baselines; P5 Karotte isolated episodes; P6 Colab Qwen SFT vs Rules/Jev/DeepSeek and justified RL; P7 Soodles causal prompt-delta hill climb; P8 real second-node scaling if required.

## Brain A / Brain B / prompt delta contract

Brain A proposes one causal change supported by observed failure. Brain B independently seeks a counterexample using frozen requirements and a discriminating control. Test Manager chooses scoped tests; independent observer measures baseline/treatment on identical inputs; Schema Manager returns next. Do not run unbounded debate or silently accept a model self-judge. If no demonstrated prompt defect, do not change prompt. For each delta preserve failure reference, baseline, treatment, observer, cost, correctness hard gates, outcome and authoritative receipt. Use Soodles canonical workflow decision accounting; per P-class state machine no more than seven actual Session choices/guesses.

## Hard gates and legal rights

- No live orders, withdrawals, signing keys or money; no LLM/RL bypass of deterministic risk gate.
- Missing ACK is not a failed external write. Read back by stable client order ID before any retry.
- A simulated fill is not an exchange-confirmed fill. Quoted spread is not realized alpha.
- Check **code**, **market data**, **model weights**, and **traces** licenses independently. Synthetic fixtures and self-produced redacted traces are preferred.
- No production throughput/HA/alpha claim without reproducible workload, source SHA, scenario/data hash, hardware, metrics, independent observer and receipts.
- Use accepted Soodles owner continuation for writes/merges; no invented authorization, provider truth or native evidence.

## Suggested skills for the next agent

Use the Skill tool on the *installed, current* copies of: `handoff` for the next compact; Soodles `issue-atom` for admitted execution; `review-writing` for engineering P-class prose; `test-manager` for scoped tests; `verify-soodles` / applicable evals skill for behavior verification; `provider-readback` for external effect reconciliation; Noodle `poteto-mode` when its selected owner calls it. For outer-loop architecture checks use `ponytail` / `ponytail-review` and `waza-check` only when evidence warrants them. Resolve actual skill availability and selected ref; names alone do not prove installation or authorization.

## Definition of completion

Each phase must have source-bound `PASS / FAIL / INCONCLUSIVE / NOT_RUN`, independent acceptance evidence and owner next/terminal receipt. P0–P8 are future work until measured. An accepted PR containing these documents is not a completed end-to-end PoC.
