# Audit Gate — filler

> Acceptance checklist for 01-edu Filler.  
> Derived from `docs/raw/AUDIT-SOURCE.md`. Each auditor question maps to one AUD ID.

**How to use**: Set up Docker per README. Build the student robot. Walk sections in order. Mark ✅ / ❌ / ⬜.

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Pass |
| ❌ | Fail |
| ⬜ | Not yet verified |
| N/A | Not applicable |

---

## 1. Functional — Docker and runtime

#### AUD-1: Docker image and container work

- **Covers**: REQ-8
- **Verify**:
  1. Inside the provided `docker_image` folder: `docker build -t filler .`
  2. `docker run -v "$(pwd)/solution":/filler/solution -it filler`
  3. In the container: `./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator`
  4. **Pass** if the engine runs without setup errors.

#### AUD-2: Student player runs against an opponent

- **Covers**: REQ-1, REQ-2, REQ-8
- **Verify**:
  1. Build the student robot binary in `solution/`.
  2. Run e.g. `./game_engine -f maps/map01 -p1 solution/<student_player> -p2 robots/bender`
  3. **Pass** if the match completes without crash/timeout from the student player.

#### AUD-3: Correct single-cell territory overlap

- **Covers**: REQ-4, REQ-5
- **Verify**:
  1. Run the student player against a reference robot on a small map.
  2. Observe accepted moves in engine output (or replay logs).
  3. **Pass** if placed pieces connect to own territory with **exactly one** overlapping cell and never overlap opponent cells on accepted moves.

---

## 2. Functional — Win rate gates

Run **five** games per gate, **alternating** student as p1 and p2. Student must win **≥ 4**.

#### AUD-4: Beat `wall_e` on `map00`

- **Covers**: REQ-9
- **Verify**:
  1. `./game_engine -f maps/map00 -p1 solution/<player> -p2 robots/wall_e` (and swapped p1/p2 across runs)
  2. **Pass** if student wins **≥ 4 / 5**.

#### AUD-5: Beat `h2_d2` on `map01`

- **Covers**: REQ-9
- **Verify**:
  1. `./game_engine -f maps/map01 -p1 solution/<player> -p2 robots/h2_d2` (alternate sides)
  2. **Pass** if student wins **≥ 4 / 5**.

#### AUD-6: Beat `bender` on `map02`

- **Covers**: REQ-9
- **Verify**:
  1. `./game_engine -f maps/map02 -p1 solution/<player> -p2 robots/bender` (alternate sides)
  2. **Pass** if student wins **≥ 4 / 5**.

---

## 3. Unit tests

#### AUD-7: All tests pass

- **Covers**: REQ-10–REQ-13, NFR-6
- **Verify**:
  1. On the host (or in container if documented): `cargo test`
  2. **Pass** if the full suite exits 0 with no errors.

#### AUD-8: Input parsing tests exist

- **Covers**: REQ-10
- **Verify**:
  1. Inspect test sources for Anfield + piece parsing from stdin strings.
  2. **Pass** if tests assert correct grid dimensions, cell symbols, and piece shape parsing.

#### AUD-9: Placement validation tests exist

- **Covers**: REQ-11
- **Verify**:
  1. Inspect tests for valid/invalid coordinates.
  2. **Pass** if tests reject two-or-more own overlaps, opponent overlap, and accept exactly-one-own-overlap cases.

#### AUD-10: Boundary detection tests exist

- **Covers**: REQ-12
- **Verify**:
  1. Inspect tests for off-board piece placements.
  2. **Pass** if placements partially outside rows/columns are rejected.

#### AUD-11: Coordinate output format tests exist

- **Covers**: REQ-13
- **Verify**:
  1. Inspect tests for stdout formatting.
  2. **Pass** if output is verified as `X Y\n` (space-separated integers, trailing newline).

---

## 4. Quality and standards

#### AUD-12: Good practices

- **Covers**: NFR-5
- **Verify**:
  1. Review code structure, naming, error handling, and absence of obvious dead code.
  2. **Pass** if code aligns with 01-edu good practices and `AGENTS.md`.

#### AUD-13: Dedicated test coverage

- **Covers**: REQ-10–REQ-13
- **Verify**:
  1. Confirm a test module/file exists for parsing, placement, boundaries, and output.
  2. **Pass** if tests cover the main success and failure cases for each area (not only happy path).

---

## 5. Bonus (non-blocking)

#### AUD-B1: Graphic visualizer

- **Covers**: REQ-B1
- **Verify**:
  1. Run or demo the visualizer against a recorded or live game.
  2. **Pass** if Anfield state and piece placements are shown graphically.

#### AUD-B2: Beat `terminator`

- **Covers**: REQ-B2
- **Verify**:
  1. Five games on a chosen map, alternating p1/p2 vs `robots/terminator`.
  2. **Pass** if student wins **≥ 4 / 5**.

---

## Cross-References

| Document | Relationship |
|----------|--------------|
| `docs/requirements.md` | Source REQ IDs audited here |
| `docs/PRD.md` | Detailed spec behind each check |
| `docs/ticket-tracker.md` | Maps tickets → AUD IDs |
| `docs/raw/AUDIT-SOURCE.md` | Authoritative auditor question source |
| `docs/pr-messages/` | Evidence of audit verification per ticket |
