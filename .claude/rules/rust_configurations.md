---
kind: rule
section: 2
enforced_by:
  - AttributeAppliedInNoConfiguration
  - CodeActiveInNoConfiguration
  - NegatedConfiguration
  - VacuousNegatedConfiguration
---

# Section 2: Declared configurations

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Every piece of code is active in at least one declared configuration. [CodeActiveInNoConfiguration]
- Never negate a feature, `test` or `debug_assertions` in a `cfg` predicate; express the condition positively. [NegatedConfiguration]
- Never negate a configuration that no declared configuration sets, such as `not(clippy)` or `not(rustfmt)`. [VacuousNegatedConfiguration]
- Every `cfg_attr` applies in at least one declared configuration; `cfg_attr(rustfmt, ...)` and `cfg_attr(clippy, ...)` do not. [AttributeAppliedInNoConfiguration]
