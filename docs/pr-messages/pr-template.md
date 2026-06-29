---
title: "feat(filler): {TICKET_NAME}"
---

# PR Implementation Report: {TICKET_ID}

<!-- Save completed reports as: docs/pr-messages/{TicketID}-{short-desc}-pr.md
     Example: docs/pr-messages/T10-input-parsing-pr.md -->

## Summary

<!-- 2–4 sentences: what changed and which REQ/AUD IDs this satisfies. -->

{BRIEF_EXECUTIVE_SUMMARY}

## Key Changes

- **{MODULE_OR_FILE}**: {CHANGE_DESCRIPTION}
- **{MODULE_OR_FILE}**: {CHANGE_DESCRIPTION}

## Technical Decisions

<!-- Document non-obvious choices so the next agent or reviewer understands "why." -->

- **{DECISION_TITLE}**: {RATIONALE}

## Verification Results

### Automated Checks

- [ ] `cargo test` passes (including new tests for this feature)
- [ ] `cargo clippy` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo build --release` succeeds

### Manual Audit (against `docs/audit.md`)

<!-- List every AUD ID tied to this ticket. Mark Pass / Fail / N/A. -->

- [ ] **AUD-{N}**: {Pass|Fail|N/A} — {NOTES}

### Win-rate battery (if applicable)

| Run | Student side | Opponent | Map | Seed | Result |
|-----|--------------|----------|-----|------|--------|
| 1 | p1 | | | | |
| 2 | p2 | | | | |
| … | | | | | |

### Requirements Traceability

- [ ] **REQ-{N}**: {HOW_THIS_TICKET_SATISFIES_IT}

## Artifacts

- **Test output**: {link, log snippet, or "see below"}
- **Lint output**: {link or "clean"}

---

## Next Steps

{NEXT_TICKET_OR_GATE}
