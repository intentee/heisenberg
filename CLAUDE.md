# Project Context

Heisenberg aims to provide a deterministic linter that establishes the foundation for static analysis of a Rust project.

Every source file, build input, and supported configuration must be explicitly accounted for. Anything that introduces ambiguity, depends on undeclared inputs, or prevents reliable analysis must result in a hard error.

Heisenberg must never silently skip unsupported constructs or treat incomplete analysis as success. Given the same declared inputs, it must produce the same results.

Heisenberg never panics. Every failure it meets is a typed error, and before calling into a dependency it upholds the invariants that dependency relies on, such as rust-analyzer needing text for every file it loaded, reporting each violation as a typed error.

Tests meet the same standards wherever they are written. A standalone test crate, an integration test and a `#[cfg(test)]` module follow the same rules, a test crate is never more lenient than the tests of a production crate, and where the standards for them would differ, the stricter one applies.

Heisenberg cannot do any kind of heuristics, it needs to base all its findings in code semantics.

The goal is to establish explicit, enforceable constraints that downstream analysis tools can rely on.

Heisenberg never fights Rust. Every rule must have a compliant form that Rust accepts; when the language leaves no other form, the rule does not apply to that construct.

The goal is that a project is statically analyzable once Heisenberg reports no issues.

Heisenberg validates every configuration that can relax its rules or work around them, whether or not it relies on that configuration: manifest lints, clippy configurations, test runner configurations, and its own settings. No configuration can sidestep or circumvent a rule. When Heisenberg finds a workaround or an override, it directs the fix to the code, such as making slow code or slow tests faster, and never to the configuration.

Heisenberg uses no heuristics and parses no code by hand. It never decides from textual patterns or regex, and never splits, joins, trims, case-folds or rebuilds source or token text to reach a decision; when a dependency already answers a question, Heisenberg asks it instead of reimplementing it. Code is read only through rust-analyzer, and every other input only through the parser its format's owner provides. Decisions come from rust-analyzer semantics, token identity (a token's span, kind and exact spelling, compared whole with the text written at its span), identities built from typed values rather than rendered strings, typed configuration, and exact names taken from structured data. Beyond that, text is inspected only by the rules about names themselves (module naming, single-letter names, the type-parameter prefix), by rustc's documented lint-name equivalence, by the check of `[env]` keys, and by reading rustc's one-cfg-per-line target tables the way cargo reads them; text is assembled only to render reports and help and to write a dependency's documented input.

Heisenberg never detects the operating system or the host, and contains no OS-specific workaround. Every run is fully described by its declared inputs and explicit arguments; platform differences live only in its dependencies.

Heisenberg computes nothing it does not use and nothing more than once. Within a run, each result is computed only when something consumes it, exactly once, and then shared, and a cheaper exact check runs first whenever it decides the outcome. Across runs, Heisenberg keeps no cache of its own: it reuses what the tools it drives already hold, forces no rebuild, and invalidates none of their caches. The only repeats are those a dependency's API imposes, and Heisenberg drives each dependency so that it repeats as little as its API allows.

Heisenberg runs as a dependency graph of tasks on one bounded worker pool. A task starts when its inputs exist, runs exactly once, and produces the same results for every worker count; blocking work never occupies a pool worker.
