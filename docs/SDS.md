# Software Design Specification (SDS) — filler

> **Project:** filler  
> **Date:** 2026-06-30  
> **Purpose:** Technical implementation guide for the Filler stdin/stdout robot.  
> **Note:** Reference alongside `AGENTS.md`, `docs/PRD.md`, `docs/requirements.md`, and `docs/audit.md`.

---

## 1. Architecture Overview

```text
main.rs
  └── GameLoop
        ├── parse_turn(stdin) → Turn { player, anfield, piece }
        ├── strategy::best_move(&anfield, &piece) → Option<(i32,i32)>
        └── io::emit_move(x, y) → stdout

src/
  main.rs       # loop until EOF; handle first-turn exec line
  parse.rs      # tokenize engine text protocol
  model.rs      # Anfield, Piece, PlayerId, Cell
  placement.rs  # validity, overlap counts, apply (for sim/tests)
  strategy.rs   # scoring heuristics, move ordering
```

### 1.1 Design Principles

- **Pure placement logic** — `placement.rs` has no I/O; fully unit-testable (REQ-10–REQ-12).
- **Parse vs decide** — parsing never chooses moves; strategy never reads raw strings.
- **Fail safe on EOF** — exit cleanly when stdin closes; never panic on bad engine data in production path.
- **Deterministic tests** — fixtures are static strings copied from the brief.

---

## 2. Module Specifications

### 2.1 `parse`

**Responsibility**: Convert engine text into structured `Turn` data.

**Public API**:

```rust
pub struct Turn {
    pub player: PlayerId,
    pub anfield: Anfield,
    pub piece: Piece,
}

pub fn parse_turn(input: &str) -> Result<Turn, ParseError>;
pub fn parse_exec_line(line: &str) -> Result<PlayerId, ParseError>;
```

**Usage example**:

```rust
let turn = parse_turn(ENGINE_SAMPLE)?;
assert_eq!(turn.anfield.width, 20);
assert_eq!(turn.piece.cells().count(), 4);
```

### 2.2 `model`

**Responsibility**: Grid representation and player symbol mapping.

**Public API**:

```rust
pub enum PlayerId { P1, P2 }

pub enum Cell { Empty, OwnLast, Own, FoeLast, Foe }

pub struct Anfield {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>, // row-major
}

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub mask: Vec<bool>, // row-major, true = filled (#)
}
```

**Symbol mapping** (REQ-3):

| Char | Meaning for P1 | Meaning for P2 |
|------|----------------|----------------|
| `.` | Empty | Empty |
| `@` | Own territory | — |
| `a` | Last piece | — |
| `$` | — | Own territory |
| `s` | — | Last piece |
| Other | Foe | Foe |

### 2.3 `placement`

**Responsibility**: Validate and score candidate anchor positions `(x, y)` — top-left of piece bounding box on the Anfield coordinate system used by the engine.

**Public API**:

```rust
pub enum PlacementError {
    OutOfBounds,
    OpponentOverlap,
    WrongOverlapCount { got: u32 },
}

pub fn validate_placement(
    anfield: &Anfield,
    piece: &Piece,
    player: PlayerId,
    x: i32,
    y: i32,
) -> Result<(), PlacementError>;

pub fn count_own_overlaps(...) -> u32;

pub fn iter_valid_placements(
    anfield: &Anfield,
    piece: &Piece,
    player: PlayerId,
) -> impl Iterator<Item = (i32, i32)>;
```

### 2.4 `strategy`

**Responsibility**: Select among valid placements to maximize territory and meet win-rate goals.

**Public API**:

```rust
pub fn choose_move(
    anfield: &Anfield,
    piece: &Piece,
    player: PlayerId,
) -> (i32, i32); // returns fallback (0,0) when none valid (REQ-7)
```

**Heuristic sketch** (tunable per ticket):

- Primary: maximize new cells claimed (`#` cells landing on `.`).
- Secondary: minimize opponent accessible frontier.
- Tertiary: prefer central / cutting placements on large maps.

### 2.5 `main` / IO

**Responsibility**: Buffered line reading, turn loop, stdout formatting.

```rust
fn write_move(x: i32, y: i32) {
    println!("{x} {y}");
}
```

---

## 3. Data Models

### 3.1 Engine turn (canonical example from brief)

**Input** (player 1):

```text
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
...
Piece 4 1:
.OO.
```

**Output**:

```text
7 2
```

Note: piece rows use `.` and `O` (or `#` in other examples); parser must treat any non-`.` cell in the piece block as filled.

### 3.2 Coordinates

- `X` = column index (horizontal, matches header digit row).
- `Y` = row index (vertical, matches `000`, `001`, … prefix).
- Anchor `(x, y)` is the top-left of the piece's bounding box aligned to those indices.

---

## 4. Critical Structures

### 4.1 Stdout move line

```text
{X} {Y}\n
```

**Required behaviors**:

- Single space between integers.
- Unix newline `\n` only (no extra spaces or text).
- Flush after each turn so the engine does not block.

### 4.2 Anfield row format

```text
{row_idx} {row_content}
```

Row index is 3-digit zero-padded; row content length equals `width` from header.

---

## 5. Integration Points

| Integration | Protocol | Notes |
|-------------|----------|-------|
| `game_engine` | stdin/stdout text | Player subprocess; timeout 10s default |
| Docker mount | filesystem | Host `solution/` → `/filler/solution` |
| Reference robots | engine `-p1`/`-p2` paths | `robots/bender`, etc. inside image |

**Example match commands** (inside container):

```bash
./game_engine -f maps/map00 -p1 /filler/solution/filler -p2 robots/wall_e
./game_engine -f maps/map01 -p1 robots/h2_d2 -p2 /filler/solution/filler
./game_engine -s 42 -t 15 -f maps/map02 -p1 /filler/solution/filler -p2 robots/bender
```

---

## 6. Error Handling

- **Parse errors**: return `Result`; in `main`, log to stderr and exit non-zero only if no valid game state possible on first turn.
- **No valid placement**: emit `0 0\n` (REQ-7); do not panic.
- **Timeout prevention**: avoid O(width × height × piece_area) per turn on large maps without pruning — use `iter_valid_placements` early exit when good enough.

---

## 7. Testing Notes

- **Unit tests** (`#[cfg(test)]` in `parse`, `placement`, and a small `io` helper):
  - Parsing: brief sample + edge cases (minimal 1×1 map, large piece header).
  - Placement: exactly-one overlap, zero overlap, two+ overlaps, opponent overlap, out of bounds.
  - Output: `format_move(7, 2) == "7 2\n"`.
- **Integration tests** (`tests/`): optional scripted stdin strings through a `play_turn` test harness.
- **Manual / Docker**: win-rate batteries for AUD-4–AUD-6; log seeds and p1/p2 order in PR messages.

---

## Cross-References

| Document | Relationship |
|----------|--------------|
| `docs/PRD.md` | Product-level requirements this spec implements |
| `docs/audit.md` | Structural and behavioral checks derived from this SDS |
| `AGENTS.md` | Coding standards agents follow when implementing this spec |
