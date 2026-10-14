# P-class guidance ↔ domain CLI state-machine contract (proposed)

**Status: SPECIFICATION ONLY.** At the reviewed source baseline QuantWeaverRL contains documentation but no executable application/CLI. The transitions and response envelope below are requirements for the future QuantWeaverRL domain CLI, **not implemented commands**. Soodles' existing CLI remains the sole owner of Session/Issue/PR, admission, Test Manager, Schema Manager and delivery. Do not implement a second workflow controller here.

## Why this separation

Agents can judge semantics when evidence is incomplete. They must not guess IDs, routes, the next command, or whether an external effect succeeded. P-class describes domain objectives, choices and counterexamples. Once inputs determine a next transition, the responsible domain producer/validator must fix it and return its exact structured continuation. The Session consumes that result unchanged. Soodles manages its own selections and authorizations separately.

**Inputs** come from a selected phase, source-bound scenario/config, a validated artifact manifest, and fresh owner observations. Never generate SHA values, source IDs, market availability, fee schedules or provider credentials from prose. Compute digests and lookups in the producer, reuse its single validated output across consumers. Missing fields remain named missing inputs.

## Proposed domain transition families

| State machine / owner | Preconditions | Program-owned transition, never Agent guess | Legitimate Session judgment | Refusal / unknown and discriminating controls |
| --- | --- | --- | --- | --- |
| **Phase admission** / future Quant domain phase producer | Current committed phase map, prior acceptance receipts, requested outcome | Verify dependency receipts and select the next incomplete phase; return exact task, scope and selected inputs | Whether new evidence requires revising the goal or acceptance claim | `NEEDS_PREREQUISITE`, `UNKNOWN_EVIDENCE`; reject a claimed P2 PASS without P1 correctness |
| **Market book** / Rust ingest owner | Exchange-specific sequence contract, snapshot and deltas, monotonic source times | `EMPTY -> SYNCING -> VALID -> STALE/INVALID -> RESYNCING -> VALID`; reject out-of-sequence/expired books | Whether to investigate a genuinely unexplained event | Drop an update, inject stale timestamps; no pricing from an invalid book |
| **Paper order** / account-scoped Rust risk/execution owner | Valid book, stable intent ID, risk reservation, exact fixed-point amounts | `CREATED -> RISK_REJECTED/ADMITTED -> SUBMITTING -> ACCEPTED/REJECTED/UNKNOWN_WRITE`; fills/cancel/reports advance through verified receipts; deduplicate by ID | Resolution of an unresolved business rule not in original policy | Missing ACK -> **readback by original client_order_id**, never blind retry; concurrent reserve and duplicate fill controls |
| **Environment episode** / Quant domain oracle + selected execution adapter | Immutable scenario/seed, agent policy and adapter identity | `PREPARED -> RESET -> RUNNING -> TERMINAL/FAILED/UNKNOWN`; result includes graded terminal receipt; grader isolated from agent | Which new failure family is worth researching | Tampered reward, escaping sandbox, missing cleanup, changed seed -> reject or unknown |
| **Dataset release** / data validator/catalog owner | Raw provenance, time-availability checks, schema, training/calibration/test splits | `INGESTED -> VALIDATED -> FROZEN -> RELEASED` or `QUARANTINED`; generate hash and snapshot reference | Whether a novel claim of data rights has sufficient external evidence | Future leakage, incomplete data rights, truncated DuckDB query -> `BLOCKED`; no training claim |
| **Policy evaluation** / independent eval owner | Frozen dataset and baselines, versioned model/decision-head, safety oracle | `CANDIDATE -> EVALUATING -> ACCEPTED/REJECTED/INCONCLUSIVE`; never auto-promote on training reward | Whether a new learning hypothesis warrants a scoped experiment | Missing token logprobs or model revisions means online RL compatibility `UNKNOWN`; safety fail rejects |
| **Performance/scaling** / Test Manager-selected workload owner | Fixed replay, hardware/arrival profile and correctness oracle | Select existing scoped controls, collect effective throughput, latency, queue age, RSS, cost and errors | Whether observed bottleneck justifies a new component | 202 response or one-host workers cannot claim completed throughput or multi-node HA |

