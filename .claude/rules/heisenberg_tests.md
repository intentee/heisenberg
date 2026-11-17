---
kind: directive
---

# Tests Meet the Same Standards, With Named Exceptions

- Test code is one thing everywhere: an inline `#[cfg(test)]` module, an integration test target, and every target of a declared test crate other than a benchmark target. Every rule treats the three alike, through one predicate; no rule asks "is this a test?" in its own way.
- Tests meet the same standards as production code, and a test crate is never more lenient than the tests of a production crate.
- A rule does not apply to test code when it exists to give production code a navigable architecture or a matchable error taxonomy. Nobody finds a test by its file name or matches on its error, and a test must be able to declare the shapes it feeds to the code under test. That is two groups of rules and no others: the error type rules (section 7 of the catalogue) and the rules about where items live (one public item per module, module names, impl placement, the constants module).
- Applicability is a property of the rule, `all code` or `production code`, printed by `heisenberg explain` and `heisenberg catalogue`. It is decided by the check, never by a configuration.
- Every other rule applies to test code unchanged: analysable code, configurations, lints and formatting, imports, order, names, data shapes, comments, and the rules about how tests themselves are organised.
