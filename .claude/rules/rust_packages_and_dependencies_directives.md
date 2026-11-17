---
kind: directive
---

# Rust Dependency Directives

No check can decide these requirements, so they are followed by judgement and verified in review.

- Before adding a dependency, verify and state its release activity, maintenance, security advisories, adoption and licence; do not add one that fails.
- Never modify vendored code or a dependency's sources; fix the problem in a wrapper, or at the deepest link of our own dependency chain where its cause lives.
- When a dependency misbehaves, assume it is used wrongly until a minimal reproducer isolates the fault to the dependency.
- Never state how a dependency behaves without reading its source or documentation first.
- Crates never reach into each other's internals; shared interfaces live in a crate that both depend on.
