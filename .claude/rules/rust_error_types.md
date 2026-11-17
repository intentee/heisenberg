---
kind: rule
section: 7
enforced_by:
  - ErrorIssuedOutsideItsCrate
  - ErrorMessageFunction
  - ErrorMessageOnTheEnum
  - ErrorSourceOutsideChain
  - ErrorTypeNotAnEnum
  - ErrorTypeOutsideItsModule
  - HandwrittenErrorDisplay
  - HandwrittenErrorType
  - MacroGeneratedErrorType
  - MultipleErrorTypes
  - ResultErrorNotAnError
  - SourceInErrorMessage
  - UntypedResultError
---

# Section 7: Error types

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition. This section applies to production code and not to test code: a test may declare any error type and return any error, such as `anyhow` or `Box<dyn Error>`.

- Each crate target has one error type: an enum that derives `thiserror::Error`, in a module file named after it. [MultipleErrorTypes, ErrorTypeNotAnEnum, ErrorTypeOutsideItsModule, HandwrittenErrorType, MacroGeneratedErrorType]
- An error type is issued only by the crate that owns it: another crate of the workspace never returns it as the `Err` of a function and never constructs it; it handles the error where it receives it, or wraps it as the source of a variant of its own error type. [ErrorIssuedOutsideItsCrate]
- `Err` is reserved for failures. An outcome of the regular flow is returned in `Ok`, as a variant of an enum of outcomes; a value is never handed back through `Err`. A failure is an error type: a workspace type used as `Err` implements `std::error::Error`. [ResultErrorNotAnError]
- A `Result` never erases its failure: no `String`, `&str`, `()`, primitive, collection, `Box<dyn Error>` or `anyhow::Error` as `Err`. [UntypedResultError]
- Several failures are returned as a `Vec` of the error type, and a variant may hold such a `Vec`; no aggregate error type is declared for it. [UntypedResultError, ErrorSourceOutsideChain]
- Each variant carries its own `#[error("...")]` format string: no message on the enum, no `fmt` function, no hand-written `Display`. [ErrorMessageOnTheEnum, ErrorMessageFunction, HandwrittenErrorDisplay]
- A variant that holds an error makes it its source, with `#[source]` or `#[from]`; a further cause is formatted in the message of the variant. [ErrorSourceOutsideChain]
- A message never formats the source of its variant. [SourceInErrorMessage]
