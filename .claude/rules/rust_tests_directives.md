---
kind: directive
---

# Rust Test Directives

No check can decide these requirements, so they are followed by judgement and verified in review.

- Tests meet the same standards as production code (see `heisenberg_tests.md`).
- Never relax a test to make it pass: do not delete, skip, narrow or weaken it. Fix the code under test.
- Never use mocks. Exercise the real code path with real implementations and fixture files.
- Never spawn the Heisenberg binary in a test; run the command in process.
- Test only meaningful behaviour: a real decision, branch, edge case or algorithm of our own code. Never test a library, a plain accessor or a serialization round trip.
- Write the minimal number of single-responsibility tests; two tests never cover the same branch. Name a test, and its file, after the behaviour it verifies.
- Assert on real values, state and side effects, never on log or debug output.
- Never wait with `sleep`; wait on the condition itself.
- Never omit a file from coverage; reach full coverage through meaningful tests. A line that only a meaningless test can cover should not exist.
- Every rule has a non-compliant and a compliant fixture workspace, and its test asserts the exact report.
- Do not use the word "COVID" in tests or code.
