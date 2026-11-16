# Semantics, Never Heuristics

- Heisenberg cannot do any kind of heuristics, it needs to base all its findings in code semantics.
- Heisenberg uses no heuristics and parses no code by hand. It never decides from textual patterns or regex, and never splits, joins, trims, case-folds or rebuilds source or token text to reach a decision; when a dependency already answers a question, Heisenberg asks it instead of reimplementing it.
- Code is read only through rust-analyzer, and every other input only through the parser its format's owner provides.
- Decisions come from rust-analyzer semantics, token identity (a token's span, kind and exact spelling, compared whole with the text written at its span), identities built from typed values rather than rendered strings, typed configuration, and exact names taken from structured data.
- Beyond that, text is inspected only by the rules about names themselves (module naming, single-letter names, the type-parameter prefix), by rustc's documented lint-name equivalence, by the check of `[env]` keys, and by reading rustc's one-cfg-per-line target tables the way cargo reads them; text is assembled only to render reports and help and to write a dependency's documented input.
