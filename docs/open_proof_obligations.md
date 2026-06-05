# Open Proof Obligations

## Active

### DBM-004

Define finite benchmark outcome, uncertainty, and audit-status descriptors as
bounded, auditable, non-promoting descriptor rows.

Acceptance criteria:

- outcome descriptors are finite and explicit
- uncertainty descriptors are finite and explicit
- audit-status descriptors are finite and explicit
- outcome rows do not assert benchmark recovery or prediction success
- every descriptor row carries non-promotion and non-validation flags
- Paper 13 evidence-intake compatibility is only referenced, not promoted
- Rust guards reject benchmark recovery, prediction success, falsification
  success, physical validation, empirical adequacy, observed-catalog recovery,
  simulation-only promotion, fit-only calibration, physical nature
  realization, and unified-field promotion
- Lean scaffold records that `DBM-004` is required before the full Paper 14
  theorem can close

## Closed

- `DBM-001`: Upstream binding and claim-boundary scaffold.
- `DBM-002`: Finite discriminating benchmark records.
- `DBM-003`: Finite benchmark target, comparator, and regime descriptors.
