# Data ownership and Agent RL lakehouse (proposed)

Status: DESIGN ONLY. The existing repo contains no deployed lake or database. Prefer the simplest reproducible path on MacBook M1 Pro 16 GB; avoid introducing all databases at once.

## Independent storage owners

| Data category | Authority / format | Consumers and hard gates |
| --- | --- | --- |
| Account state, intents, risk reservations, order receipts, outbox | **PostgreSQL** operational transactions (start in-memory at P1) | Rust risk/order owner; no double-effective order or over-reservation |
| Raw market feed and paper execution events | Immutable timestamped **Parquet / JSONL** | Rust adapters, replay, independent observer; sequences and market-time visible cutoff |
| Binance/Hyperliquid order books, funding, fees; Taiwan TWSE/MOPS; SEC/OpenDART | Bronze object/files + source license manifest | Simulator/research; each provider has separate license/use/redistribution restrictions |
| Supply-chain disclosures and link graph | Versioned document references + Parquet tables | Qlib/Agent research; `valid_time`, `published_at`, `first_seen_at`, `ingested_at` preserved |
| Event-study features, fundamentals, consensus, herding and risk | Silver/Gold **Parquet**, queried with **DuckDB** | Out-of-sample feature computations; prohibit future leaks |
| Agent episode observations, actions, tool results, outcomes and grader evidence | Immutable JSONL plus tabular Parquet; partition by dataset/date as measured | AgentEnv/Karotte/NeMo Gym adapters; never allow grader tampering |
| Training-grade trajectories | Pinned dataset with token IDs, generated logprobs when available, masks, policy/tokenizer/reward versions | Colab trainer. Missing token evidence is never silently reconstructed as on-policy RL |
| Frozen splits, oracle results, evals, checkpoints/LoRA | Versioned artifact files/object storage and manifest IDs; operational run registry | Test Manager, independent holdout and Soodles feedback; no repeated blind-test contamination |

**Do not use DuckDB as a concurrent transaction ledger.** DuckDB is an analytical reader/producer. Once reproducible snapshots and concurrent data producers are actually needed, evaluate **DuckLake** with a metadata catalog (SQLite local or PostgreSQL for cross-host clients). Apache Iceberg, ClickHouse and large object stores are optional scale experiments, not day-zero requirements.

## Publication and licensing

For each **code, data, model and trace** artifact retain `source, revision, checksum, license, commercial_use, redistribution, first_seen_at, content_visibility, owner, pii_status`. Do not treat a code license as a data license. Prefer synthetic self-generated paper data and redacted traces for publicly redistributable Golden Datasets. Never commit provider API keys, broker accounts, raw protected analyst reports or personally identifying data.

## Ingestion & point-in-time verification

1. Save exact source bytes or immutable file reference and timing from the ingestion producer.
2. Validate sequence continuity, symbol/currency/tick scale, duplicates, UTC and exchange-local session calendars.
3. Keep availability time separate from the historical value's effective period; a revised quarterly report is not earlier public information.
4. Release `Silver` only when schema and source checks pass; otherwise quarantine with a named reason.
5. Materialize `Gold` derived features from the same frozen source snapshot.
6. Create `Training` and blind `Holdout` manifests with disjoint market-time and scenario-family splits, preserving negative controls and failed experiments.

**Important distinction:** data-lake snapshot time does not prove when a market participant first could have known an event.

## Local-to-Colab flow

```text
Rust/Python P0–P5 -> immutable Bronze/Silver/Gold Parquet
    -> DuckDB validated SQL -> frozen manifest + checksums
    -> object/file export for Colab -> SFT or compatible RL training
    -> model candidate + metrics/checkpoint
    -> Mac independent replay/evals -> Soodles feedback
```

Colab is an ephemeral GPU worker; it never owns the persistent ledger or the canonical dataset. Read-only model API outputs (Jev/DeepSeek) do not imply access to token-level online training data.

## Validation metrics

Data completeness, delta/sequence gap rates, event-time vs receive-time lag, duplicate key rate, PIT leakage, license coverage, stable replay hashes, train/test overlap, hidden grader contamination, trajectory mask alignment, artifact retention and cost per valid episode. Compute exact completeness checks in SQL/validator; DuckDB Agent Mode's compact or truncated results are **not** a full-data acceptance receipt.
