#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IssueText {
    pub definition: &'static str,
    pub name: &'static str,
    pub purpose: &'static str,
    pub summary: &'static str,
}
