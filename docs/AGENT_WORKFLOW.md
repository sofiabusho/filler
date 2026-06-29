# Agent Workflow — filler

<!-- How coding agents should navigate docs, pick up work, and close tickets.
     This is the "map" — AGENTS.md is the "rules of the road." -->

---

## 1. Document Hierarchy

Read documents in this order when starting any task:

```text
AGENTS.md                    ← coding rules, stack, directory layout
    ↓
docs/requirements.md         ← what stakeholders require (REQ IDs)
docs/audit.md                ← how "done" is verified (AUD IDs)
    ↓
docs/PRD.md                  ← product detail and priorities
docs/SDS.md                  ← technical specs and API contracts
    ↓
docs/ticket-tracker.md       ← current sprint, ticket status, dependencies
    ↓
docs/pr-messages/pr-template.md  ← handover format when closing a ticket
```

**Rule**: Never mark a ticket done without verifying relevant **AUD-*** items in `docs/audit.md`.

---

## 2. Starting a New Ticket

1. **Locate the ticket** in `docs/ticket-tracker.md` — note ID, dependencies, REQ/AUD coverage.
2. **Read source of truth** — `AGENTS.md`, relevant PRD/SDS sections, linked REQ and AUD IDs.
3. **Check dependencies** — prerequisite tickets must be ✅ Done (or explicitly waived).
4. **Update status** — set ticket to 🟢 In Progress in `docs/ticket-tracker.md`.
5. **Run the workflow** — follow `.agents/workflows/implement-ticket.md`.

---

## 3. During Implementation

| Activity | Reference |
|----------|-----------|
| Architecture decisions | `docs/SDS.md`, `docs/PRD.md` |
| Coding style | `AGENTS.md` |
| Protocol / placement contracts | `docs/SDS.md` §4 |
| Acceptance criteria | `docs/audit.md` (AUD IDs on the ticket) |
| Scope boundaries | `docs/requirements.md`, PRD non-goals |

**Do not**:

- Expand scope beyond the ticket without updating `docs/ticket-tracker.md` and requirements.
- Skip tests or audit checks to "move faster."
- Edit `docs/requirements.md` or `docs/audit.md` without stakeholder approval (they are gates, not scratch pads).

---

## 4. Closing a Ticket

1. **Run quality commands** — `cargo test`, `cargo clippy`, `cargo fmt --check` (see `AGENTS.md`).
2. **Verify audit items** — every AUD ID listed on the ticket row in the tracker.
3. **Write PR message** — copy `docs/pr-messages/pr-template.md`; save as `docs/pr-messages/{TicketID}-{short-desc}-pr.md`.
4. **Update tracker** — set ticket status to ✅ Done; refresh coverage matrices if needed.
5. **Update docs** — if public API or behavior changed, update SDS and/or README.

**Win-rate tickets (T20–T22)**: include seeds, p1/p2 order, and W/L table in the PR message.

---

## 5. Traceability Flow

```text
requirements.md (REQ-*)  ──→  PRD.md (detailed spec)
        │                           │
        └──────────→  ticket-tracker.md (tickets + coverage)
                                │
audit.md (AUD-*)  ──────────────┘
                                │
                                ↓
                    pr-messages/{ticket}-pr.md (evidence)
```

Every ticket should list which **REQ-*** and **AUD-*** IDs it satisfies. Gates (G1, G2) aggregate ticket completion.

---

## 6. When Stuck

| Situation | Action |
|-----------|--------|
| Spec ambiguity | Check PRD → SDS → `docs/raw/REQUIREMENTS-SOURCE.md`; note assumption in PR message |
| Audit failure | Fix before marking Done; do not waive AUD items without documenting in tracker |
| Blocked dependency | Set ticket 🔴 Blocked; note blocker in tracker; pick next unblocked ticket |
| Scope creep | Split into new ticket; add to tracker with new REQ linkage |
| Docker unavailable | Complete unit-test tickets on host; document engine runs for later |

---

## 7. Optional Extensions

| Path | Purpose |
|------|---------|
| `.agents/workflows/implement-ticket.md` | Step-by-step implementation playbook |
| `.agents/skills/` | Project-specific agent skills (see README there) |
| `.agents/rules/` | Supplemental rules beyond `AGENTS.md` |
| `.cursor/rules/project-standards.mdc` | Cursor IDE rule pointing agents to this doc set |
