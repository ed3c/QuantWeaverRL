#!/usr/bin/env python3
"""Observe real replay files independently with Python's standard library."""
import hashlib
import json
import os
import platform
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
GOLDENS = {
    0: ("a043f56bb5e62c1438c54221f789bfe4ec2ba9708473f811ea84775e0d7cefa1",
        "c5c067f2b3f6604410efcd6ea3e97fc827a87cfd81fdeca6b8a331985352b161", 261686),
    1: ("ee9e76f67be2ebc73e9accea1f3730460fe0689d1ac025fff2c2cc56d7b1f447",
        "5c5a1a8d6413bb50e045ca4dda3712a5238b7cbc758bf18312c918dad7bd8be9", 312047),
    42: ("189b4982d2f0b9ef2eb849378ba13f66522349764682d4173ebafd15b600b9ba",
        "989040b289eaff2642a25d05dd28ea195200b928deba4568460e972b0f3d8eba", 240844),
    2**64 - 1: ("2f807d05ece1171c73e9b839f89e84782dd663ae934e7e95fc76803e4d6d010f",
               "501929bdc6ca551c01c858d17e5e17cbd96aa1f5e45eb24235cb34c41a6d0f4e", 362058),
}
BUILD_PATHS = ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "crates/replay/Cargo.toml",
               "crates/replay/src/lib.rs", "crates/replay/src/main.rs"]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def compact(value):
    return json.dumps(value, separators=(",", ":")).encode()


def unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load(raw):
    return json.loads(raw, object_pairs_hook=unique)


def command(argv):
    started = time.monotonic()
    run = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=180)
    require(run.returncode == 0, f"command failed {argv}: {run.stderr}")
    return run.stdout, {"argv": [str(v) for v in argv], "exit_status": run.returncode,
                        "elapsed_seconds": time.monotonic() - started}


