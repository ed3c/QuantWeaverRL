use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "qwr-replay-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).expect("new isolated test directory");
        Self(path)
    }

    fn run(&self, seed: &str, events: &str) -> Output {
        Command::new(env!("CARGO_BIN_EXE_quantweaver-replay"))
            .args(["--seed", seed, "--events", events, "--output"])
            .arg(self.0.join("run"))
            .output()
            .expect("run replay process")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove this test's own fixture");
    }
}

#[test]
fn seeded_process_matches_independent_golden_bytes() {
    let fixture = Fixture::new();
    let output = fixture.run("42", "3");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let events = fs::read(fixture.0.join("run/events.jsonl")).unwrap();
    let receipts = fs::read(fixture.0.join("run/receipts.jsonl")).unwrap();
    assert_eq!(
        quantweaver_replay::sha256(&events),
        "189b4982d2f0b9ef2eb849378ba13f66522349764682d4173ebafd15b600b9ba"
    );
    assert_eq!(
        quantweaver_replay::sha256(&receipts),
        "989040b289eaff2642a25d05dd28ea195200b928deba4568460e972b0f3d8eba"
    );
    let manifest: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(manifest["summary"]["cumulative_trade_quote_atoms"], 240844);
    assert_eq!(
        manifest["receipt_semantics"],
        "MARKET_OBSERVATION_ONLY_NOT_ORDER_FILL"
    );
}

#[test]
fn invalid_numeric_inputs_leave_no_output() {
    for (seed, count, diagnostic) in [
        ("-1", "3", "unsigned 64-bit"),
        ("18446744073709551616", "3", "unsigned 64-bit"),
        ("42", "0", "1..=1000000"),
        ("42", "1000001", "1..=1000000"),
        ("42", "bad", "unsigned integer"),
    ] {
        let fixture = Fixture::new();
        let output = fixture.run(seed, count);
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains(diagnostic));
        assert!(!fixture.0.join("run").exists());
    }
}

#[test]
fn malformed_options_fail_and_help_has_no_effect() {
    for args in [
        vec![],
        vec!["--seed"],
        vec!["--seed", "42"],
        vec!["--unknown", "42"],
        vec!["--seed", "42", "--seed", "1"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_quantweaver-replay"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("Usage:"));
    }
    let help = Command::new(env!("CARGO_BIN_EXE_quantweaver-replay"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).starts_with("Usage: quantweaver-replay"));
}

#[test]
fn collision_refuses_without_changing_original_evidence() {
    let fixture = Fixture::new();
    assert!(fixture.run("42", "3").status.success());
    let files = ["events.jsonl", "receipts.jsonl", "manifest.json"];
    let before: Vec<_> = files
        .iter()
        .map(|name| fs::read(fixture.0.join("run").join(name)).unwrap())
        .collect();
    let repeated = fixture.run("43", "3");
    assert_eq!(repeated.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&repeated.stderr).contains("never overwritten"));
    let after: Vec<_> = files
        .iter()
        .map(|name| fs::read(fixture.0.join("run").join(name)).unwrap())
        .collect();
    assert_eq!(after, before);
}

#[test]
fn missing_parent_fails_before_creating_output() {
    let fixture = Fixture::new();
    let missing_parent = fixture.0.join("missing/run");
    let output = Command::new(env!("CARGO_BIN_EXE_quantweaver-replay"))
        .args(["--seed", "42", "--events", "3", "--output"])
        .arg(&missing_parent)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("output must be a new directory"));
    assert!(!missing_parent.exists());
}
