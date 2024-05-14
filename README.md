# merge_whitespace

[![codecov](https://codecov.io/gh/sunsided/merge-whitespace-rs/graph/badge.svg?token=U6viefmywe)](https://codecov.io/gh/sunsided/merge-whitespace-rs)

This crate contains procedural macros for removing multiple consecutive whitespaces from a
given string literal, replacing them with a single space.

## Example

```rust
use merge_whitespace::merge_whitespace;

fn main() {
    let output = merge_whitespace!("Hello     World!\r\n      How        are         you?");
    assert_eq!(output, "Hello World! How are you?");
}
```
