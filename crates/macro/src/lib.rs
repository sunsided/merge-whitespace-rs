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
//!                   users (limit: 1) {
//!                     id
//!                     name
//!                     todos(order_by: {created_at: desc}, limit: 5) {
//!                       id
//!                       title
//!                     }
//!                   }
//!                 }
//!                 "#);
//!
//! assert_eq!(QUERY, "query { users (limit: 1) { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }");
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Token, Expr};
use syn::parse::{Parse, ParseStream};

/// This is a procedural macro that removes multiple consecutive whitespaces from a given string
/// literal and replaces them with a single space.
///
/// ## Example
///
/// ```
/// # use merge_whitespace::merge_whitespace;
/// let output = merge_whitespace!("Hello     World!\r\n      How        are         you?");
/// assert_eq!(output, "Hello World! How are you?");
/// ```
///
/// # Return
///
/// The macro expands to the modified string literal.
#[proc_macro]
pub fn merge_whitespace(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as LitStr);

    // Get the string literal value
    let input_str = input.value();

    // Replace multiple whitespaces with a single space
    let output_str = merge_whitespace_segment(&input_str);

    // Generate the output tokens
    let output = quote! {
        #output_str
    };

    output.into()
}

/// This is a procedural macro that removes multiple consecutive whitespaces from a given string
/// literal and replaces them with a single space. Quoted text will be ignored and kept as-is.
///
/// ## Example
///
/// ```
/// # use merge_whitespace::merge_whitespace_quoted;
/// let output = merge_whitespace_quoted!("Hello     World!\r\n      \"How        are\"         you?");
/// assert_eq!(output, r#"Hello World! "How        are" you?"#);
/// ```
///
/// Alternatively, you can specify the character used for quotation:
///
/// ```
/// # use merge_whitespace::merge_whitespace_quoted;
/// let output = merge_whitespace_quoted!("Hello     World!\r\n      'How        are'         you?", '\'');
/// assert_eq!(output, "Hello World! 'How        are' you?");
/// ```
///
/// # Return
///
/// The macro expands to the modified string literal.
#[proc_macro]
pub fn merge_whitespace_quoted(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as MacroInput);

    let input_str = input.string.value();
    let quote_char = input.quote_char.unwrap_or('"');

    // Replace multiple whitespaces with a single space, skipping quoted blocks
    let output_str = merge_whitespace_with_quotes(&input_str, quote_char);

    // Generate the output tokens
    let output = quote! {
        #output_str
    };

    output.into()
}

struct MacroInput {
    string: LitStr,
    quote_char: Option<char>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let string = input.parse()?;
        let quote_char = if input.is_empty() {
            None
        } else {
            input.parse::<Token![,]>()?;
            let expr: Expr = input.parse()?;
            if let Expr::Lit(expr_lit) = expr {
                if let syn::Lit::Char(lit_char) = expr_lit.lit {
                    Some(lit_char.value())
                } else {
                    return Err(input.error("Expected a char literal"));
                }
            } else {
                return Err(input.error("Expected a char literal"));
            }
        };
        Ok(MacroInput { string, quote_char })
    }
}


fn merge_whitespace_with_quotes(input: &str, quote_char: char) -> String {
    let input = input.trim();
    let mut result = String::with_capacity(input.len());
    let mut in_quotes = false;
    let mut prev_char_was_space = false;

    for c in input.chars() {
        if c.is_whitespace() && !in_quotes {
            prev_char_was_space = true;
            continue;
        }

        if c == quote_char {
            in_quotes = !in_quotes;
        }

        if prev_char_was_space {
            result.push(' ');
        }

        prev_char_was_space = false;
        result.push(c);
    }

    result
}

fn merge_whitespace_segment(segment: &str) -> String {
    segment.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUOTE: char = '"';

    #[test]
    fn whitespace_only_is_trimmed() {
        assert_eq!(merge_whitespace_with_quotes("  ", QUOTE), "");
        assert_eq!(merge_whitespace_with_quotes("  \n \t  ", QUOTE), "");
    }

    #[test]
    fn non_whitespace_is_ignored() {
        assert_eq!(
            merge_whitespace_with_quotes("abcdefgh.ihkl-", QUOTE),
            "abcdefgh.ihkl-"
        );
    }

    #[test]
    fn single_whitespace_in_text_is_kept() {
        assert_eq!(
            merge_whitespace_with_quotes("foo bar baz", QUOTE),
            "foo bar baz"
        );
    }

    #[test]
    fn multiple_whitespace_in_text_is_merged() {
        assert_eq!(
            merge_whitespace_with_quotes("foo  bar\nbaz", QUOTE),
            "foo bar baz"
        );
    }

    #[test]
    fn quoted_whitespace_in_text_is_kept() {
        assert_eq!(
            merge_whitespace_with_quotes("foo   foobar   \"  bar\n\" baz", QUOTE),
            "foo foobar \"  bar\n\" baz"
        );
    }
}
