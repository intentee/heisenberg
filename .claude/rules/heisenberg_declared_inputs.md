---
kind: directive
---

# Declared Inputs and Determinism

- Every source file, build input, and supported configuration must be explicitly accounted for. Anything that introduces ambiguity, depends on undeclared inputs, or prevents reliable analysis must result in a hard error.
- Heisenberg must never silently skip unsupported constructs or treat incomplete analysis as success. Given the same declared inputs, it must produce the same results.
- Heisenberg never detects the operating system or the host, and contains no OS-specific workaround. Every run is fully described by its declared inputs and explicit arguments; platform differences live only in its dependencies.
