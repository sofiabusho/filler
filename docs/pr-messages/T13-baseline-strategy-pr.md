# PR Implementation Report: T13

## Summary

Implemented baseline move selection: among valid placements, pick the anchor that maximizes new territory (filled piece cells landing on empty Anfield cells). Added pure `count_new_territory` in `placement.rs` and updated `choose_move` to score via `max_by` with deterministic `(x, y)` tie-breaking. Partial REQ-9 foundation for win-rate tuning in T20–T22.

## Key Changes

- **src/placement.rs**: `count_new_territory(anfield, piece, x, y)` counts empty cells covered; unit test for scoring.
- **src/strategy.rs**: `choose_move` selects valid placement with highest territory gain; tests for maximize-gain, fallback, and tie-break.

## Technical Decisions

- **Scoring in placement module**: `count_new_territory` lives alongside overlap counting as pure, testable placement logic (SDS §2.3 “validate and score”).
- **Primary heuristic only**: T13 scope is maximize new cells on `.`; opponent blocking and map-specific tuning remain T20–T22.
- **Tie-break**: When gains are equal, prefer smaller `(x, y)` so behavior is deterministic and brief-fixture output stays `(7, 2)`.

## Verification Results

### Automated Checks

- [x] `cargo test` passes (28 tests total: 27 lib + 1 integration smoke)
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] `cargo build --release` succeeds

### Manual Audit (against `docs/audit.md`)

- N/A — T13 has no AUD IDs; win-rate gates (AUD-4–AUD-6) are T20–T22.

### Requirements Traceability

- [x] **REQ-9 (partial)**: Baseline strategy maximizes territory gain per move (`ST-1` in PRD §5.3); full 4/5 win gates deferred to T20–T22.

## Artifacts

- **Test output**: `cargo test` — 28 passed, 0 failed
- **Lint output**: clean

---

## Next Steps

T20 — Beat `wall_e` on `map00` (5-run battery, AUD-4)
