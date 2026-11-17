---
kind: rule
section: 10
enforced_by:
  - CodeWithoutCoverageData
  - UncoveredCode
---

# Section 10: Coverage

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Every production function is instrumented and has coverage data. [CodeWithoutCoverageData]
- Every region of production code is executed by the tests; write tests for the code, or remove the code that nothing needs. [UncoveredCode]
