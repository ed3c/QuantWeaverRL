use quantweaver_replay::{Result, Scenario, invalid, replay, sha256};
use serde::Serialize;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const USAGE: &str =
    "Usage: quantweaver-replay --seed <u64> --events <1..1000000> --output <new-directory>";

fn arguments(args: Vec<OsString>) -> Result<(u64, u32, PathBuf)> {
    let mut seed = None;
    let mut events = None;
    let mut output = None;
    let mut args = args.into_iter();
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| invalid("each option requires a value"))?;
        match flag.to_str() {
            Some("--seed") if seed.is_none() => {
                seed = Some(
                    value
                        .to_str()
                        .ok_or_else(|| invalid("seed must be UTF-8"))?
                        .parse::<u64>()
                        .map_err(|_| invalid("seed must be an unsigned 64-bit integer"))?,
                );
            }
            Some("--events") if events.is_none() => {
                events = Some(
                    value
                        .to_str()
                        .ok_or_else(|| invalid("events must be UTF-8"))?
                        .parse::<u32>()
                        .map_err(|_| {
                            invalid("events must be an unsigned integer in 1..=1000000")
                        })?,
                );
            }
            Some("--output") if output.is_none() => output = Some(PathBuf::from(value)),
            _ => return Err(invalid("unknown or duplicate option").into()),
        }
    }
    Ok((
        seed.ok_or_else(|| invalid("missing --seed"))?,
        events.ok_or_else(|| invalid("missing --events"))?,
        output.ok_or_else(|| invalid("missing --output"))?,
    ))
}

#[derive(Serialize)]
struct SourceFile {
    path: &'static str,
    sha256: String,
}

fn build_inputs() -> Vec<SourceFile> {
    [
        (
            "Cargo.toml",
            include_bytes!("../../../Cargo.toml").as_slice(),
        ),
        (
            "Cargo.lock",
            include_bytes!("../../../Cargo.lock").as_slice(),
        ),
        (
            "rust-toolchain.toml",
            include_bytes!("../../../rust-toolchain.toml").as_slice(),
        ),
        (
            "crates/replay/Cargo.toml",
            include_bytes!("../Cargo.toml").as_slice(),
        ),
        (
            "crates/replay/src/lib.rs",
            include_bytes!("lib.rs").as_slice(),
        ),
        (
            "crates/replay/src/main.rs",
            include_bytes!("main.rs").as_slice(),
        ),
    ]
    .into_iter()
    .map(|(path, bytes)| SourceFile {
        path,
        sha256: sha256(bytes),
    })
    .collect()
}

fn git_readback(root: &Path, args: &[&str]) -> Result<String> {
    let result = Command::new("git").args(args).current_dir(root).output()?;
    if !result.status.success() {
        return Err(invalid("source Git readback failed; no completed manifest").into());
    }
    Ok(String::from_utf8(result.stdout)?.trim().to_owned())
}

#[derive(Serialize)]
struct Rights {
    category: &'static str,
    owner: &'static str,
    source: &'static str,
    revision: &'static str,
    checksum: Option<String>,
    license: &'static str,
    commercial_use: &'static str,
    redistribution: &'static str,
    pii_status: &'static str,
    content_visibility: &'static str,
    first_seen_at_unix_ns: u64,
}

fn rights(
    category: &'static str,
    source: &'static str,
    revision: &'static str,
    checksum: Option<String>,
    observed_at: u64,
) -> Rights {
    let model = category == "model";
    Rights {
        category,
        owner: if model {
            "NOT_APPLICABLE"
        } else {
            "QuantWeaverRL; rights not adjudicated"
        },
        source,
        revision,
        checksum,
        license: if model {
            "NOT_APPLICABLE"
        } else {
            "UNSPECIFIED"
        },
        commercial_use: if model { "NOT_APPLICABLE" } else { "UNKNOWN" },
        redistribution: if model { "NOT_APPLICABLE" } else { "UNKNOWN" },
        pii_status: if category == "data" || category == "trace" {
            "SYNTHETIC_NO_PERSONAL_DATA"
        } else {
            "NOT_ASSESSED"
        },
        content_visibility: "LOCAL_RUN",
        first_seen_at_unix_ns: observed_at,
    }
}

