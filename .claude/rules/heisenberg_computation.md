# Compute Once

- Heisenberg computes nothing it does not use and nothing more than once. Within a run, each result is computed only when something consumes it, exactly once, and then shared, and a cheaper exact check runs first whenever it decides the outcome.
- Across runs, Heisenberg keeps no cache of its own: it reuses what the tools it drives already hold, forces no rebuild, and invalidates none of their caches.
- The only repeats are those a dependency's API imposes, and Heisenberg drives each dependency so that it repeats as little as its API allows.
