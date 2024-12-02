//! # merge-whitespace
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

#![forbid(unsafe_code)]

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
    let output_str =
        merge_whitespace_utils::merge_whitespace_with_quotes(&input_str, quote_char, escape_char);

    // Generate the output tokens
    let output = quote! {
        #output_str
    };

    output.into()
}
