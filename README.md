# merge_whitespace

[![Crates.io](https://img.shields.io/crates/v/merge-whitespace)](https://crates.io/crates/merge-whitespace)
[![Crates.io](https://img.shields.io/crates/l/merge-whitespace)](https://crates.io/crates/merge-whitespace)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/merge-whitespace-rs/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/merge-whitespace)](https://docs.rs/merge-whitespace/)
[![codecov](https://codecov.io/gh/sunsided/merge-whitespace-rs/graph/badge.svg?token=U6viefmywe)](https://codecov.io/gh/sunsided/merge-whitespace-rs)


This crate contains procedural macros for removing multiple consecutive whitespaces from a
given string literal, replacing them with a single space.

## Example

The example below uses an optional quotation character to keep quoted text ranges un-merged, as well as
an optional escape character to ensure that quotation character literals are kept as-is.

```rust
use merge_whitespace::merge_whitespace;

const QUERY: &str = merge_whitespace!(r#"
     query {
       users (limit: 1, filter: "bought a 12\" vinyl
                                 named \"spaces  in  space \"") {
         id
         name
         todos(order_by: {created_at: desc}, limit: 5) {
           id
           title
         }
       }
     }
     "#,
     quote_char = '"',
     escape_char = '\\');

#[test]
fn test() {
    assert_eq!(QUERY, r#"query { users (limit: 1, filter: "bought a 12\" vinyl
                                 named \"spaces  in  space \"") { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }"#);
}
```
