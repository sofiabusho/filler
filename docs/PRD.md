# Product Requirements Document — filler

> Version 0.1 · Date 2026-06-30 · Status DRAFT

## 1. Executive Summary

**filler** is a Rust stdin/stdout robot for the 01-edu Filler game. Two players take turns placing tetris-like pieces on a grid; each placement must touch existing territory at exactly one cell. The student robot competes against bundled reference AIs inside a Dockerized `game_engine`, aiming to maximize occupied area and win mandated matchups on `map00`–`map02`.

The project must deliver:

1. A compiled robot binary mountable at `solution/` and invokable by `game_engine`.
2. Correct protocol handling (parse turns, emit `X Y\n`, survive no-move situations).
3. Unit-tested parsing, placement validation, boundary checks, and output formatting — plus win-rate evidence against `wall_e`, `h2_d2`, and `bender`.

## 2. Goals and Non-Goals

### 2.1 Goals

| ID | Goal |
|----|------|
| G1 | Protocol-correct robot: parse engine input, output `X Y\n`, no crashes/timeouts |
| G2 | Rule-correct placements: 1-cell territory link, no opponent overlap, in-bounds |
| G3 | Win ≥ 4/5 vs `wall_e` (map00), `h2_d2` (map01), `bender` (map02) alternating sides |
| G4 | Comprehensive unit tests for parsing, placement, boundaries, and output format |

### 2.2 Non-Goals

- Beating `terminator` (bonus stretch goal).
- Reimplementing or patching `game_engine`.
- GUI/visualizer (bonus).
- Online leaderboards or persistent match storage.
- Piece-shape prediction beyond current-turn data.

## 3. User Flows

### 3.1 Auditor — Docker smoke test

```text
docker build -t filler .   # in docker_image/
docker run -v "$(pwd)/solution":/filler/solution -it filler
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
  → Engine runs (AUD-1)
```

### 3.2 Auditor — student player smoke test

```text
cargo build --release   # on host → copy binary to solution/
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender
  → Match completes; student places valid pieces (AUD-2, AUD-3)
```

### 3.3 Auditor — win-rate battery

```text
For each (map, opponent) pair in REQ-9:
  Run 5 games, alternate p1/p2
  Record wins (larger territory at game end)
  → ≥ 4 wins each (AUD-4, AUD-5, AUD-6)
```

### 3.4 Developer — unit tests

```text
cargo test
cargo clippy
cargo fmt --check
```

## 4. Architecture Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                     game_engine (Docker)                     │
│  reads map, spawns two player processes, judges moves        │
└───────────────┬─────────────────────────────┬───────────────┘
                │ stdin: Anfield + Piece      │ stdin: ...
                ▼                             ▼
        ┌───────────────┐             ┌───────────────┐
        │ Student robot │             │ Reference AI  │
        │  (solution/)  │             │   (robots/) │
        └───────┬───────┘             └───────┬───────┘
                │ stdout: "X Y\n"             │
                └──────────────┬──────────────┘
                               ▼
                    Territory scoring → winner
```

**In-process modules (student robot)**:

```text
main loop
  ├── parse::read_turn()        # exec line, Anfield, Piece
  ├── model::Anfield / Piece    # grid + player symbols
  ├── placement::is_valid()     # 1 overlap, no foe, in bounds
  ├── strategy::choose_move()   # heuristic / search
  └── io::write_move(x, y)      # "X Y\n"
```

## 5. Detailed Requirements

### 5.1 Protocol (maps to REQ-1–REQ-3)

| ID | Requirement | Priority |
|----|-------------|----------|
| IO-1 | Parse `$$$ exec pN : [...]` to learn player number | P0 |
| IO-2 | Parse `Anfield H W:` header and row grid with coordinates | P0 |
| IO-3 | Parse `Piece H W:` and `#`/`.` shape rows | P0 |
| IO-4 | Emit `"{x} {y}\n"` per turn; flush stdout | P0 |

### 5.2 Placement engine (maps to REQ-4–REQ-7)

| ID | Requirement | Priority |
|----|-------------|----------|
| PL-1 | Enumerate or search anchor `(x,y)` for top-left of piece bbox | P0 |
| PL-2 | Count overlaps with own `@`/`a` or `$`/`s` — require count == 1 | P0 |
| PL-3 | Reject if any `#` cell overlaps opponent or `.` outside board | P0 |
| PL-4 | If no valid move, output fallback (e.g. `0 0\n`) | P0 |

### 5.3 Strategy (maps to REQ-9)

| ID | Requirement | Priority |
|----|-------------|----------|
| ST-1 | Baseline: maximize territory gain per move | P0 |
| ST-2 | Block or limit opponent expansion when tied on area | P1 |
| ST-3 | Tune per-map heuristics to meet 4/5 win gates | P0 |

### 5.4 Testing (maps to REQ-10–REQ-13)

| ID | Requirement | Priority |
|----|-------------|----------|
| UT-1 | Fixture stdin strings → parsed Anfield/Piece | P0 |
| UT-2 | Table-driven placement valid/invalid cases | P0 |
| UT-3 | Edge maps: 1×1 piece, corners, full-row pieces | P1 |
| UT-4 | Output serializer unit test | P0 |

## 6. Technology Constraints

| Concern | Decision |
|---------|----------|
| Language | Rust (edition 2021) |
| Runtime | Native binary executed by `game_engine` |
| Dependencies | Standard library preferred; minimal crates only if justified in ticket/PR |
| Tooling | `cargo`, `clippy`, `rustfmt`; Docker for engine matches |
| Test runner | `cargo test` on host |

## 7. Repository Structure

```text
filler/
├── src/                    # Robot source
│   ├── main.rs             # stdin/stdout game loop
│   ├── parse.rs            # Engine input parsing
│   ├── model.rs            # Anfield, Piece, Player
│   ├── placement.rs        # Validity + overlap counting
│   └── strategy.rs         # Move selection
├── tests/                  # Integration tests (optional)
├── solution/               # Release binary target for Docker mount (gitignored binary)
├── docker_image/           # Official engine (from 01-edu zip, not in repo)
├── docs/                   # Agent + audit docs
├── Cargo.toml
├── AGENTS.md
└── README.md
```

## 8. Acceptance and Audit Mapping

Primary gate: `docs/audit.md`.

| Category | AUD IDs |
|----------|---------|
| Docker / runtime | AUD-1–AUD-3 |
| Win rate | AUD-4–AUD-6 |
| Unit tests | AUD-7–AUD-11 |
| Quality | AUD-12–AUD-13 |
| Bonus | AUD-B1–AUD-B2 |

## 9. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Slow search on large maps | HIGH | Prune invalid anchors early; cap search depth |
| Timeout on first move | HIGH | Fast parse path; avoid allocations per cell |
| Docker path mismatch | MEDIUM | Document exact mount path in README |
| Opponent-specific quirks | MEDIUM | Record seeds (`-s`) for reproducible tuning |

## 10. Success Metrics

1. All mandatory AUD items (AUD-1–AUD-13) pass.
2. `cargo test` green on CI/host.
3. Win-rate gates documented in `docs/pr-messages/` with command logs.

## Cross-References

| Document | Relationship |
|----------|--------------|
| `docs/requirements.md` | Authoritative stakeholder requirements |
| `docs/SDS.md` | Technical implementation of this PRD |
| `docs/audit.md` | Acceptance gate |
| `docs/ticket-tracker.md` | Implementation traceability |
