use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleEvaluation {
    EachAnalysedConfiguration,
    EveryDeclaredConfiguration,
    EveryDeclaredConfigurationInPart,
}

impl Display for RuleEvaluation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::EachAnalysedConfiguration => "in each analysed configuration",
            Self::EveryDeclaredConfiguration => {
                "only when every declared configuration is analysed"
            }
            Self::EveryDeclaredConfigurationInPart => {
                "in each analysed configuration, and in full only when every declared configuration is analysed"
            }
        })
    }
}
