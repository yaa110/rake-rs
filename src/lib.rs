//! # RAKE.rs
//!
//! The library provides a multilingual implementation of [Rapid Automatic Keyword Extraction (RAKE)][1] algorithm for Rust.
//!
//! [1]: http://onlinelibrary.wiley.com/doi/10.1002/9780470689646.ch1/summary
//!
//! ## How to Use
//! - Add `rake` to the `dependencies` of your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rake = "0.1"
//! ```
//!
//! If you're using Rust 2015, then you'll also need to add it to your crate root:
//!
//! ```rust
//! extern crate rake;
//! ```
//!
//! ### Example
//!
//! ```
//! // Import modules
//! use rake::*;
//!
//! // Create a new instance of `Rake` struct
//! let text = "a long text";
//! let stop_words_list_path = "tests/SmartStoplist.txt";
//! let sw = StopWords::from_file(stop_words_list_path).unwrap();
//! let r = Rake::new(sw);
//! let keywords = r.run(text);
//!
//! // Iterate over keywords
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

mod inner;
mod keyword;
mod rake;
mod stopwords;

pub use keyword::{KeywordScore, KeywordSort};
pub use rake::Rake;
pub use stopwords::StopWords;
