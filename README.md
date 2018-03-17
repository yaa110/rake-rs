RAKE.rs
=======

[![crates.io](https://img.shields.io/crates/v/rake.svg)](https://crates.io/crates/rake) [![Documentation](https://img.shields.io/badge/Docs-iptables-blue.svg)](https://docs.rs/rake) [![Build Status](https://travis-ci.org/yaa110/rake-rs.svg)](https://travis-ci.org/yaa110/rake-rs) ![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg)

The library provides a multilingual implementation of [Rapid Automatic Keyword Extraction (RAKE)](http://onlinelibrary.wiley.com/doi/10.1002/9780470689646.ch1/summary) algorithm for Rust.

## How to Use
- Append `rake` to `dependencies` of `Cargo.toml`:

```toml
rake = "0.1"
```

- Import modules:

```rust
extern crate rake;
use rake::*;
```

- Create a new instance of `Rake` struct:

```rust
let text = "a long text";
let sw = StopWords::from_file("path/to/stop_words_list.txt").unwrap();
let r = Rake::new(sw);
let keywords = r.run(text);
```

- Iterate over keywords:

```rust
keywords.iter().for_each(
    |&KeywordScore {
        ref keyword,
        ref score,
    }| println!("{}: {}", keyword, score),
);
```
