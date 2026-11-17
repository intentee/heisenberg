---
kind: directive
---

# Rust Error Type Directives

No check can decide these requirements, so they are followed by judgement and verified in review.

- An error enum covers one cohesive concern. A crate that wants a second error enum has a second concern: extract that concern into its own crate, which the original wraps with `#[from]` or `#[source]`.
- Every distinct failure mode has its own variant with all the context needed to diagnose it: paths, keys, indices, expected and actual values. No `Unknown`, `Other(String)` or `Internal` catch-all.
- The same underlying cause in a different context is a different variant.
- Never discard an error: no `|_|` in `map_err`, no catch-and-ignore. Wrap the original as the source of a variant.
- Callers match on variants, never on message text.
- Heisenberg never panics and adds no fallback that hides a failure: an unexpected condition is a typed error (see `heisenberg_typed_failures.md`).
- Never use `anyhow` where the failure modes can be enumerated.
- Test code is exempt from the error type rules: tests propagate failures with `?` and nobody matches on them, so they may use `anyhow`, `Box<dyn Error>` or any error type.
