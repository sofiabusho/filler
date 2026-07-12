# PR Implementation Report: T21

## Summary

Generalized win-rate strategy for larger maps: replaced hardcoded map00 spawn column (`x=9`) with dynamic territory-center tie-breaking, expansion scoring, horizontal/vertical advance toward the opponent, and border touches. **AUD-5 passed: 5/5** wins vs `h2_d2` on `map01`. Map00 regression vs `wall_e` also **5/5**.

## Key Changes

- **src/strategy.rs**: Map-agnostic `EvalContext` and lexicographic `placement_rank` (gain → expansion → Voronoi → foe Voronoi → territory column → vertical/horizontal advance → border → proximity → span). Instant-win check retained.
- **src/strategy.rs**: Added `map00_spawn_column_stays_centered` test (dynamic center resolves to column 9 on symmetric map00).

## Technical Decisions

- **Dynamic territory center**: `(min_x + max_x) / 2` of own territory replaces fixed `SPAWN_COLUMN=9`, which pulled map01 P1 (x≈4) and P2 (x≈33) toward the wrong column.
- **Expansion + diagonal advance**: On 40×30 `map01`, P1/P2 start in opposite corners; `expansion_bonus` and horizontal advance (P1 → +x, P2 → −x) drive toward the center and cut off `h2_d2`.
- **Lexicographic tuple rank**: Single `placement_rank` tuple avoids clippy `too_many_arguments` while preserving cascading tie-break order.

## Verification Results

### Automated Checks

- [x] `cargo test` passes (27 tests)
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] `cargo build --release` succeeds

### Manual Audit (against `docs/audit.md`)

- [x] **AUD-5**: Pass — **5/5** wins vs `h2_d2` on `map01` (mandated battery below).

### Win-rate battery

| Run | Student side | Opponent | Map | Seed | Result |
|-----|--------------|----------|-----|------|--------|
| 1 | p1 | h2_d2 | map01 | 1 | W (949–92) |
| 2 | p2 | h2_d2 | map01 | 2 | W (145–891) |
| 3 | p1 | h2_d2 | map01 | 3 | W (864–168) |
| 4 | p2 | h2_d2 | map01 | 4 | W (117–920) |
| 5 | p1 | h2_d2 | map01 | 5 | W (976–121) |

Map00 regression (vs `wall_e`, seeds 1–5): **5/5**.

### Requirements Traceability

- [x] **REQ-9 (map01)**: AUD-5 gate met for `h2_d2` / `map01`.

## Artifacts

- **Engine command**: `cd docker_image && ./linux_game_engine -q -s <seed> -f maps/map01 -p1 ../solution/filler -p2 linux_robots/h2_d2` (swap p1/p2 on even runs)
- **Test output**: 27 passed
- **Lint output**: clean

---

## Next Steps

1. **T22** — Beat `bender` on `map02` (AUD-6).
