use std::{collections::hash_map::RandomState, hash::BuildHasher};

/// Iconic lines evoking Ghost in the Shell (1995), the project's inspiration.
///
/// The collection is a fixed catalog of memorable lines from the film. Callers
/// pick one deterministically with `select`, or let `random` vary the line
/// between application runs.
pub struct SplashQuotes;

impl SplashQuotes {
    const LINES: [&'static str; 6] = [
        "the net is vast and infinite",
        "overspecialize, and you breed in weakness",
        "a life born in the sea of information",
        "for now we see through a glass, darkly",
        "memory cannot be defined, yet it defines us",
        "your effort to remain what you are limits you",
    ];

    /// Returns the line at `index`, wrapping around the end of the catalog.
    pub fn select(index: usize) -> &'static str {
        Self::LINES[index % Self::LINES.len()]
    }

    /// Returns a line chosen from system entropy, varying between runs.
    pub fn random() -> &'static str {
        Self::select(Self::entropy_index())
    }

    fn entropy_index() -> usize {
        RandomState::new().hash_one(0u8) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_wraps_around_the_catalog() {
        let count = SplashQuotes::LINES.len();

        let wrapped = SplashQuotes::select(count);

        assert_eq!(wrapped, SplashQuotes::select(0));
    }

    #[test]
    fn select_returns_distinct_lines_across_the_catalog() {
        let first = SplashQuotes::select(0);

        let second = SplashQuotes::select(1);

        assert_ne!(first, second);
    }
}
