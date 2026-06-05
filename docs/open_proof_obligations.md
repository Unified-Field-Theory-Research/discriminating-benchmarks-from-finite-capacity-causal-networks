# Open Proof Obligations

## Active

### DBM-002

Define finite discriminating benchmark records as bounded, auditable,
non-promoting interface rows.

Acceptance criteria:

- the record schema is finite and explicit
- benchmark identifiers, target labels, comparator labels, regime labels,
  observable labels, expected-output descriptors, and audit-status descriptors
  are bounded
- every row carries non-promotion and non-validation flags
- Paper 13 evidence-intake compatibility is only referenced, not promoted
- Rust guards reject benchmark recovery, prediction success, falsification
  success, physical validation, empirical adequacy, observed-catalog recovery,
  simulation-only promotion, fit-only calibration, physical nature
  realization, and unified-field promotion
- Lean scaffold records that `DBM-002` is required before the full Paper 14
  theorem can close

## Closed

- `DBM-001`: Upstream binding and claim-boundary scaffold.
