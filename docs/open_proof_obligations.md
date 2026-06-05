# Open Proof Obligations

## Active

### DBM-006

Define benchmark stability and coarse-graining behavior as finite,
benchmark-local, auditable, non-promoting maps.

Acceptance criteria:

- stability descriptors are finite and explicit
- coarse-graining descriptors are finite and explicit
- stability is benchmark-local and does not assert validation
- coarse-graining behavior does not assert physical promotion
- every stability row carries non-promotion and non-validation flags
- Rust guards reject benchmark recovery, prediction success, falsification
  success, physical validation, empirical adequacy, observed-catalog recovery,
  simulation-only promotion, fit-only calibration, physical nature
  realization, and unified-field promotion
- Lean scaffold records that `DBM-006` is required before the full Paper 14
  theorem can close

## Closed

- `DBM-001`: Upstream binding and claim-boundary scaffold.
- `DBM-002`: Finite discriminating benchmark records.
- `DBM-003`: Finite benchmark target, comparator, and regime descriptors.
- `DBM-004`: Finite benchmark outcome, uncertainty, and audit-status
  descriptors.
- `DBM-005`: Paper 13 evidence-intake compatibility without validation.
