---
kind: process
---

# Development Process

How changes to Heisenberg are made.

- Use the Makefile targets to build, lint and test: `make fmt`, `make clippy`, `make test.coverage`, `make heisenberg.check`.
- Reproduce every bug with a failing test before fixing it; the fix is complete when that test passes unchanged.
- Add every rule test first: the fixture workspace and the test asserting the exact report, then the implementation.
- Fix the root cause, at the deepest link of the dependency chain, and never mask an error.
- Preserve every existing feature unless its removal was requested; when unsure whether something is still needed, keep it and ask.
- Heisenberg passes its own check and its own coverage check with no issue of any kind.
- Every feature behaves identically on Linux and Windows; each target is built and tested on its own platform, never cross-compiled.
