//! # RAKE.rs
//!
//! The library provides a multilingual implementation of [Rapid Automatic Keyword Extraction (RAKE)][1] algorithm for Rust.
//!
//! [1]: http://onlinelibrary.wiley.com/doi/10.1002/9780470689646.ch1/summary
//!
//! ## How to Use
//! - Append `rake` to `dependencies` of `Cargo.toml`:
//!
//! ```ignore
//! rake = "0.1"
//! ```
//!
//! - Import modules:
//!
//! ```ignore
//! extern crate rake;
//! use rake::*;
//! ```
//!
//! - Create a new instance of `Rake` struct:
//!
//! ```ignore
//! let text = "a long text";
//! let sw = StopWords::from_file("path/to/stop_words_list.txt").unwrap();
//! let r = Rake::new(sw);
//! let keywords = r.run(text);
//! ```
//!
//! - Iterate over keywords:
//!
//! ```ignore
//! keywords.iter().for_each(
//!     |&KeywordScore {
//!         ref keyword,
//!         ref score,
//!     }| println!("{}: {}", keyword, score),
//! );
//! ```
//!

#![doc(html_root_url = "https://docs.rs/rake/0.1")]
#![deny(missing_docs)]

extern crate regex;
#[macro_use]
extern crate lazy_static;

mod inner;
mod keyword;
mod rake;
mod stopwords;

pub use keyword::{KeywordScore, KeywordSort};
pub use rake::Rake;
pub use stopwords::StopWords;
