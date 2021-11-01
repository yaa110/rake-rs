use serde::{Deserialize, Serialize};

/// Ranking metric
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Metric {
    /// d(w)/f(w) (Default metric) Ratio of degree of word to its frequency.
    DegreeToFrequencyRatio,

    /// d(w) Degree of word only. 
    WordDegree,

    /// f(w) Frequency of word only.
    WordFrequency,
}
