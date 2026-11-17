---
kind: directive
---

# Rust Lint Directives

No check can decide these requirements, so they are followed by judgement and verified in review.

- Never relax a linter, formatter or type checker to make code pass: no loosened configuration, no inline suppression, no restructuring whose only purpose is to escape a rule.
- Fix every compiler, clippy and Heisenberg diagnostic before building or running tests, including those that existed before the change.
- Treat a diagnostic as current unless there is concrete evidence that it is stale; when in doubt, run the check again.
- This is stricter than `#[expect]` with a reason: Heisenberg accepts no suppression at all.