def observe(directory):
    manifest = load((directory / "manifest.json").read_bytes())
    require(manifest["schema_version"] == "synthetic-replay-manifest/v1"
            and manifest["status"] == "COMPLETE", "incomplete replay manifest")
    require(manifest["receipt_semantics"] == "MARKET_OBSERVATION_ONLY_NOT_ORDER_FILL",
            "receipt claims unsupported order semantics")
    scenario = manifest["scenario"]
    numeric = ("seed", "events", "start_time_ns", "interval_ns", "receive_latency_ns",
               "price_tick_denominator", "quantity_lot_denominator", "quote_atom_denominator")
    require(all(type(scenario[field]) is int for field in numeric), "scenario requires integer units")
    seed, count = scenario["seed"], scenario["events"]
    require(type(seed) is int and 0 <= seed < 2**64 and type(count) is int
            and 1 <= count <= 1_000_000, "invalid scenario bounds")
    require(scenario == {
        "schema_version": "synthetic-scenario/v1", "generator": "splitmix64-l2-trade/v1",
        "seed": seed, "events": count, "start_time_ns": 1_700_000_000_000_000_000,
        "interval_ns": 1_000_000, "receive_latency_ns": 100_000,
        "price_tick_denominator": 100, "quantity_lot_denominator": 1_000,
        "quote_atom_denominator": 100_000,
    }, "scenario schema or units changed")
    require(manifest["scenario_hash"] == manifest["config_hash"] == sha(compact(scenario)),
            "scenario hash does not bind exact config")
    raw_events = (directory / "events.jsonl").read_bytes()
    raw_receipts = (directory / "receipts.jsonl").read_bytes()
    require(raw_events.endswith(b"\n") and raw_receipts.endswith(b"\n"), "missing JSONL newline")
    events = raw_events.splitlines(keepends=True)
    receipts = raw_receipts.splitlines(keepends=True)
    require(len(events) == len(receipts) == count, "event/receipt count mismatch")
    total = 0
    for sequence, (event_bytes, receipt_bytes) in enumerate(zip(events, receipts), 1):
        event, receipt = load(event_bytes), load(receipt_bytes)
        require(set(event) == {"schema_version", "event_id", "sequence", "event_time_ns",
                              "receive_time_ns", "symbol", "bids", "asks", "trade"},
                "event schema mismatch")
        require(event["schema_version"] == "synthetic-market-event/v1"
                and event["symbol"] == "SYNBASE/SYNQUOTE"
                and event["event_id"] == f"synthetic:{seed}:{sequence}"
                and type(event["sequence"]) is int and event["sequence"] == sequence,
                "event identity or sequence mismatch")
        timestamp = 1_700_000_000_000_000_000 + (sequence - 1) * 1_000_000
        require(type(event["event_time_ns"]) is int and event["event_time_ns"] == timestamp
                and type(event["receive_time_ns"]) is int
                and event["receive_time_ns"] == timestamp + 100_000, "invalid visibility time")
        bids, asks, trade = event["bids"], event["asks"], event["trade"]
        require(len(bids) == len(asks) == 2, "book must contain two levels per side")
        for level in [*bids, *asks, trade]:
            require(set(level) == {"price_ticks", "quantity_lots"}
                    and all(type(value) is int and 0 < value < 2**64 for value in level.values()),
                    "invalid integer tick/lot amount")
        require(bids[0]["price_ticks"] > bids[1]["price_ticks"]
                and asks[0]["price_ticks"] < asks[1]["price_ticks"]
                and bids[0]["price_ticks"] < asks[0]["price_ticks"], "invalid or crossed book")
        require(bids[0]["price_ticks"] <= trade["price_ticks"] <= asks[0]["price_ticks"],
                "trade outside synthetic top of book")
        quote = trade["price_ticks"] * trade["quantity_lots"]
        total += quote
        require(total < 2**64, "quote accumulation overflow")
        require(receipt == {
            "schema_version": "market-observation-receipt/v1", "kind": "market-observation",
            "event_id": event["event_id"], "sequence": sequence,
            "observation_time_ns": event["receive_time_ns"], "event_sha256": sha(event_bytes),
            "trade_quote_atoms": quote, "cumulative_trade_quote_atoms": total,
        } and all(type(receipt[field]) is int for field in
                  ("sequence", "observation_time_ns", "trade_quote_atoms", "cumulative_trade_quote_atoms")),
                "receipt link, exact arithmetic or semantics mismatch")
    event_hash, receipt_hash = sha(raw_events), sha(raw_receipts)
    require(type(manifest["summary"]["events"]) is int
            and type(manifest["summary"]["cumulative_trade_quote_atoms"]) is int,
            "summary requires integer amounts")
    require(manifest["summary"] == {"events": count, "event_sha256": event_hash,
            "receipt_sha256": receipt_hash, "cumulative_trade_quote_atoms": total},
            "manifest does not match actual file bytes")
    require(manifest["dataset_hash"] == event_hash and manifest["policy_hash"] is None,
            "dataset/model provenance mismatch")
    inputs = manifest["build_inputs"]
    require([item["path"] for item in inputs] == BUILD_PATHS, "missing build input identity")
    require(all(item == {"path": path, "sha256": sha((ROOT / path).read_bytes())}
                for item, path in zip(inputs, BUILD_PATHS)), "binary build input differs from source")
    head, _ = command(["git", "rev-parse", "HEAD"])
    dirty, _ = command(["git", "status", "--porcelain", "--untracked-files=all"])
    require(re.fullmatch(r"[0-9a-f]{40}", manifest["source_sha"])
            and manifest["source_sha"] == head.strip()
            and type(manifest["source_dirty"]) is bool
            and manifest["source_dirty"] == bool(dirty.strip()), "source checkout readback mismatch")
    started, ended = manifest["started_at_unix_ns"], manifest["completed_at_unix_ns"]
    require(type(started) is int and type(ended) is int and 0 < started <= ended,
            "invalid observed execution timestamps")
    require(manifest["runtime"].startswith("rust/") and "quantweaver-replay/0.1.0" in manifest["runtime"],
            "missing runtime identity")
    rights = manifest["rights"]
    require([item["category"] for item in rights] == ["code", "data", "model", "trace"],
            "missing separate rights category")
    expected_hashes = [sha(compact(inputs)), event_hash, None, receipt_hash]
    for item, checksum in zip(rights, expected_hashes):
        model = item["category"] == "model"
        require(set(item) == {"category", "owner", "source", "revision", "checksum", "license",
            "commercial_use", "redistribution", "pii_status", "content_visibility", "first_seen_at_unix_ns"}
            and all(item[field] for field in ("owner", "source", "revision", "pii_status")),
            "rights schema incomplete")
        require(item["checksum"] == checksum and type(item["first_seen_at_unix_ns"]) is int
                and item["first_seen_at_unix_ns"] == started
                and item["content_visibility"] == "LOCAL_RUN", "rights source binding mismatch")
        require(item["license"] == ("NOT_APPLICABLE" if model else "UNSPECIFIED")
                and item["commercial_use"] == item["redistribution"]
                == ("NOT_APPLICABLE" if model else "UNKNOWN"), "unsupported rights claim")
    return {"seed": seed, "events": count, "event_sha256": event_hash,
            "receipt_sha256": receipt_hash, "cumulative_trade_quote_atoms": total}


def rebind_mutation(directory):
    manifest = load((directory / "manifest.json").read_bytes())
    events = (directory / "events.jsonl").read_bytes().splitlines(keepends=True)
    receipts = [load(line) for line in (directory / "receipts.jsonl").read_bytes().splitlines()]
    for event_bytes, receipt in zip(events, receipts):
        receipt["event_sha256"] = sha(event_bytes)
    raw_receipts = b"".join(compact(row) + b"\n" for row in receipts)
    (directory / "receipts.jsonl").write_bytes(raw_receipts)
    event_hash = sha(b"".join(events))
    manifest["dataset_hash"] = manifest["summary"]["event_sha256"] = event_hash
    manifest["summary"]["receipt_sha256"] = sha(raw_receipts)
    manifest["rights"][1]["checksum"] = event_hash
    manifest["rights"][3]["checksum"] = sha(raw_receipts)
    (directory / "manifest.json").write_bytes(compact(manifest))


