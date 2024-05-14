# merge_whitespace

[![codecov](https://codecov.io/gh/sunsided/merge-whitespace-rs/graph/badge.svg?token=U6viefmywe)](https://codecov.io/gh/sunsided/merge-whitespace-rs)

This crate contains procedural macros for removing multiple consecutive whitespaces from a
given string literal, replacing them with a single space.

## Example

```rust
use merge_whitespace::merge_whitespace;

const QUERY: &str = merge_whitespace!(r#"
                query {
                  users (limit: 1) {
                    id
                    name
                    todos(order_by: {created_at: desc}, limit: 5) {
                      id
                      title
                    }
                  }
                }
                "#);

#[test]
fn test() {
    assert_eq!(QUERY, "query { users (limit: 1) { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }");
}
```