#[derive(Serialize)]
struct Manifest {
    schema_version: &'static str,
    status: &'static str,
    receipt_semantics: &'static str,
    scenario: Scenario,
    scenario_hash: String,
    config_hash: String,
    dataset_hash: String,
    policy_hash: Option<String>,
    summary: quantweaver_replay::ReplaySummary,
    source_sha: String,
    source_dirty: bool,
    build_inputs: Vec<SourceFile>,
    runtime: String,
    started_at_unix_ns: u64,
    completed_at_unix_ns: u64,
    rights: Vec<Rights>,
}

fn timestamp() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_nanos()
        .try_into()?)
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args == [OsString::from("--help")] {
        println!("{USAGE}");
        return Ok(());
    }
    let (seed, count, output) = arguments(args)?;
    let scenario = Scenario::new(seed, count)?;
    let started = timestamp()?;
    fs::create_dir(&output).map_err(|error| {
        std::io::Error::new(
            error.kind(),
            format!(
                "output must be a new directory; existing evidence is never overwritten: {error}"
            ),
        )
    })?;
    let mut events = BufWriter::new(File::create_new(output.join("events.jsonl"))?);
    let mut receipts = BufWriter::new(File::create_new(output.join("receipts.jsonl"))?);
    let summary = replay(seed, count, &mut events, &mut receipts)?;
    events.flush()?;
    receipts.flush()?;
    events.get_ref().sync_all()?;
    receipts.get_ref().sync_all()?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| invalid("repository build path is unavailable"))?;
    let source_sha = git_readback(root, &["rev-parse", "HEAD"])?;
    let source_dirty =
        !git_readback(root, &["status", "--porcelain", "--untracked-files=all"])?.is_empty();
    let inputs = build_inputs();
    let input_hash = sha256(&serde_json::to_vec(&inputs)?);
    let scenario_hash = sha256(&serde_json::to_vec(&scenario)?);
    let rights = vec![
        rights(
            "code",
            "repository build inputs; repository license not selected",
            "build-inputs/v1",
            Some(input_hash),
            started,
        ),
        rights(
            "data",
            "self-generated synthetic market events",
            "synthetic-market-event/v1",
            Some(summary.event_sha256.clone()),
            started,
        ),
        rights(
            "model",
            "no model weights or inference",
            "NOT_APPLICABLE",
            None,
            started,
        ),
        rights(
            "trace",
            "self-generated market observation receipts",
            "market-observation-receipt/v1",
            Some(summary.receipt_sha256.clone()),
            started,
        ),
    ];
    let manifest = Manifest {
        schema_version: "synthetic-replay-manifest/v1",
        status: "COMPLETE",
        receipt_semantics: "MARKET_OBSERVATION_ONLY_NOT_ORDER_FILL",
        scenario,
        config_hash: scenario_hash.clone(),
        scenario_hash,
        dataset_hash: summary.event_sha256.clone(),
        policy_hash: None,
        summary,
        source_sha,
        source_dirty,
        build_inputs: inputs,
        runtime: format!(
            "rust/{}/{}; quantweaver-replay/{}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            env!("CARGO_PKG_VERSION")
        ),
        started_at_unix_ns: started,
        completed_at_unix_ns: timestamp()?,
        rights,
    };
    let bytes = serde_json::to_vec_pretty(&manifest)?;
    let mut file = File::create_new(output.join("manifest.json"))?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", String::from_utf8(bytes)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("quantweaver-replay: {error}\n{USAGE}");
        std::process::exit(2);
    }
}
