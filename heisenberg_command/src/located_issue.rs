use crate::issue::Issue;
use crate::issue_location::IssueLocation;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct LocatedIssue {
    pub location: IssueLocation,
    pub issue: Issue,
}
