use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TestViolation {
    Ignored,
    ModuleInItsOwnFile,
    ModuleNotNamedTests,
    OutsideTestModule,
}

impl Display for TestViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Ignored => "Test skipped by an ignore attribute",
            Self::ModuleInItsOwnFile => "Test module is declared in a file of its own",
            Self::ModuleNotNamedTests => "Test module is not named tests",
            Self::OutsideTestModule => "Test is written outside a test module",
        })
    }
}
