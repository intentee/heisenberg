---
kind: directive
---

# Configuration Cannot Sidestep Rules

- Heisenberg validates every configuration that can relax its rules or work around them, whether or not it relies on that configuration: manifest lints, clippy configurations, test runner configurations, and its own settings.
- No configuration can sidestep or circumvent a rule.
- When Heisenberg finds a workaround or an override, it directs the fix to the code, such as making slow code or slow tests faster, and never to the configuration.
