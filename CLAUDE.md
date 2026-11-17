# Project Context

Heisenberg is a deterministic linter that makes a Rust project statically analyzable and turns coding rules into automated, airtight checks.

Its standards live in `.claude/rules/` and are self-contained. Every file states its kind in its front matter:

- `rule`: `rust_<section>.md`, one file per section of the rule catalogue (`heisenberg catalogue`). Every statement is enforced by the checks it names.
- `directive`: `heisenberg_*.md`, the principles of the linter itself, `rust_*_directives.md`, the requirements no check can decide, and the standards for Makefiles and GitHub workflows.
- `process`: `heisenberg_development_process.md`, how changes are made.

Where a rule here is stricter than a general one, the rule here applies.
