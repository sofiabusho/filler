# Ticket Tracker — filler

> Legend: 🔴 Blocked · 🟡 Ready · 🟢 In Progress · ✅ Done · ⬜ Not Started
>
> **ID Legend**:
> - **REQ-***: Functional Requirements (`docs/requirements.md`)
> - **AUD-***: Audit Acceptance Criteria (`docs/audit.md`)

Last refreshed: 2026-06-30 (T01–T03 ✅)

---

## 1) Scope Contract

This tracker is **requirements-first** and **audit-first**.

Execution order:

1. `docs/requirements.md` — stakeholder requirements
2. `docs/audit.md` — acceptance gates
3. Feature delivery per `docs/PRD.md` / `docs/SDS.md`
4. Optional stretch goals (T40–T41, non-blocking)

---

## 2) Requirement and Audit IDs

### Requirements IDs (from `docs/requirements.md`)

- `REQ-1`: Read engine stdin protocol
- `REQ-2`: Write `X Y\n` to stdout
- `REQ-3`: Player symbols (`a`/`@` vs `s`/`$`)
- `REQ-4`: Exactly one territory overlap
- `REQ-5`: No opponent overlap
- `REQ-6`: In-bounds placement
- `REQ-7`: No-move fallback output
- `REQ-8`: Docker + `game_engine` setup
- `REQ-9`: Win ≥ 4/5 vs wall_e, h2_d2, bender
- `REQ-10`–`REQ-13`: Unit test areas (parse, placement, boundary, output)
- `REQ-B1`, `REQ-B2`: Bonus visualizer, beat terminator
- `NFR-1`–`NFR-6`: Rust, binary, Docker, timeout safety, good practices, host tests

### Audit IDs (from `docs/audit.md`)

- `AUD-1`–`AUD-3`: Docker, runtime, placement correctness
- `AUD-4`–`AUD-6`: Win-rate gates (map00–map02)
- `AUD-7`–`AUD-11`: Unit tests (pass + four areas + output)
- `AUD-12`–`AUD-13`: Good practices and test breadth
- `AUD-B1`–`AUD-B2`: Bonus visualizer, terminator

---

## Sprint 0 — Bootstrap and guardrails

> **Goal**: Repo scaffold, quality commands, Docker documented.

| ID | Status | Ticket | Size | Deps | Coverage |
|----|--------|--------|------|------|----------|
| T01 | ✅ | **Project scaffolding**: `cargo init`, module stubs (`parse`, `model`, `placement`, `strategy`), `solution/` dir | S | — | NFR-1, NFR-2 |
| T02 | ✅ | **Quality gates**: `cargo test` harness, clippy/fmt in README | S | T01 | NFR-5, NFR-6, AUD-12 |
| T03 | ✅ | **Docker runbook**: document `docker_image` setup, mount path, smoke `game_engine` command | S | — | REQ-8, AUD-1 |

---

## Sprint 1 — Protocol and placement core

> **Goal**: Parse turns, validate placements, emit formatted moves.

| ID | Status | Ticket | Size | Deps | Coverage |
|----|--------|--------|------|------|----------|
| T10 | ⬜ | **Input parsing**: `parse_turn`, Anfield grid, piece mask from brief fixtures | M | T01 | REQ-1, REQ-3, REQ-10, AUD-8 |
| T11 | ⬜ | **Placement validation**: overlap counting, bounds, opponent check | M | T10 | REQ-4–REQ-6, REQ-11, REQ-12, AUD-9, AUD-10 |
| T12 | ⬜ | **IO loop + output**: stdin loop, `X Y\n` serializer, `0 0` fallback | M | T10, T11 | REQ-2, REQ-7, REQ-13, AUD-2, AUD-3, AUD-11 |
| T13 | ⬜ | **Baseline strategy**: pick valid move maximizing new territory | M | T11 | REQ-9 (partial) |

---

## Sprint 2 — Win-rate tuning

> **Goal**: Meet mandatory opponent gates.

