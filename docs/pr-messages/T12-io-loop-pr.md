# PR Implementation Report: T12

## Summary

Implemented the stdin game loop, stdout move emission, and multi-turn protocol handling. The robot reads engine turns from stdin, selects a move via `strategy::choose_move` (first valid placement or `0 0` fallback), and writes `X Y\n` to stdout with flush. Satisfies REQ-2, REQ-7, REQ-13, AUD-2 (local smoke), AUD-3 (via validated move selection), and AUD-11.

## Key Changes

- **src/game.rs**: `run_game` stdin loop with incremental turn buffering; handles first-turn exec line and continuation turns; three integration-style unit tests.
- **src/io.rs**: `write_move` serializer using `format_move`, with flush; output format unit tests.
- **src/main.rs**: Wires `BufReader` stdin and stdout into `run_game`.
- **src/parse.rs**: Added `parse_turn_continuation` for turns after the initial exec line.
- **src/strategy.rs**: `choose_move` returns first valid placement from `iter_valid_placements` or `FALLBACK_MOVE` (REQ-7); T13 will replace first-valid with territory-maximizing heuristics.
- **src/lib.rs**: Exported `game` and `io` modules.

## Technical Decisions

- **Incremental turn detection**: Buffer stdin lines until `parse_turn` / `parse_turn_continuation` succeeds; treat `UnexpectedEof` as incomplete turn and keep reading.
- **Testable game loop**: `run_game<R, W>` accepts generic reader/writer so output format and fallback behavior are verified without subprocesses.
- **Minimal move selection**: First valid anchor from `iter_valid_placements` wires the IO loop to placement validation; scoring heuristics remain T13 scope.
- **Output path**: `io::write_move` delegates formatting to `model::format_move` (existing REQ-13 tests) and flushes after each turn per SDS §4.1.

## Verification Results

### Automated Checks

- [x] `cargo test` passes (26 tests total: 25 lib + 1 integration smoke)
- [x] `cargo clippy` passes (`-D warnings`)
- [x] `cargo fmt --check` passes
- [x] `cargo build --release` succeeds

### Manual Audit (against `docs/audit.md`)

- [x] **AUD-2**: Pass (local smoke) — release binary piped brief fixture stdin; emitted `7 2\n`, exit 0, no crash. Full Docker engine run deferred (no `docker_image` in workspace); binary copied to `solution/filler`.
- [x] **AUD-3**: Pass — `run_game_emits_valid_move_for_brief_fixture` confirms output `(7, 2)` which passes `validate_placement`; `choose_move` only returns coordinates from `iter_valid_placements` (exactly-one overlap enforced by T11).
- [x] **AUD-11**: Pass — `io::tests::write_move_*`, `model::format_move_matches_engine_protocol`, and `tests/smoke.rs` verify `X Y\n` format with space separator and trailing newline.

### Requirements Traceability

- [x] **REQ-2**: `io::write_move` emits `"{x} {y}\n"` to stdout each turn via `main` → `run_game`.
- [x] **REQ-7**: `choose_move` returns `(0, 0)` when `iter_valid_placements` is empty; verified in `run_game_emits_fallback_when_no_valid_placement`.
- [x] **REQ-13**: Output serializer tests in `io.rs`, `model.rs`, and `tests/smoke.rs` assert exact `X Y\n` formatting.

## Artifacts

- **Test output**: `cargo test` — 26 passed, 0 failed
- **Smoke test**: `printf '<brief fixture>' | ./target/release/filler` → `7 2`
- **Lint output**: clean

---

## Next Steps

T13 — Baseline strategy (maximize new territory per move)
