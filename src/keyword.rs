use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// An interface to sort a vector of keywords by score
pub trait KeywordSort {
    /// Reverse sort by score of keyword from greater to less
    fn sort_by_score(&mut self);
}

/// Represents a keyword score
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeywordScore {
    /// The keyword
    pub keyword: String,

    /// The score of keyword
    pub score: f64,
}

impl KeywordScore {
    /// Creates a vector of `KeywordScore` from `mp`
    pub fn from_map(mp: impl IntoIterator<Item = (String, f64)>) -> Vec<Self> {
        mp.into_iter()
            .map(|(kw, score)| KeywordScore { keyword: kw, score })
            .collect()
    }
}

impl Ord for KeywordScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(Ordering::Less)
    }
}

impl PartialOrd for KeywordScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for KeywordScore {}

impl PartialEq for KeywordScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl KeywordSort for Vec<KeywordScore> {
    fn sort_by_score(&mut self) {
        self.sort_by(|a, b| b.cmp(a));
    }
}
