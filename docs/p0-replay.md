# P0 synthetic market observation replay

Scope: [Issue #3](https://github.com/ed3c/QuantWeaverRL/issues/3). This is the first
executable P0 slice. It implements no exchange connection, account risk writer,
paper order lifecycle, server, RL policy or Soodles workflow state machine.

## Input and deterministic bytes

Rust 1.93.0 runs one bounded process. The CLI requires `--seed` as a u64,
`--events` in `1..=1000000`, and `--output` as a new directory. Missing, duplicate
or unknown options fail with a diagnostic and exit 2. The output's parent must
exist. An existing directory, file or symlink is refused without replacement.

The generator uses SplitMix64 with explicit wrapping u64 operations. Each event
consumes six draws in order: midprice, spread, bid quantity, ask quantity, trade
quantity and trade side. The midprice is `10000 + draw % 201`. Spread is
`1 + draw % 5`. Top-level bid/ask sizes are `1 + draw % 100`; the second level
is one tick farther out and one lot larger. Trade size is `1 + draw % 25`.
An even final draw chooses the ask; an odd draw chooses the bid. This simple
synthetic feed supplies repeatable fixtures, not calibrated market behavior.

Each event carries a full two-level book and one trade observation. It is not a
snapshot/delta ingestion implementation. Sequence starts at 1. Event time is
`1700000000000000000 + (sequence - 1) * 1000000` UTC Unix nanoseconds; receive
time is 100000 ns later. These are scenario times, not live arrival observations.
The ID `synthetic:{seed}:{sequence}` identifies an event within generator v1.
Future adapters must not treat it as a global exchange or client-order identity.

Prices use `PriceTicks`, quantities use `QuantityLots`, and quote observations
use checked u64 arithmetic. One tick is 1/100 synthetic quote units. One lot is
1/1000 synthetic base units. One quote atom is 1/100000 synthetic quote units.
`trade_quote_atoms = price_ticks * quantity_lots`. No ledger uses binary floats.
At the maximum count, the generated cumulative value is bounded by
`10205 * 25 * 1000000 = 255125000000`, below u64 capacity. Checked multiplication
and addition remain in the observation path.

The Rust observer checks ordered, uncrossed positive books, positive quantities,
trade price within the top of book and receive time after event time. It emits
one `market-observation-receipt/v1` per event. The receipt links the exact event
line hash and cumulative observed trade quote atoms. These are observations of
synthetic market trades, **not** orders, fills, balances, P&L or risk reservations.

Serde writes struct fields in the declared order as compact UTF-8 JSON followed
by one LF per record. SHA-256 covers those bytes, including each LF. Cargo.lock
pins dependencies. Hash stability covers the same generator, schema, seed,
count and encoding; it is not a promise across arbitrary future versions.

## Completion, source and rights

The program exclusively creates the output directory and each file. It streams
events/receipts with bounded memory, flushes and syncs them, then saves a manifest.
An I/O or metadata failure leaves incomplete residue for diagnosis. There is no
automatic retry or cleanup. A consumer requires a parseable complete manifest
and independently matched file hashes; presence of a file alone is not completion.
The program does not claim crash-safe directory publication or an operational
transaction ledger. Those require separate P1 controls.

`scenario_hash` and `config_hash` bind compact scenario JSON without a newline.
`dataset_hash` binds actual event JSONL bytes. `policy_hash` is null because
there is no policy/model. Event and receipt hashes exclude wall-clock timestamps.
The manifest includes observed execution times, OS/architecture and package
version. `source_sha` is the checkout HEAD read at execution time. `source_dirty`
records uncommitted source. Neither alone proves which bytes built a binary.
`build_inputs` embeds digests of the Cargo files, toolchain file and Rust sources
at compilation. The Python observer compares them with the inspected checkout.

Rights are independent records for code, data, model and trace. Synthetic data
and observation traces contain no personal/provider data. The repository has
no selected license; commercial use and redistribution remain `UNKNOWN` for
code/data/trace. Model rights are `NOT_APPLICABLE`. The lockfile records dependency
revisions/checksums; the verification report records package license declarations.
Declarations do not establish a complete legal approval or license the dataset.

## Selected controls and acceptance boundary

Soodles at `784f922c569e728bfdd34ed0f9b7bb2e2b7e21e2` supplies the original
`issue_admission.parse_contract` and `test_manager.target_scope`. They validated
the real Issue body and fixed external binding. Test Manager returned `ready`,
the three commands in README, `next=null`, and `authorizes_landing=false`.
This is Source preparation under the current user authorization, not a Native
Noodle admission or a provider landing receipt. No Soodles control plane is copied.

The golden expected values were calculated independently before comparing actual
Rust output. Seed 42 with three events must give:

| Observable | Expected value |
| --- | --- |
| Event JSONL SHA-256 | `189b4982d2f0b9ef2eb849378ba13f66522349764682d4173ebafd15b600b9ba` |
| Receipt JSONL SHA-256 | `989040b289eaff2642a25d05dd28ea195200b928deba4568460e972b0f3d8eba` |
| Cumulative trade quote atoms | `240844` |

The Rust integration tests invoke the actual executable. They cover the golden
bytes, numeric/option refusals, help, missing parent and collision preservation.
The Python observer separately hashes and parses actual files. It checks seeds
0, 1, 42 and u64::MAX, identical fresh 64-event runs, a changed seed and the
release build's maximum-seed vector. It recomputes integer products, cumulative
values and event/receipt links instead of trusting producer-reported hashes.

Planted negatives remove the manifest or final newline, cross the book, move
receive time before event time, change a receipt's quote value, or replace an
integer scale with a float. The latter
mutations rebind file hashes; consistent self-reported hashes must not conceal
invalid semantics. Each control must fail for its named reason.

PASS covers these Source/process/file behaviors only. Native admission, native
Agent behavior, Schema Manager behavior feedback, whole-P0 acceptance, P1 and
P2 remain separate requirements. The Python uv workspace and financial execution
Oracle are not implemented by this slice. Hosted CI is `NOT_RUN` until its real
exact-head result is read back. The workflow runs these same scoped commands.
