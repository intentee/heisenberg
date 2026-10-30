use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_module_graph::source_location::SourceLocation;

use crate::issue_position::IssuePosition;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueLocation {
    pub file: String,
    pub position: IssuePosition,
}

impl IssueLocation {
    #[must_use]
    pub fn new_position(SourceLocation { file, position }: &SourceLocation) -> Self {
        Self {
            file: file.clone(),
            position: IssuePosition::Line(position.clone()),
        }
    }

    #[must_use]
    pub fn new_whole_file(file: &str) -> Self {
        Self {
            file: file.to_owned(),
            position: IssuePosition::WholeFile,
        }
    }
}

impl Display for IssueLocation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self { file, position } = self;

        match position {
            IssuePosition::WholeFile => formatter.write_str(file),
            IssuePosition::Line(line_position) => write!(formatter, "{file}:{line_position}"),
        }
    }
}