def verify(evidence):
    processes = []
    for args in (["cargo", "build", "--locked", "-p", "quantweaver-replay"],
                 ["cargo", "build", "--release", "--locked", "-p", "quantweaver-replay"]):
        _, process = command(args)
        processes.append(process)
    observations = []

    def run(name, seed, count, mode="debug"):
        directory = evidence / name
        stdout, process = command([ROOT / f"target/{mode}/quantweaver-replay", "--seed", str(seed),
                                    "--events", str(count), "--output", directory])
        processes.append(process)
        require(load(stdout) == load((directory / "manifest.json").read_bytes()),
                "stdout differs from saved manifest")
        result = observe(directory)
        observations.append({"name": name, **result})
        return result

    first = run("repeat-a", 42, 64)
    second = run("repeat-b", 42, 64)
    different = run("different-seed", 43, 64)
    require(first == second, "same seed replay differs")
    require(first["event_sha256"] != different["event_sha256"]
            and first["receipt_sha256"] != different["receipt_sha256"], "seed does not affect replay")
    for seed, expected in GOLDENS.items():
        result = run(f"golden-{seed}", seed, 3)
        require((result["event_sha256"], result["receipt_sha256"],
                 result["cumulative_trade_quote_atoms"]) == expected, "literal golden vector mismatch")
    release = run("release-max-seed", 2**64 - 1, 3, "release")
    require((release["event_sha256"], release["receipt_sha256"],
             release["cumulative_trade_quote_atoms"]) == GOLDENS[2**64 - 1],
            "debug/release wrapping differs")
    negatives = []
    for name, diagnostic in [("missing-manifest", "manifest.json"), ("missing-newline", "newline"),
                              ("crossed-book", "crossed book"), ("future-visibility", "visibility time"),
                              ("wrong-quote", "exact arithmetic"), ("float-scale", "integer units")]:
        directory = evidence / name
        shutil.copytree(evidence / "repeat-a", directory)
        if name == "missing-manifest":
            (directory / "manifest.json").unlink()
        elif name == "float-scale":
            path = directory / "manifest.json"
            manifest = load(path.read_bytes())
            manifest["scenario"]["price_tick_denominator"] = 100.0
            manifest["scenario_hash"] = manifest["config_hash"] = sha(compact(manifest["scenario"]))
            path.write_bytes(compact(manifest))
        elif name == "missing-newline":
            path = directory / "events.jsonl"
            path.write_bytes(path.read_bytes()[:-1])
            rebind_mutation(directory)
        elif name == "wrong-quote":
            path = directory / "receipts.jsonl"
            rows = [load(line) for line in path.read_bytes().splitlines()]
            rows[0]["trade_quote_atoms"] += 1
            path.write_bytes(b"".join(compact(row) + b"\n" for row in rows))
            rebind_mutation(directory)
        else:
            path = directory / "events.jsonl"
            rows = [load(line) for line in path.read_bytes().splitlines()]
            if name == "crossed-book":
                rows[0]["bids"][0]["price_ticks"] = rows[0]["asks"][0]["price_ticks"]
            else:
                rows[0]["receive_time_ns"] = rows[0]["event_time_ns"] - 1
            path.write_bytes(b"".join(compact(row) + b"\n" for row in rows))
            rebind_mutation(directory)
        try:
            observe(directory)
        except (ValueError, FileNotFoundError) as error:
            require(diagnostic in str(error), f"negative rejected for wrong reason: {error}")
            negatives.append({"case": name, "result": "PASS", "observed_refusal": str(error)})
        else:
            raise ValueError(f"observer accepted planted negative {name}")
    metadata, _ = command(["cargo", "metadata", "--locked", "--format-version", "1"])
    dependencies = [{"name": item["name"], "version": item["version"], "license": item["license"],
                     "source": item["source"]} for item in load(metadata)["packages"]]
    source_sha, _ = command(["git", "rev-parse", "HEAD"])
    rustc, _ = command(["rustc", "--version", "--verbose"])
    cargo, _ = command(["cargo", "--version"])
    report = {"result": "PASS", "test_scope": "P0 Source replay only", "observer": "Python stdlib file/arithmetic observer",
              "source_sha": source_sha.strip(), "observer_sha256": sha(Path(__file__).read_bytes()),
              "runtime_readback": {"rustc": rustc.strip(), "cargo": cargo.strip(),
                                   "python": platform.python_version(), "platform": platform.platform()},
              "source_dirty": bool(command(["git", "status", "--porcelain", "--untracked-files=all"])[0].strip()),
              "observations": observations, "negative_controls": negatives, "processes": processes,
              "dependencies": dependencies, "native_admission": "NOT_RUN", "native_agent_behavior": "NOT_RUN",
              "whole_p0_acceptance": "NOT_RUN", "authorizes_landing": False}
    (evidence / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    selected = os.environ.get("QWR_EVIDENCE_DIR")
    if selected:
        directory = Path(selected).resolve()
        directory.mkdir()
        verify(directory)
    else:
        with tempfile.TemporaryDirectory(prefix="qwr-replay-observer-") as temporary:
            verify(Path(temporary))
