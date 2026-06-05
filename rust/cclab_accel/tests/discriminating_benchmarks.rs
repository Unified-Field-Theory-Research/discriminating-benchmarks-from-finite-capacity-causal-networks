use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, paper14_skeleton_marker, DBM001UpstreamBinding, DBM002FiniteBenchmarkRecord,
    DBM003TargetComparatorRegimeDescriptors, DBM004OutcomeUncertaintyAuditDescriptors,
    Paper14SkeletonCertificate, PAPER13_FINAL_CERTIFICATE, PAPER13_FORMAL_ENDPOINT,
    PAPER13_FROZEN_COMMIT,
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
fn dbm002_defines_finite_nonpromoting_benchmark_record() {
    let record = DBM002FiniteBenchmarkRecord::canonical();
    assert!(record.closes_dbm002());
    assert!(record.paper13_compatibility_referenced_only);
    assert!(record.no_benchmark_recovery_claim);
    assert!(record.no_prediction_success_claim);
    assert!(record.no_falsification_success_claim);
    assert!(record.no_physical_promotion_claim);
    assert!(record.no_physical_validation_claim);
    assert!(record.no_empirical_adequacy_claim);
    assert!(record.no_observed_catalog_recovery_claim);
    assert!(record.no_simulation_only_promotion);
    assert!(record.no_fit_only_calibration_claim);
    assert!(record.no_unified_field_theory_claim);
}

#[test]
fn dbm002_skeleton_still_keeps_paper14_theorem_open() {
    let skeleton = Paper14SkeletonCertificate::through_dbm002();
    assert!(skeleton.dbm001_upstream_binding_closed);
    assert!(skeleton.dbm002_finite_benchmark_record_closed);
    assert!(!skeleton.dbm003_target_comparator_regime_closed);
    assert!(!skeleton.dbm008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper14_theorem());
}

#[test]
fn dbm003_defines_finite_target_comparator_regime_descriptors() {
    let descriptors = DBM003TargetComparatorRegimeDescriptors::canonical();
    assert!(descriptors.closes_dbm003());
    assert!(descriptors.benchmark_record.closes_dbm002());
    assert!(descriptors.target_descriptor_is_finite);
    assert!(descriptors.comparator_descriptor_is_finite);
    assert!(descriptors.regime_descriptor_is_finite);
    assert!(descriptors.descriptor_schema_is_bounded);
    assert!(descriptors.descriptor_provenance_audit_visible);
    assert!(descriptors.paper13_compatibility_referenced_only);
    assert!(descriptors
        .claim_boundary
        .all_physical_and_benchmark_claims_remain_false());
}

#[test]
fn dbm003_skeleton_still_keeps_paper14_theorem_open() {
    let skeleton = Paper14SkeletonCertificate::through_dbm003();
    assert!(skeleton.dbm001_upstream_binding_closed);
    assert!(skeleton.dbm002_finite_benchmark_record_closed);
    assert!(skeleton.dbm003_target_comparator_regime_closed);
    assert!(!skeleton.dbm004_outcome_uncertainty_audit_closed);
    assert!(!skeleton.dbm008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper14_theorem());
}

#[test]
fn dbm004_defines_finite_outcome_uncertainty_audit_descriptors() {
    let descriptors = DBM004OutcomeUncertaintyAuditDescriptors::canonical();
    assert!(descriptors.closes_dbm004());
    assert!(descriptors.descriptor_set.closes_dbm003());
    assert!(descriptors.outcome_descriptor_is_finite);
    assert!(descriptors.uncertainty_descriptor_is_finite);
    assert!(descriptors.audit_status_descriptor_is_finite);
    assert!(descriptors.bounded_outcome_schema);
    assert!(descriptors.uncertainty_provenance_audit_visible);
    assert!(descriptors.outcome_is_not_benchmark_recovery);
    assert!(descriptors.outcome_is_not_prediction_success);
    assert!(descriptors.outcome_is_not_falsification_success);
    assert!(descriptors.audit_status_blocks_physical_promotion);
    assert!(descriptors.paper13_compatibility_referenced_only);
}

#[test]
fn dbm004_skeleton_still_keeps_paper14_theorem_open() {
    let skeleton = Paper14SkeletonCertificate::through_dbm004();
    assert!(skeleton.dbm001_upstream_binding_closed);
    assert!(skeleton.dbm002_finite_benchmark_record_closed);
    assert!(skeleton.dbm003_target_comparator_regime_closed);
    assert!(skeleton.dbm004_outcome_uncertainty_audit_closed);
    assert!(!skeleton.dbm005_paper13_intake_compatibility_closed);
    assert!(!skeleton.dbm008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper14_theorem());
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
fn docs_keep_dbm005_active_and_physical_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let config = read_repo_file("GPD/config.json");
    let paper_config = read_repo_file(
        "GPD/publication/discriminating-benchmarks-theorem/manuscript/PAPER-CONFIG.json",
    );
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/discriminating_benchmarks_theorem.md");

    assert_eq!(active_obligation(), "DBM-005");
    assert!(state.contains("\"active_obligation\": \"DBM-005\""));
    assert!(config.contains("\"active_obligation\": \"DBM-005\""));
    assert!(paper_config.contains("\"active_obligation\": \"DBM-005\""));
    assert!(state.contains("\"dbm002_finite_benchmark_record_closed\": true"));
    assert!(state.contains("\"dbm003_target_comparator_regime_closed\": true"));
    assert!(state.contains("\"dbm004_outcome_uncertainty_audit_closed\": true"));
    assert!(state.contains("\"discriminating_benchmarks_theorem_closed\": false"));
    assert!(state.contains("\"benchmark_recovery_claim\": false"));
    assert!(state.contains("\"prediction_success_claim\": false"));
    assert!(state.contains("\"falsification_success_claim\": false"));
    assert!(state.contains("\"physical_promotion_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state.contains("\"empirical_adequacy_claim\": false"));
    assert!(state_md.contains("discriminating benchmarks theorem is not closed."));
    assert!(theorem.contains("DBM-005"));
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
