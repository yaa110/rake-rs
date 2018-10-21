use std::collections::HashSet;
use std::convert::AsRef;
use std::convert::{From, Into};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::path::Path;

/// Represents a set of stop words
#[derive(Debug, Clone)]
pub struct StopWords(HashSet<String>);

impl From<HashSet<String>> for StopWords {
    fn from(set: HashSet<String>) -> StopWords {
        let mut sw = StopWords::new();
        set.into_iter().for_each(|w| {
            sw.insert(w);
        });
        sw
    }
}

impl Into<HashSet<String>> for StopWords {
    fn into(self) -> HashSet<String> {
        self.0
    }
}

impl Deref for StopWords {
    type Target = HashSet<String>;

    fn deref(&self) -> &HashSet<String> {
        &self.0
    }
}

impl DerefMut for StopWords {
    fn deref_mut(&mut self) -> &mut HashSet<String> {
        &mut self.0
    }
}

impl StopWords {
    /// Creates an empty instance of `StopWords`
    pub fn new() -> Self {
        StopWords(HashSet::new())
    }

    /// Inserts a new stop word to the set
    pub fn insert(&mut self, value: String) -> bool {
        self.0.insert(value.to_lowercase())
    }

    /// Creates a new instance of `StopWords` from the contents of file in `path`.
    /// Each line of the file must contain one stop word (lines started with `#` are ignored).
    /// The returned instance includes unique stop words.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut sw = StopWords::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let word = line?;
            if !word.is_empty() && !word.starts_with('#') {
                sw.insert(word);
            }
        }
        Ok(sw)
    }
}
