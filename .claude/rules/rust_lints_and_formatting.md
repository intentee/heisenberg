---
kind: rule
section: 3
enforced_by:
  - ClippyConfigurationAmbiguous
  - ClippyConfigurationMissing
  - LintAllowedInManifest
  - LintSuppression
  - RequiredClippySettingMissing
  - RequiredLintNotDenied
  - RustVersionBelowSupported
  - RustfmtConfigurationAmbiguous
  - RustfmtConfigurationInvalid
  - RustfmtConfigurationMissing
  - RustfmtConfigurationUnreadable
  - ToolAttribute
  - UnvettedClippySetting
  - UnvettedRustfmtSetting
---

# Section 3: Lints and formatting

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Never suppress or lower a lint: no `#[allow]`, no `#[expect]`, with or without a reason, and no allow in a manifest `[lints]` table. Fix the code the lint reports. [LintSuppression, LintAllowedInManifest]
- No attribute of the clippy or rustfmt tool, such as `#[rustfmt::skip]` or `#[clippy::msrv]`. [ToolAttribute]
- Every workspace member denies the required lint groups and the panicking lints. [RequiredLintNotDenied]
- The workspace has one clippy configuration per directory, with the required settings and otherwise only settings that add restrictions. [ClippyConfigurationMissing, ClippyConfigurationAmbiguous, RequiredClippySettingMissing, UnvettedClippySetting]
- The workspace root holds a readable, valid rustfmt configuration, one per directory, with only stable settings that choose a layout. [RustfmtConfigurationMissing, RustfmtConfigurationAmbiguous, RustfmtConfigurationInvalid, RustfmtConfigurationUnreadable, UnvettedRustfmtSetting]
- `rust-version` is never older than the supported Rust version. [RustVersionBelowSupported]
