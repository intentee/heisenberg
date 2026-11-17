---
kind: directive
---

# Dependencies Answer Semantic Questions

- Heisenberg asks a dependency what code means: which definition a name refers to, what a macro call expands to, what type an expression has. It does not use a dependency's editor conveniences that find candidates by searching text, such as rust-analyzer's usage search.
- A dependency's debug assertions are its invariants. Heisenberg behaves the same in debug and release builds, and never relies on an assertion being compiled out.
- When a convenience of a dependency breaks its own invariants on valid input, Heisenberg composes the semantic primitives of that dependency itself, and keeps a test with the input that broke it. It never patches the dependency and never switches its assertions off.
- A question asked for every item, such as which code refers to it, is answered from an index built once per run and looked up many times.
