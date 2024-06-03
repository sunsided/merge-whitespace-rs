//! # merge_whitespace
//!
//! This crate contains procedural macros for removing multiple consecutive whitespaces from a
//! given string literal, replacing them with a single space.
//!
//! ## Example
//!
//! ```
//! # use merge_whitespace::merge_whitespace;
//! const QUERY: &str = merge_whitespace!(r#"
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
//!                 quote_char = '"',
//!                 escape_char = '\\');
//!
//! assert_eq!(QUERY, r#"query { users (limit: 1, filter: "bought a 12\" vinyl
//!                                             named \"spaces  in  space \"") { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }"#);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::macro_input::MacroInput;

mod macro_input;

/// This is a procedural macro that removes multiple consecutive whitespaces from a given string
/// literal and replaces them with a single space. Quoted text will be ignored and kept as-is.
///
/// ## Example
///
/// ```
/// # use merge_whitespace::merge_whitespace;
/// let output = merge_whitespace!("Hello     World!\r\n      \"How        are\"         you?");
/// assert_eq!(output, r#"Hello World! "How are" you?"#);
/// ```
///
/// If you want to keep quoted text as is, you can specify a quotation mark character.
/// Everything within a pair of these markers is kept as-is:
///
/// ```
/// # use merge_whitespace::merge_whitespace;
/// let output = merge_whitespace!("Hello     World!\r\n      \"How        are\"         you?", quote_char = '"');
/// assert_eq!(output, "Hello World! \"How        are\" you?");
/// ```
///
/// # Return
///
/// The macro expands to the modified string literal.
#[proc_macro]
pub fn merge_whitespace(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as MacroInput);

    let input_str = input.string.value();
    let quote_char = input.quote_char;
    let escape_char = input.escape_char;

    // Replace multiple whitespaces with a single space, skipping quoted blocks
    let output_str = merge_whitespace_with_quotes(&input_str, quote_char, escape_char);

    // Generate the output tokens
    let output = quote! {
        #output_str
    };

    output.into()
}

fn merge_whitespace_with_quotes(
    input: &str,
    quote_char: Option<char>,
    escape_char: Option<char>,
) -> String {
    let input = input.trim();
    let mut result = String::with_capacity(input.len());
    let mut in_quotes = false;
    let mut prev_char_was_space = false;
    let mut in_escape = false;

    for c in input.chars() {
        // If we hit an escape character, purge any previous spaces, then skip over it.
        if escape_char == Some(c) && !in_escape {
            if prev_char_was_space {
                result.push(' ');
            }

            prev_char_was_space = false;
            in_escape = true;
            result.push(c);
            continue;
        }

        // If we hit a whitespace, buffer it unless it is escaped or within quotes.
        if c.is_whitespace() && !in_quotes && !in_escape {
            prev_char_was_space = true;
            in_escape = false;
            continue;
        }

        if quote_char == Some(c) && !in_escape {
            in_quotes = !in_quotes;
        }

        if prev_char_was_space {
            result.push(' ');
        }

        prev_char_was_space = false;
        in_escape = false;
        result.push(c);
    }

    result
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
