# Project Context

Heisenberg aims to provide a deterministic linter that establishes the foundation for static analysis of a Rust project.

Every source file, build input, and supported configuration must be explicitly accounted for. Anything that introduces ambiguity, depends on undeclared inputs, or prevents reliable analysis must result in a hard error.

Heisenberg must never silently skip unsupported constructs or treat incomplete analysis as success. Given the same declared inputs, it must produce the same results.

Heisenberg cannot do any kind of heuristics, it needs to base all its findings in code semantics.

The goal is to establish explicit, enforceable constraints that downstream analysis tools can rely on.

Heisenberg never fights Rust. Every rule must have a compliant form that Rust accepts; when the language leaves no other form, the rule does not apply to that construct.

The goal is that a project is statically analyzable once Heisenberg reports no issues.

Heisenberg validates every configuration it relies on, such as manifest lints, clippy configurations and its own settings, so that no configuration can sidestep or circumvent a rule.

Heisenberg decides nothing from textual patterns and uses no regex. Decisions come from rust-analyzer semantics, token identity (a token's span, kind and exact spelling), typed configuration, exact names taken from structured data, and documented formats parsed by their libraries. Beyond that, text is inspected only by the rules about names themselves (module naming, single-letter names, the type-parameter prefix), by rustc's documented lint-name equivalence, and by the check of `[env]` keys.

Heisenberg never detects the operating system or the host, and contains no OS-specific workaround. Every run is fully described by its declared inputs and explicit arguments; platform differences live only in its dependencies.

Heisenberg never computes the same result twice. Work whose result already exists is reused, never redone or invalidated: it forces no rebuild and invalidates no cache of the tools it drives, and within a run each result is computed once and then shared. The only repeats are those a dependency's API imposes.