The arrow labels are **domain semantics**; they are not the same as Soodles Issue/CI state names. Do not take a row's states as a full exchange FIX model. Preserve actual exchange or broker native status and map into explicitly versioned domain fields.

## Proposed validated response envelope

This is a target **data contract**, not a shell command or a claim about Soodles' own response schema.

```json
{
  "schema_version": "domain-next/v1",
  "machine": "paper-order",
  "execution_id": "producer-owned",
  "owner": "paper-execution",
  "state": "UNKNOWN_WRITE",
  "evidence_refs": ["immutable-observation-ref"],
  "status": "NEEDS_READBACK",
  "next": {
    "actor": "Session",
    "operation": "READBACK_EXISTING_ORDER",
    "inputs": {"client_order_id": "original-stable-id"},
    "effect_class": "read_only"
  },
  "missing_inputs": []
}
```

Only the actual producer may issue executable `argv` after validation and authorization. A `next` in a design document is descriptive and non-executable. If the result references another owner, the **same Session** continues with that owner's selected entry; do not require a different identity or duplicate admission. Before an uncertain write repeat, the original side-effect owner must return fresh readback.

## Thinking Route and prompt_delta

For one **observed** defect, Brain A proposes the smallest candidate treatment. Brain B receives the frozen task, independent baseline and treatment description, not the expected answer, and proposes a disconfirming test. If the defect is mechanically determined, do not ask either model. The independent observer checks that hard gates remain; Soodles Test Manager chooses the smallest required controls. Schema Manager consumes returned evidence; the current Session uses the owner's continuation. Keep both a failed and successful outcome as evidence.

`prompt_delta` is **not** a free-form optimization log: `failure_ref, original_requirement_ref, observed_barrier, baseline_ref, treatment_ref, counterexample_ref, independent_eval_ref, effect_readback_ref, cost_delta, remaining_requirements, outcome`. A claim with no native consumer observation must remain `NOT_RUN` or `INCONCLUSIVE`. A change in a Rust function does not automatically justify changing AGENTS.md.

## Decision accounting and verification boundary

Soodles' **canonical decision calculator** counts each actual Session path choice or guess in one P-class state-machine execution, including repeats and cross-Session continuations. Maximum seven. Command count, field count, deterministic program branches and tool calls are separate. Use its complete coverage semantics; incomplete capture does not imply the count is within bounds. Do not implement a competing decision counter here.

| Required discriminating test | Negative control | Passing condition |
| --- | --- | --- |
| Owner selection | Wrong saved owner/phase ref | Reject instead of selecting an invented route |
| Admission | P2 requested without P1 receipt | Return prerequisite, no false PASS |
| Market ingestion | Missing L2 delta / timestamp regression | Invalidate book and resync |
| Trading order | ACK lost after accepted paper order | Original ID readback, one effective order |
| Portfolio risk | Concurrent reservations to same account | Never exceed limits |
| Data release | Disclosure first_seen after decision cutoff | Quarantine future feature |
| RL training | Trace with missing logprobs labeled as GRPO-ready | Refuse online-RL compatibility claim |
| Environment | Agent modifies protected oracle/grader | Failure, not reward |
| Deployment | One-host concurrency labeled multi-node HA | Unknown/unsupported |
| Model acceptance | High reward but hidden safety regression | Reject candidate |

## Acceptance and implementation location

First implement the **domain state machines** inside P0/P1 Rust crates and tests. Add the state response adapter only if a real caller consumes it. Do not generate hypothetical CLI commands for docs. Domain tests prove domain transitions only. Soodles P-class *Agent behavior* still requires fresh consumer execution, the original feedback skill, independent observer, Schema Manager readback and owner continuation. Current state: **not implemented; native behavior verification NOT_RUN**.

Source procedure: Soodles `.agents/skills/review-writing/SKILL.md`, `.agents/skills/review-writing/features/ste-reference.md` (Issue 9 rules 3.6, 5.1, 6.3, 9.1 as clarity cues), `contracts/system-v1/common.md`, `.agents/skills/test-manager/SKILL.md`. These are references to current upstream authority, not copied source code.
