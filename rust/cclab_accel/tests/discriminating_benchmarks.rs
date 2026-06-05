use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, paper14_skeleton_marker, DBM001UpstreamBinding, Paper14SkeletonCertificate,
    PAPER13_FINAL_CERTIFICATE, PAPER13_FORMAL_ENDPOINT, PAPER13_FROZEN_COMMIT,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|err| {
        panic!("failed to read {path}: {err}");
    })
}

#[test]
fn required_scaffold_files_exist() {
    for path in [
        "README.md",
        "UPSTREAM-PAPERS.json",
        "GPD/STATE.md",
        "GPD/ROADMAP.md",
        "GPD/state.json",
        "docs/discriminating_benchmarks_theorem.md",
        "docs/open_proof_obligations.md",
        "GPD/formal/FiniteCapacity/DiscriminatingBenchmarks.lean",
    ] {
        assert!(
            repo_root().join(path).exists(),
            "missing required file: {path}"
        );
    }
}

#[test]
fn dbm001_consumes_closed_paper13_chain_without_physical_claims() {
    let binding = DBM001UpstreamBinding::canonical();
    assert!(binding.closes_dbm001());
    assert_eq!(binding.paper13_frozen_commit, PAPER13_FROZEN_COMMIT);
    assert_eq!(binding.paper13_formal_endpoint, PAPER13_FORMAL_ENDPOINT);
    assert_eq!(binding.paper13_final_certificate, PAPER13_FINAL_CERTIFICATE);
    assert!(binding
        .claim_boundary
        .all_physical_and_benchmark_claims_remain_false());
}

#[test]
fn initial_skeleton_keeps_paper14_theorem_open() {
    let skeleton = Paper14SkeletonCertificate::initial_dbm001_only();
    assert!(skeleton.dbm001_upstream_binding_closed);
    assert!(!skeleton.dbm002_finite_benchmark_record_closed);
    assert!(!skeleton.dbm008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper14_theorem());
    assert_eq!(
        paper14_skeleton_marker(),
        "paper14-discriminating-benchmarks-dbm001-nonpromoting-skeleton"
    );
}

#[test]
fn upstream_json_records_paper13_certificate_and_nonpromotion() {
    let upstream = read_repo_file("UPSTREAM-PAPERS.json");
    assert!(upstream.contains(PAPER13_FROZEN_COMMIT));
    assert!(upstream.contains(PAPER13_FORMAL_ENDPOINT));
    assert!(upstream.contains(PAPER13_FINAL_CERTIFICATE));
    assert!(upstream.contains(
        "\"paper13_conditional_external_physical_evidence_intake_theorem_closed\": true"
    ));
    assert!(upstream.contains("\"paper13_physical_promotion_claim\": false"));
    assert!(upstream.contains("\"unified_field_theory_claim\": false"));
}

#[test]
fn docs_keep_dbm002_active_and_physical_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/discriminating_benchmarks_theorem.md");

    assert_eq!(active_obligation(), "DBM-002");
    assert!(state.contains("\"active_obligation\": \"DBM-002\""));
    assert!(state.contains("\"discriminating_benchmarks_theorem_closed\": false"));
    assert!(state.contains("\"benchmark_recovery_claim\": false"));
    assert!(state.contains("\"prediction_success_claim\": false"));
    assert!(state.contains("\"falsification_success_claim\": false"));
    assert!(state.contains("\"physical_promotion_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state.contains("\"empirical_adequacy_claim\": false"));
    assert!(
        state_md.contains("The local Paper 14 discriminating benchmarks theorem is not closed.")
    );
    assert!(theorem.contains("DBM-002"));
    assert!(theorem.contains("no unified field theory claim"));
}

#[test]
fn repository_contains_no_python_artifacts() {
    let forbidden = [
        "pyproject.toml",
        "requirements.txt",
        "setup.py",
        "setup.cfg",
    ];
    for path in forbidden {
        assert!(
            !repo_root().join(path).exists(),
            "unexpected Python artifact: {path}"
        );
    }
}