| ID | Status | Ticket | Size | Deps | Coverage |
|----|--------|--------|------|------|----------|
| T20 | ⬜ | **Beat wall_e**: tune/heuristics for `maps/map00`, 5-run battery | M | T13 | REQ-9, AUD-4 |
| T21 | ⬜ | **Beat h2_d2**: tune for `maps/map01`, 5-run battery | M | T20 | REQ-9, AUD-5 |
| T22 | ⬜ | **Beat bender**: tune for `maps/map02`, 5-run battery | M | T21 | REQ-9, AUD-6 |

---

## Sprint 3 — Audit delivery

> **Goal**: Full test pass and audit sign-off.

| ID | Status | Ticket | Size | Deps | Coverage |
|----|--------|--------|------|------|----------|
| T30 | ⬜ | **Audit dry-run**: `cargo test`, AUD-1–AUD-13 evidence in PR messages | S | T22, T03 | AUD-7, AUD-12, AUD-13, Gate G2 |

---

## Sprint B — Bonus (non-blocking)

| ID | Status | Ticket | Size | Deps | Coverage |
|----|--------|--------|------|------|----------|
| T40 | ⬜ | **Graphic visualizer** *(bonus)* | L | T12 | REQ-B1, AUD-B1 |
| T41 | ⬜ | **Beat terminator** *(bonus)* | L | T22 | REQ-B2, AUD-B2 |

---

## Verification Gates

### Gate G1 — Core robot (Required)

**Pass criteria**:

- T10–T12 ✅
- `cargo test` passes (AUD-7)
- Student player runs in Docker without crash (AUD-2)

**Evidence**:

- Test output in `docs/pr-messages/T12-io-loop-pr.md`

### Gate G2 — Audit ready (Required)

**Pass criteria**:

- AUD-1–AUD-13 pass
- Win ≥ 4/5 for AUD-4, AUD-5, AUD-6
- Coverage matrices below accurate

**Evidence**:

- Win-rate logs in `docs/pr-messages/T20`–`T22` PR messages
- `docs/pr-messages/T30-audit-dry-run-pr.md`

---

## Requirements Coverage Matrix

| Requirement ID | Requirement | Tickets | Gate |
|----------------|-------------|---------|------|
| REQ-1 | Stdin parsing | T10 | G1 |
| REQ-2 | Stdout `X Y\n` | T12 | G1 |
| REQ-3 | Player symbols | T10 | G1 |
| REQ-4–REQ-6 | Placement rules | T11 | G1 |
| REQ-7 | No-move fallback | T12 | G1 |
| REQ-8 | Docker setup | T03 | G1 |
| REQ-9 | Win opponents | T20–T22 | G2 |
| REQ-10–REQ-13 | Unit tests | T10–T12 | G2 |
| REQ-B1 | Visualizer | T40 | — |
| REQ-B2 | Terminator | T41 | — |

---

## Audit Coverage Matrix

| Audit ID | Audit Check | Tickets | Gate |
|----------|-------------|---------|------|
| AUD-1 | Docker works | T03 | G1 |
| AUD-2 | Player runs | T12 | G1 |
| AUD-3 | Correct overlap | T11, T12 | G1 |
| AUD-4 | vs wall_e | T20 | G2 |
| AUD-5 | vs h2_d2 | T21 | G2 |
| AUD-6 | vs bender | T22 | G2 |
| AUD-7 | Tests pass | T02, T30 | G2 |
| AUD-8–AUD-11 | Test areas | T10–T12 | G2 |
| AUD-12–AUD-13 | Quality + breadth | T02, T30 | G2 |
| AUD-B1 | Visualizer | T40 | — |
| AUD-B2 | Terminator | T41 | — |

---

## Immediate Next Work Queue

1. **T10** — Input parsing (`parse_turn`, Anfield grid, piece mask)
2. **T11** — Placement validation (overlap counting, bounds)
3. **T12** — IO loop + `X Y\n` output

---

## Cross-References

| Document | Relationship |
|----------|--------------|
| `docs/requirements.md` | Source of REQ IDs |
| `docs/audit.md` | Source of AUD IDs |
| `docs/pr-messages/` | Per-ticket handover artifacts |
| `.agents/workflows/implement-ticket.md` | Playbook for executing tickets |
