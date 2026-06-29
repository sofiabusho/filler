# Requirements — filler

> Derived from `docs/raw/REQUIREMENTS-SOURCE.md` (01-edu Filler brief).  
> Treat as READ-ONLY once approved. Changes require stakeholder review.

## 1. Context

**filler** is an 01-edu school project: an algorithmic territory-capture game where two robots compete on a 2D grid called the **Anfield**. Each turn the provided `game_engine` sends the current board and a random piece; the student robot must respond with placement coordinates on standard output. The player who occupies the largest surface wins.

The `game_engine`, maps, reference robots, and Docker image are supplied externally (see REQ-8). The student deliverable is a compiled robot binary plus unit tests, run inside the Docker container against school opponents.

Reference brief: [`docs/raw/REQUIREMENTS-SOURCE.md`](raw/REQUIREMENTS-SOURCE.md).

## 2. Objectives

- Build a **stdin/stdout robot** that parses engine input and outputs valid `X Y\n` placements each turn.
- Implement **correct placement rules**: exactly one overlap with own territory, no opponent overlap, in-bounds pieces.
- **Win** against school robots (`wall_e`, `h2_d2`, `bender`) on designated maps (audit thresholds).
- Ship **unit tests** covering parsing, placement validation, boundaries, and output formatting.

## 3. Functional Requirements

### Game protocol

#### REQ-1: Read engine input from stdin

Each turn the `game_engine` writes the Anfield state and the piece to place to the robot's standard input. The robot must read and parse this stream reliably across the full match.

The first line identifies the player:

```text
$$$ exec p<number> : [<player path>]
```

#### REQ-2: Write placement coordinates to stdout

The robot must output placement coordinates in the exact format `X Y\n` (column then row, newline-terminated).

#### REQ-3: Player identity and symbols

- **Player 1** is represented on the board by `a` (last placed piece) and `@` (older territory).
- **Player 2** is represented by `s` (last placed) and `$` (older territory).
- `.` is empty; opponent symbols must be detected and avoided.

### Placement rules

#### REQ-4: Exactly one territory overlap

A valid placement requires **one and only one** cell of the piece to overlap a cell of the robot's existing territory.

#### REQ-5: No opponent overlap

The piece must not overlap any opponent cell. Overlapping opponent territory invalidates the move (engine ignores it; game continues for the other player).

#### REQ-6: In-bounds placement

The entire piece must fit inside the Anfield rows and columns. Partial off-board placements are invalid.

#### REQ-7: No valid move fallback

When no valid placement exists, the robot must still emit a response (reference robots use `0 0\n`). The match may continue for the opponent.

### Winning and opponents

#### REQ-8: Docker-based game engine

Development and audit runs use the supplied Docker image:

1. Build: `docker build -t filler .` (inside provided `docker_image` folder).
2. Run: `docker run -v "$(pwd)/solution":/filler/solution -it filler`.
3. Student binary lives in `solution/` (mounted at `/filler/solution` in the container).

`game_engine` flags include `-f` (map), `-p1`, `-p2`, `-q`, `-r`, `-s`, `-t` (timeout, default 10s).

#### REQ-9: Beat school robots (mandatory)

The student player must win **at least 4 of 5** games (alternating p1/p2) against:

| Map | Opponent |
|-----|----------|
| `maps/map00` | `robots/wall_e` |
| `maps/map01` | `robots/h2_d2` |
| `maps/map02` | `robots/bender` |

Winning means occupying more Anfield surface when the game ends. `terminator` is explicitly out of mandatory scope (bonus only).

### Unit tests

#### REQ-10: Input parsing tests

Tests must verify Anfield dimensions/grid and piece shapes are parsed correctly from representative stdin strings.

#### REQ-11: Placement validation tests

Tests must verify a candidate coordinate is accepted only when there is **exactly one** overlap with own territory and **zero** overlaps with the opponent.

#### REQ-12: Boundary tests

Tests must ensure placements extending outside the grid are rejected.

#### REQ-13: Output format tests

Tests must verify the robot formats answers as `X Y\n` exactly as required by the engine.

### Bonus (optional, non-blocking)

#### REQ-B1: Graphic visualizer

Create a visualizer for matches or board state.

#### REQ-B2: Beat terminator

Create a player that wins **at least 4 of 5** games against `robots/terminator` on a chosen map.

## 4. Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| NFR-1 | Implementation language: **Rust** (edition 2021) |
| NFR-2 | Robot is a **native compiled binary** invoked by `game_engine` via path (`-p1` / `-p2`) |
| NFR-3 | **Docker** is mandatory for running matches against the official engine |
| NFR-4 | Robot must respond within engine **timeout** (default 10s per move) — no hangs, segfaults, or memory faults |
| NFR-5 | Code follows 01-edu **good practices**; auditor can build, test, and run without undocumented steps |
| NFR-6 | Unit tests runnable on the host via `cargo test` without Docker |

## 5. Constraints

- Use the **official `game_engine`** and maps from the provided Docker image — do not reimplement the engine.
- **Stdin/stdout protocol** is fixed; coordinate format must be literal `X Y\n`.
- Timeout or crash **loses** the entire game for that player.
- Territory overlap rule is strict: **exactly one** connecting cell, not zero and not two or more.

## 6. Out of Scope

- Beating `robots/terminator` (mandatory gate — bonus only).
- Graphical visualizer (bonus).
- Modifying `game_engine`, maps, or reference robots.
- Networked or multiplayer beyond the two-process engine model.
- Predicting future piece shapes (pieces are random per turn).

## 7. Documentation Requirements

- README must explain Docker setup, building the robot, running `game_engine` matches, and `cargo test`.
- `docs/audit.md` must be executable by an auditor without guessing commands or pass criteria.

## Cross-References

| Document | Relationship |
|----------|--------------|
| `docs/PRD.md` | Expands these requirements into detailed product spec |
| `docs/audit.md` | Maps acceptance checks back to REQ IDs |
| `docs/ticket-tracker.md` | Tracks implementation coverage per REQ ID |
| `docs/raw/REQUIREMENTS-SOURCE.md` | Authoritative stakeholder source text |
| `AGENTS.md` | Agent coding guidelines aligned with these requirements |
