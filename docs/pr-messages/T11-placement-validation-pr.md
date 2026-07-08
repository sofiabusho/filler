# PR Implementation Report: T11

## Summary

Implemented pure placement validation: bounds checking, opponent overlap detection, exactly-one own-territory overlap rule, `count_own_overlaps`, and `iter_valid_placements`. Eight unit tests cover valid/invalid cases per REQ-4–REQ-6, REQ-11, REQ-12, AUD-9, and AUD-10.

## Key Changes

- **src/placement.rs**: `validate_placement`, `count_own_overlaps`, `iter_valid_placements`; table-driven tests for overlap, bounds, and opponent rejection.
- **src/model.rs**: Added `Piece::is_filled(x, y)` helper for mask lookups during validation.

## Technical Decisions

- **Bounds checked on filled cells only**: Empty cells in the piece mask are ignored when testing in-bounds placement, matching engine behavior for sparse shapes.
- **Error precedence**: Out-of-bounds is reported before overlap errors; opponent overlap is reported before wrong own-overlap count.
- **Player-relative cells**: Validation uses pre-classified `Cell` values from parsing; `PlayerId` is retained in the API for SDS compatibility and future use.

## Verification Results

### Automated Checks

- [x] `cargo test` passes (19 tests total: 8 new placement tests + 11 existing)
- [x] `cargo clippy` passes (`-D warnings`)
- [x] `cargo fmt --check` passes
- [x] `cargo build --release` succeeds

### Manual Audit (against `docs/audit.md`)

- [x] **AUD-9**: Pass — tests accept exactly-one-own-overlap, reject zero/two+ own overlaps, and reject opponent overlap.
- [x] **AUD-10**: Pass — tests reject negative anchors and pieces extending past board edges.

### Requirements Traceability

- [x] **REQ-4**: `validate_placement` accepts only when `count_own_overlaps == 1`.
- [x] **REQ-5**: `has_opponent_overlap` rejects placements touching `Foe`/`FoeLast` cells.
- [x] **REQ-6**: `piece_fits_in_bounds` ensures all filled piece cells lie within the Anfield grid.
- [x] **REQ-11**: Eight placement validation tests cover valid and invalid coordinate cases.
- [x] **REQ-12**: Boundary tests reject negative anchors and off-board piece extension.

## Artifacts

- **Test output**: `cargo test` — 19 passed, 0 failed
- **Lint output**: clean

---

## Next Steps

T12 — IO loop + `X Y\n` output serializer and `0 0` fallback
