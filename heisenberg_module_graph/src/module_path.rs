use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModulePath {
    pub segments: Vec<String>,
}

impl ModulePath {
    #[must_use]
    pub fn child(&self, name: &str) -> Self {
        let mut segments = self.segments.clone();

        segments.push(name.to_owned());

        Self { segments }
    }

    #[must_use]
    pub fn common_ancestor(&self, other: &Self) -> Self {
        let mut shared_length = 0;

        while self
            .segments
            .get(shared_length)
            .is_some_and(|segment| other.segments.get(shared_length) == Some(segment))
        {
            shared_length += 1;
        }

        Self {
            segments: self.segments.iter().take(shared_length).cloned().collect(),
        }
    }

    #[must_use]
    pub fn child_towards(&self, descendant: &Self) -> Option<Self> {
        descendant
            .segments
            .get(self.segments.len())
            .map(|name| self.child(name))
    }
}

impl Display for ModulePath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            &iter::once("crate")
                .chain(self.segments.iter().map(String::as_str))
                .collect::<Vec<&str>>()
                .join("::"),
        )
    }
}
