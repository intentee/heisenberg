---
kind: rule
section: 7
enforced_by:
  - ErrorMessageFunction
  - ErrorMessageOnTheEnum
  - ErrorSourceOutsideChain
  - ErrorTypeNotAnEnum
  - ErrorTypeOutsideItsModule
  - HandwrittenErrorDisplay
  - HandwrittenErrorType
  - MacroGeneratedErrorType
  - MultipleErrorTypes
  - SourceInErrorMessage
  - UntypedResultError
---

# Section 7: Error types

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Each crate target has one error type: an enum that derives `thiserror::Error`, in a module file named after it. Error types in test code are not counted. [MultipleErrorTypes, ErrorTypeNotAnEnum, ErrorTypeOutsideItsModule, HandwrittenErrorType, MacroGeneratedErrorType]
- Every `Result` names an error type; never a string, a unit or another value that is not an error. [UntypedResultError]
- Each variant carries its own `#[error("...")]` format string: no message on the enum, no `fmt` function, no hand-written `Display`. [ErrorMessageOnTheEnum, ErrorMessageFunction, HandwrittenErrorDisplay]
- A variant that holds an error makes it its source, with `#[source]` or `#[from]`; a further cause is formatted in the message of the variant. [ErrorSourceOutsideChain]
- A message never formats the source of its variant. [SourceInErrorMessage]
