# AGENTS.md — filler

> Coding agent instructions for **filler**.
> Read this file in full before writing any code.

---

## Project Overview

**filler** is a 01-edu Filler robot: a Rust binary that reads Anfield + piece data from stdin and writes `X Y\n` placements to stdout, competing inside the official Dockerized `game_engine`.

Primary scope in this repository:

- satisfy `docs/requirements.md`
- satisfy `docs/audit.md`
- deliver a winning `solution/` binary plus unit tests

---

## Source of Truth

| Document | Path | Purpose |
|----------|------|---------|
| Requirements | `docs/requirements.md` | What must be built (external or stakeholder spec) |
| Audit Gate | `docs/audit.md` | Pass/fail acceptance criteria |
| PRD | `docs/PRD.md` | Detailed requirements and architecture |
| SDS | `docs/SDS.md` | Technical spec, API contracts, and examples |
| Ticket Tracker | `docs/ticket-tracker.md` | Work breakdown, status, and traceability |
| Agent Workflow | `docs/AGENT_WORKFLOW.md` | How agents navigate docs and close tickets |
| This file | `AGENTS.md` | Agent coding guidelines |

**Always verify your work against `docs/audit.md` before marking a ticket done.**

---

## Technology Constraints

| Rule | Detail |
|------|--------|
| **Language** | Rust (edition 2021) |
| **Runtime / Platform** | Native binary executed by `game_engine` (Linux inside Docker) |
| **Package manager** | cargo |
| **Frameworks** | **Allowed:** std only by default. **Forbidden:** heavy game engines, async runtimes, networking crates unless a ticket explicitly approves |
| **Dependencies** | Prefer std; add crates only with justification in PR/ticket |
| **Styling** | N/A (CLI robot) |
| **Database** | N/A |

---

## Directory Structure

```
filler/
├── src/                    # Robot source
│   ├── lib.rs              # Library root (modules + tests)
│   ├── main.rs             # Binary entry (stdin loop in T12)
│   ├── parse.rs            # Engine protocol parsing
│   ├── model.rs            # Anfield, Piece, PlayerId
│   ├── placement.rs        # Validity and overlap rules
│   └── strategy.rs         # Move selection heuristics
├── tests/                  # Optional integration tests
├── solution/               # Release binary for Docker mount (build artifact)
├── docker_image/           # Official engine zip contents (local, not committed)
├── docs/
│   ├── requirements.md
│   ├── audit.md
│   ├── PRD.md
│   ├── SDS.md
│   ├── ticket-tracker.md
│   ├── AGENT_WORKFLOW.md
│   ├── raw/                # Read-only source briefs
│   └── pr-messages/
├── .agents/
│   ├── workflows/
│   ├── skills/
│   └── rules/
├── .cursor/rules/
├── Cargo.toml
├── AGENTS.md
└── README.md
```

---

## Coding Standards

### General

- Keep **placement logic pure** — no I/O in `placement.rs` or `parse` internals used by tests.
- Prefer `const` for symbol maps and direction tables; tunables in one place if needed.
- No `println!` in library paths except the final move emission in `main`/io helper.
- Follow existing patterns before introducing new abstractions.

### Naming Conventions

| Entity | Convention | Example |
|--------|-----------|---------|
| Files | `snake_case.rs` | `placement.rs` |
| Functions | `snake_case` | `validate_placement`, `parse_turn` |
| Structs / enums | `PascalCase` | `Anfield`, `PlayerId` |
| Constants | `SCREAMING_SNAKE_CASE` | `FALLBACK_MOVE` |
| Test modules | `snake_case` | `mod parse_tests` |

### Module Pattern

- One concern per module (`parse`, `model`, `placement`, `strategy`).
- `main.rs` only wires stdin loop and calls `strategy::choose_move`.
- Public types used across modules live in `model.rs`.

### Domain-Specific Rules

- **Exactly one overlap** — `validate_placement` must count own cells (`@`/`a` or `$`/`s`) under filled piece cells; valid only when count == 1 (REQ-4).
- **Opponent symbols** — treat any non-empty, non-own cell as opponent (REQ-5).
- **Output format** — `"{x} {y}\n"` only; no debug prefix (REQ-2, REQ-13).
- **No-move fallback** — `0 0\n` when `iter_valid_placements` is empty (REQ-7).
- **Docker path** — release binary copied to `solution/` for `/filler/solution` mount (REQ-8).

---

## Testing Guidelines

- **Runner**: `cargo test`
- **Location**: `#[cfg(test)]` in modules; optional `tests/` for integration
- **Coverage expectation**: parsing, placement (valid + invalid), boundaries, output format — per REQ-10–REQ-13 and AUD-8–AUD-11
- **Run command**: `cargo test`

Before completing a ticket:

- [ ] New logic has tests where applicable.
- [ ] All existing tests pass.
- [ ] Manual audit items from `docs/audit.md` relevant to this ticket are verified.

---

## Development Workflow

```bash
cargo build --release          # Build robot
cp target/release/filler solution/   # For Docker mount (adjust binary name)
cargo test                     # Unit tests
cargo clippy                   # Lint
cargo fmt --check              # Format check
```

**Docker matches** (inside `docker_image` container after mount):

```bash
./linux_game_engine -f maps/map00 -p1 solution/filler -p2 linux_robots/wall_e
```

---

## Commit & Branch Conventions

| Branch | Purpose |
|--------|---------|
| `main` | Stable, audit-ready code |
| `feat/<ticket-id>-<short-desc>` | Feature branches per ticket |
| `fix/<ticket-id>-<short-desc>` | Bug fix branches |

Commit messages: `feat(filler): <short description>` or `fix(placement): <short description>`

---

## PR Checklist (for each ticket)

- [ ] Code compiles (`cargo build --release`).
- [ ] All existing tests pass (`cargo test`).
- [ ] New code has tests where applicable.
- [ ] Lint/format checks pass (`cargo clippy`, `cargo fmt --check`).
- [ ] No unapproved dependencies added.
- [ ] Audit checklist items covered by this ticket still pass (`docs/audit.md`).
- [ ] Documentation updated if public API or behavior changed.
- [ ] `docs/ticket-tracker.md` updated with ticket status.
- [ ] PR message saved to `docs/pr-messages/` using `docs/pr-messages/pr-template.md`.
