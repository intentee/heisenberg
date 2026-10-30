# Project Context

Heisenberg aims to provide a deterministic linter that establishes the foundation for static analysis of a Rust project.

Every source file, build input, and supported configuration must be explicitly accounted for. Anything that introduces ambiguity, depends on undeclared inputs, or prevents reliable analysis must result in a hard error.

Heisenberg must never silently skip unsupported constructs or treat incomplete analysis as success. Given the same declared inputs, it must produce the same results.

Heisenberg cannot do any kind of heuristics, it needs to base all its findings in code semantics.

The goal is to establish explicit, enforceable constraints that downstream analysis tools can rely on.
