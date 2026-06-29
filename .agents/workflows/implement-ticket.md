---
description: Implement a specific ticket for filler.
---

# Ticket Implementation Playbook: {Ticket}

You are implementing a single ticket for **filler**. Produce maintainable code that strictly follows project standards and passes the audit gate before handover.

---

## 1. Primary Directives

Before writing code, read:

| Document | Why |
|----------|-----|
| `AGENTS.md` | Stack, conventions, directory layout, commands |
| `docs/ticket-tracker.md` | Ticket scope, dependencies, REQ/AUD IDs, status |
| `docs/SDS.md` & `docs/PRD.md` | Technical and product detail for this area |
| `docs/audit.md` & `docs/requirements.md` | Verification gates and stakeholder requirements |
| `docs/AGENT_WORKFLOW.md` | How docs connect and how to close tickets |

**Verification gate**: Work is not finished until every AUD ID listed on this ticket is verified and tests pass.

---

## 2. Technical Standards

Follow `AGENTS.md` for language, framework, and style rules. In summary:

- Match existing patterns in the codebase before introducing new abstractions.
- Keep functions focused; extract helpers when logic grows unwieldy.
- **Placement logic stays pure** — no I/O in `placement.rs` tests path.
- Prefer std library; no new crates without ticket/PR justification.
- Robot output must be exactly `X Y\n` per turn.

---

## 3. Workflow Steps

### Step 1: Analysis

- Find the ticket in `docs/ticket-tracker.md` — confirm status, dependencies, size, and coverage (REQ/AUD IDs).
- Read the relevant SDS sections for protocol, placement rules, and module APIs.
- If dependencies are not ✅ Done, stop and pick an unblocked ticket (or document the blocker).

### Step 2: Implementation

- Implement only what the ticket describes; split scope creep into a new tracker row.
- Follow naming, module, and error-handling conventions from `AGENTS.md`.
- Update README if Docker steps, build path, or run commands change.

### Step 3: Testing and QA

- Run `cargo test` — all existing and new tests must pass.
- Run `cargo clippy` and `cargo fmt --check` — fix lint/format issues.
- **Bug workflow** (if you find a regression):
  1. Add a minimal failing test that reproduces the bug.
  2. Fix the implementation.
  3. Re-run the full test suite.

### Step 4: Audit Verification

- Walk through each **AUD-*** ID listed on the ticket in `docs/audit.md`.
- For win-rate tickets: run 5 games alternating p1/p2; record results in PR message.
- Do not mark the ticket Done if any mandatory AUD item fails.

### Step 5: Documentation and Handover

1. **PR message** — use `docs/pr-messages/pr-template.md` as the blueprint.
2. **Save artifact** — `docs/pr-messages/{TicketID}-{short-desc}-pr.md`.
3. **Update tracker** — set ticket status to ✅ Done in `docs/ticket-tracker.md`.
4. **Refresh matrices** — update Requirements/Audit coverage tables if this ticket closes a gap.

---

## 4. Ticket Context: {Ticket}

> [!IMPORTANT]
> Paste the full ticket row (ID, description, deps, REQ/AUD coverage) from `docs/ticket-tracker.md` here before executing.

| Field | Value |
|-------|-------|
| Ticket ID | {Ticket} |
| Dependencies | (from tracker) |
| REQ coverage | (from tracker) |
| AUD coverage | (from tracker) |

**Begin implementation.** Focus on correct protocol behavior, testable placement logic, and audit-ready handover.
