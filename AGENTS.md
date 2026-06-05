# Repository Guidelines

## Scope And Purpose

This repository is Paper 14 in the finite-capacity causal-network theorem
ladder. It starts only after the closed Paper 13 external physical evidence
intake certificate and defines the next isolated workspace: discriminating
benchmarks.

Do not treat Paper 14 as a physical validation, prediction, falsification, or
unified field theory claim. The initial rung, `DBM-001`, only records upstream
binding and claim boundaries through the frozen Paper 13 certificate. The
active next rung is `DBM-002`: define finite discriminating benchmark records
without importing physical promotion, physical validation, empirical adequacy,
observed-catalog recovery, prediction success, falsification closure,
simulation-only promotion, or fit-only calibration.

## Project Structure

- `README.md`: repository overview and claim boundary.
- `UPSTREAM-PAPERS.json`: frozen upstream commits and formal endpoints for
  Papers 1-13.
- `GPD/STATE.md`, `GPD/ROADMAP.md`, `GPD/state.json`: active theorem-ladder
  state and next obligation.
- `docs/discriminating_benchmarks_theorem.md`: theorem target and current
  closed/open rungs.
- `docs/open_proof_obligations.md`: active obligation list.
- `docs/proof_log.md`: chronological proof and guard log.
- `GPD/formal/FiniteCapacity/DiscriminatingBenchmarks.lean`: Lean scaffold for
  Paper 14 gates and fail-closed blockers.
- `rust/cclab_accel/src/lib.rs`: Rust audit surface for finite bindings and
  non-promotion flags.
- `rust/cclab_accel/tests/discriminating_benchmarks.rs`: regression guards for
  the Paper 14 skeleton.

## Commands

Run local checks from this repository root:

```bash
make test-fast
make lean-build
```

Use `cargo fmt --all` before committing Rust changes.

## Research Rules

Keep theorem claims conditional and local. Paper 14 may define finite
benchmark records, comparator labels, regime descriptors, audit rows, and
compatibility maps, but it must not claim physical validation, empirical
adequacy, prediction success, falsification of alternatives, or unified-field
promotion unless a later explicit external-evidence and review track closes
those claims.

Do not add Python artifacts. Runtime guards and audits are Rust-only; formal
scaffolding is Lean-only.

## Commit Style

Use concise Conventional Commit-style messages, for example:

- `chore: create paper 14 scaffold`
- `formal: add dbm002 benchmark record contract`
- `test: guard paper 14 claim boundary`
