use clap::Args;

use crate::issue_type::IssueType;

#[derive(Args, Debug)]
pub struct ExplainCommand {
    #[arg(
        value_enum,
        value_name = "ISSUE_TYPE",
        help = "Issue type, as the report and check --help name it, such as LintSuppression"
    )]
    pub issue_type: IssueType,
}
