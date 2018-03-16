//! # RAKE.rs
//!
//! The library provides a multilingual implementation of [Rapid Automatic Keyword Extraction (RAKE)][1] algorithm for Rust.
//!
//! [1]: http://onlinelibrary.wiley.com/doi/10.1002/9780470689646.ch1/summary

#![doc(html_root_url = "https://docs.rs/rake/0.1")]
#![deny(missing_docs)]

extern crate regex;

mod rake;
mod stopwords;
mod keyword;
mod inner;

pub use rake::Rake;
pub use stopwords::StopWords;
pub use keyword::{KeywordScore, KeywordSort};
