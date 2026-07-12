# PR Implementation Report: T22

## Summary

Tuned strategy for the 100×99 `map02` diagonal spawns vs `bender`: bounded placement search near own territory, two-phase ranking (light heuristics → Voronoi refine on top 16), and map-agnostic `toward_opponent_bias` instead of fixed horizontal advance. **AUD-6 passed: 4/5** wins vs `bender` on `map02`. Map00/map01 regressions remain **5/5**.

## Key Changes

- **src/placement.rs**: `iter_valid_placements_near_own()` — search only bbox around own territory ± piece size (avoids full-grid scan on ~9900 cells).
- **src/strategy.rs**: Large-map path (`>2500` cells): candidate cap 64, instant-win scan on top gain moves, `light_rank` pre-filter, `large_map_rank` with Voronoi on top 16 only.
- **src/strategy.rs**: `toward_opponent_bias()` — advance direction derived from own vs foe territory centers (fixes map02 P1 needing left/up expansion).
- **src/strategy.rs**: Light rank adds proximity-to-foe and reachable-empty delta; column bias dropped on large maps.

## Technical Decisions

- **Performance**: Full-grid Voronoi per candidate caused >10s/turn timeouts on map02; bounded search + Voronoi only on finalists keeps games finishable (~6 min/battery).
- **Two-phase rank**: Phase 1 (`light_rank`) sorts 64 candidates cheaply; phase 2 applies Voronoi/foe-Voronoi on top 16 for territory control without evaluating every placement.
- **Diagonal maps**: Fixed P1 → +x/+y advance (map01) is wrong when P1 spawns bottom-right; center-to-center bias generalizes across map geometries.

## Verification Results

### Automated Checks

- [x] `cargo test` passes (27 tests)
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] `cargo build --release` succeeds

### Manual Audit (against `docs/audit.md`)

- [x] **AUD-6**: Pass — **4/5** wins vs `bender` on `map02` (mandated battery below).

### Win-rate battery

| Run | Student side | Opponent | Map | Seed | Result |
|-----|--------------|----------|-----|------|--------|
| 1 | p1 | bender | map02 | 1 | W (4723–3543) |
| 2 | p2 | bender | map02 | 2 | W (3972–4434) |
| 3 | p1 | bender | map02 | 3 | L (3951–4085) |
| 4 | p2 | bender | map02 | 4 | W (4185–4225) |
| 5 | p1 | bender | map02 | 5 | W (7643–848) |

Map00 regression (vs `wall_e`, seeds 1–5): **5/5**.  
Map01 regression (vs `h2_d2`, seeds 1–5): **5/5**.

### Requirements Traceability

- [x] **REQ-9 (map02)**: AUD-6 gate met for `bender` / `map02`.

## Artifacts

- **Engine command**: `cd docker_image && ./linux_game_engine -q -s <seed> -f maps/map02 -p1 ../solution/filler -p2 linux_robots/bender` (swap p1/p2 on even runs)
- **Build**: `CARGO_TARGET_DIR=/home/someuan/filler/target cargo build --release && cp target/release/filler solution/filler`
- **Test output**: 27 passed
- **Lint output**: clean

---

## Next Steps

1. **T30** — Audit dry-run (AUD-1–AUD-13 evidence).
