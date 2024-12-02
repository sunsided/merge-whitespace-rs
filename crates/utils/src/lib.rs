//! # merge-whitespace-utils
//!
//! This crate contains the [`merge_whitespace`] and [`merge_whitespace_with_quotes`] functions
//! for removing multiple consecutive whitespaces from a given string, replacing them with a single space.
//!
//! ## Example
//!
//! ```
//! # use merge_whitespace_utils::merge_whitespace_with_quotes;
//! let query = merge_whitespace_with_quotes(r#"
//!                 query {
//!                   users (limit: 1, filter: "bought a 12\" vinyl
//!                                             named \"spaces  in  space \"") {
//!                     id
//!                     name
//!                     todos(order_by: {created_at: desc}, limit: 5) {
//!                       id
//!                       title
//!                     }
//!                   }
//!                 }
//!                 "#,
//!                 Some('"'),
//!                 Some('\\'));
//!
//! assert_eq!(query, r#"query { users (limit: 1, filter: "bought a 12\" vinyl
//!                                             named \"spaces  in  space \"") { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }"#);
//! ```

#![forbid(unsafe_code)]

use std::borrow::Cow;

/// Remove multiple consecutive whitespaces from a given string and replace them with a single space.
/// If special handling of quoted text is required, see [`merge_whitespace_with_quotes`] instead.
///
/// ## Example
///
/// ```
/// # use merge_whitespace_utils::merge_whitespace;
/// let output = merge_whitespace("Hello     World!\r\n      \"How        are\"         you?");
/// assert_eq!(output, r#"Hello World! "How are" you?"#);
/// ```
///
/// # Return
///
/// The modified string.
pub fn merge_whitespace(input: &str) -> Cow<str> {
    merge_whitespace_with_quotes(input, None, None)
}

/// Remove multiple consecutive whitespaces from a given string literal and replace them with a
/// single space. Quoted text will be ignored and kept as-is.
///
/// ## Example
///
/// ```
/// # use merge_whitespace_utils::merge_whitespace_with_quotes;
/// let output = merge_whitespace_with_quotes("Hello     World!\r\n      \"How        are\"         you?", Some('"'), None);
/// assert_eq!(output, "Hello World! \"How        are\" you?");
/// ```
///
/// # Return
///
/// The modified string.
pub fn merge_whitespace_with_quotes(
    input: &str,
    quote_char: Option<char>,
    escape_char: Option<char>,
) -> Cow<str> {
    let trimmed_input = input.trim();
    let mut result = None; // Use this to lazily initialize a String if needed
    let mut in_quotes = false;
    let mut prev_char_was_space = false;
    let mut in_escape = false;

    for c in trimmed_input.chars() {
        if escape_char == Some(c) && !in_escape {
            if prev_char_was_space {
                result
                    .get_or_insert_with(|| String::with_capacity(trimmed_input.len()))
                    .push(' ');
            }
            prev_char_was_space = false;
            in_escape = true;
            result
                .get_or_insert_with(|| String::with_capacity(trimmed_input.len()))
                .push(c);
            continue;
        }
        if c.is_whitespace() && !in_quotes && !in_escape {
            prev_char_was_space = true;
            continue;
        }
        if quote_char == Some(c) && !in_escape {
            in_quotes = !in_quotes;
        }
        if prev_char_was_space {
            result
                .get_or_insert_with(|| String::with_capacity(trimmed_input.len()))
                .push(' ');
        }
        result
            .get_or_insert_with(|| String::with_capacity(trimmed_input.len()))
            .push(c);
        prev_char_was_space = false;
        in_escape = false;
    }

    match result {
        Some(resulting_string) => Cow::Owned(resulting_string),
        None => Cow::Borrowed(trimmed_input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUOTE: Option<char> = Some('"');
    const ESCAPE: Option<char> = Some('\\');

    #[test]
    fn whitespace_only_is_trimmed() {
        assert_eq!(merge_whitespace_with_quotes("  ", QUOTE, None), "");
        assert_eq!(merge_whitespace_with_quotes("  \n \t  ", QUOTE, None), "");
    }

    #[test]
    fn non_whitespace_is_ignored() {
        assert_eq!(
            merge_whitespace_with_quotes("abcdefgh.ihkl-", QUOTE, None),
            "abcdefgh.ihkl-"
        );
    }

    #[test]
    fn single_whitespace_in_text_is_kept() {
        assert_eq!(
            merge_whitespace_with_quotes("foo bar baz", QUOTE, None),
            "foo bar baz"
        );
    }

    #[test]
    fn multiple_whitespace_in_text_is_merged() {
        assert_eq!(
            merge_whitespace_with_quotes("foo  bar\nbaz", QUOTE, None),
            "foo bar baz"
        );
    }

    #[test]
    fn quoted_whitespace_in_text_is_kept() {
        assert_eq!(
            merge_whitespace_with_quotes("foo   foobar   \"  bar\n\" baz", QUOTE, None),
            "foo foobar \"  bar\n\" baz"
        );
    }

    #[test]
    fn escape_a_space() {
        assert_eq!(
            merge_whitespace_with_quotes("what   \\   if I quote\\ spaces", QUOTE, ESCAPE),
            "what \\  if I quote\\ spaces"
        );
    }

    #[test]
    fn quoted_whitespace_with_escaped_quotes() {
        assert_eq!(
            merge_whitespace_with_quotes(
                r#"foo   foobar   "  \"bar   \"   "   baz"#,
                QUOTE,
                ESCAPE
            ),
            r#"foo foobar "  \"bar   \"   " baz"#
        );
    }

    #[test]
    fn test_complex_escaped() {
        let result = merge_whitespace_with_quotes(
            r#"
                query {
                  users (limit: 1, name: "Froozle   '78\"'   Frobnik") {
                    id
                    name
                    todos(order_by: {created_at: desc}, limit: 5) {
                      id
                      title
                    }
                  }
                }
                "#,
            QUOTE,
            ESCAPE,
        );
        assert_eq!(result, "query { users (limit: 1, name: \"Froozle   '78\\\"'   Frobnik\") { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }");
    }

    #[test]
    fn test_complex_unescaped() {
        let result = merge_whitespace_with_quotes(
            r#"
                query {
                  users (limit: 1, name: "Froozle   Frobnik") {
                    id
                    name
                    todos(order_by: {created_at: desc}, limit: 5) {
                      id
                      title
                    }
                  }
                }
                "#,
            QUOTE,
            None,
        );
        assert_eq!(result, "query { users (limit: 1, name: \"Froozle   Frobnik\") { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }");
    }
}
