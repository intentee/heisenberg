# Purpose

- Heisenberg aims to provide a deterministic linter that establishes the foundation for static analysis of a Rust project.
- The goal is to establish explicit, enforceable constraints that downstream analysis tools can rely on.
- The goal is that a project is statically analyzable once Heisenberg reports no issues.
- Heisenberg also turns coding rules into automated checks, but only a rule that can be enforced deterministically and airtight, with no edge case and no loophole, becomes a check.
