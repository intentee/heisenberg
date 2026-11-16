# Typed Failures

- Heisenberg never panics. Every failure it meets is a typed error.
- Before calling into a dependency it upholds the invariants that dependency relies on, such as rust-analyzer needing text for every file it loaded, reporting each violation as a typed error.
