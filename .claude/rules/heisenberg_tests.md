---
kind: directive
---

# Tests Meet the Same Standards

- Tests meet the same standards wherever they are written. A standalone test crate, an integration test and a `#[cfg(test)]` module follow the same rules, a test crate is never more lenient than the tests of a production crate, and where the standards for them would differ, the stricter one applies.
- One named exception: an error type declared in test code is not counted among the error types of its crate target. Every other error rule applies to test code unchanged.
