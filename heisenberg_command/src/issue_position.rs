use heisenberg_module_graph::line_position::LinePosition;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssuePosition {
    WholeFile,
    Line(LinePosition),
}
