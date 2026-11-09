use std::collections::BTreeSet;

use crate::constants::WORD_SEPARATOR;

pub struct NameWords<'name> {
    pub name: &'name str,
}

impl NameWords<'_> {
    pub fn are_spelled_by(&self, module: &str) -> bool {
        let words = self.words();
        let segments: Vec<&str> = module
            .split(WORD_SEPARATOR)
            .filter(|segment| !segment.is_empty())
            .collect();

        words.concat() == segments.concat()
            && Self::boundaries(segments.iter().copied())
                .is_subset(&Self::boundaries(words.iter().map(String::as_str)))
    }

    pub fn snake_case(&self) -> String {
        self.words().join(&WORD_SEPARATOR.to_string())
    }

    fn acronym_words(run: &[char]) -> Vec<&[char]> {
        run.iter()
            .position(|character| !character.is_uppercase())
            .filter(|tail| {
                run.get(*tail)
                    .is_some_and(|character| character.is_lowercase())
            })
            .and_then(|tail| tail.checked_sub(1))
            .filter(|start| *start > 0)
            .and_then(|start| run.split_at_checked(start))
            .map_or_else(|| vec![run], |(head, tail)| vec![head, tail])
    }

    fn boundaries<'word>(words: impl Iterator<Item = &'word str>) -> BTreeSet<usize> {
        let mut spelled = String::new();

        words
            .map(|word| {
                spelled.push_str(word);

                spelled.len()
            })
            .collect()
    }

    fn part_words(part: &str) -> Vec<String> {
        let characters: Vec<char> = part.chars().collect();

        characters
            .chunk_by(|previous, character| previous.is_uppercase() || !character.is_uppercase())
            .flat_map(Self::acronym_words)
            .map(|word| {
                word.iter()
                    .flat_map(|character| character.to_lowercase())
                    .collect()
            })
            .collect()
    }

    fn words(&self) -> Vec<String> {
        self.name
            .split(WORD_SEPARATOR)
            .filter(|part| !part.is_empty())
            .flat_map(Self::part_words)
            .collect()
    }
}
