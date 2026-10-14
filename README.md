# QuantWeaverRL

A paper-only quantitative trading and Agent RL laboratory. The current executable
is a finite, offline Rust synthetic market replay. The backend, paper-order risk
engine, data lake, Agent environments and trainers remain future phase work.

```sh
cargo run --locked -p quantweaver-replay -- --seed 42 --events 64 --output /tmp/qwr-replay-example
```

The output directory must not exist; its parent must exist. The program writes
`events.jsonl`, `receipts.jsonl` and then `manifest.json`. Receipts record market
observations and exact trade-value arithmetic. They do not represent order fills.
Same seed and count give identical event and receipt bytes, including newlines.
The manifest separately records execution times and source readback, which can vary.

Run the controls selected by Soodles Test Manager for [Issue #3](https://github.com/ed3c/QuantWeaverRL/issues/3):

```sh
cargo fmt --all -- --check
cargo test --locked -p quantweaver-replay
python3 scripts/verify_replay.py
```

The independent Python standard-library observer reads actual files, checks four
golden seeds, repeats a workload, changes its seed, compares debug/release at the
maximum u64 seed and rejects six planted corruptions. No Python package or uv
workspace is needed by this observer. To retain its evidence outside a disposable
worktree, set `QWR_EVIDENCE_DIR` to a new directory under an existing parent.

See [replay contract](docs/p0-replay.md), [handoff](docs/handoff.md) and
[phase plan](docs/phases.md). Passing this slice does not accept the whole P0 phase.
No repository license has been selected. The manifest records separate code,
data, model and trace rights; it does not infer commercial or redistribution rights.
