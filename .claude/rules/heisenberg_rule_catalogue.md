---
kind: directive
---

# Rule Catalogue

How Heisenberg's rules are organised.

- Every rule is mandatory and decidable. There are no advisory rules, no deviations, no re-categorisation and no "possible violation" findings, because no configuration can sidestep a rule.
- A requirement that no check can decide is a directive: it lives in a `*_directives.md` file and never becomes a check.
- Every rule states its applicability, `all code` or `production code`. The property lives on the rule itself, one function enforces it for every rule, and the catalogue prints the same property, so text and behaviour cannot drift.
- Exceptions are part of the rule record: each is enumerated under the rule and decided by the check. A rule whose remedy has no form that compiles and passes every other rule gains an exception or a compliant form before it ships.
- Every rule has a stable, opaque identifier `H<number>`, such as `H001`, beside its name. A number is assigned once, to one rule, and never reused; the section is metadata and never part of the identifier.
- Every rule has a section, a scope (module file, crate target, package or workspace) and an evaluation (each analysed configuration, or every declared configuration).
- Every rule states its headline, its definition, its rationale and its remedy, in that order, and names its related rules.
- The partial-run footer, the help and the catalogue are derived from that metadata, never written by hand a second time.
- A report ends with how many rules it evaluated, evaluated in part and did not evaluate, and with the version of Heisenberg that produced it.
- Each `rust_*.md` rule file mirrors one section of the catalogue and names, for every statement, the checks that enforce it.
