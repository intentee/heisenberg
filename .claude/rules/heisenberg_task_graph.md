---
kind: directive
---

# Task Graph

- Heisenberg runs as a dependency graph of tasks on one bounded worker pool.
- A task starts when its inputs exist, runs exactly once, and produces the same results for every worker count; blocking work never occupies a pool worker.
