---
kind: directive
---

# Rust Design Directives

No check can decide these requirements, so they are followed by judgement and verified in review.

- Use the fewest functions, types, files, dependencies and parameters that solve the problem in front of you; no speculative generality.
- Keep APIs as small as possible: one function and one error type before options and abstractions.
- No duplicated behaviour, even where the text differs: extract it into a third module that every user depends on, never one peer importing from another.
- A struct holds fields of one responsibility; group parameters by responsibility, never into one catch-all params or config type.
- Every piece of state has one authoritative home. An entity's id lives on the entity beside its other fields, and derived values are derived, not copied.
- Never use `Option` as a state heuristic. Keep state in explicit fields, or in one struct per state that maps into the next while consuming `self`.
- Every constant is grounded in measured behaviour, a documented limit or a system constraint; a value that cannot be grounded is flagged as an assumption.
- Decide from structured data and types, never from regex, pattern matching on text or string heuristics.
- Always use explicit, descriptive lifetime names and destructure struct arguments wherever possible.
