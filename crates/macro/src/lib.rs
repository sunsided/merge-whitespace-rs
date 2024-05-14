//! # merge_whitespace
//!
//! This crate contains procedural macros for removing multiple consecutive whitespaces from a
//! given string literal, replacing them with a single space.
//!
//! ## Example
//!
//! ```
//! use merge_whitespace::merge_whitespace;
//!
//! fn main() {
//!     let output = merge_whitespace!("Hello     World!\r\n      How        are         you?");
//!     assert_eq!(output, "Hello World! How are you?");
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// This is a procedural macro that removes multiple consecutive whitespaces from a given string
/// literal and replaces them with a single space.
///
/// ## Example
///
/// ```
/// use merge_whitespace::merge_whitespace;
///
/// fn main() {
///     let output = merge_whitespace!("Hello     World!\r\n      How        are         you?");
///     assert_eq!(output, "Hello World! How are you?");
/// }
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
    let output_str = input_str.split_whitespace().collect::<Vec<_>>().join(" ");

    // Generate the output tokens
    let output = quote! {
        #output_str
    };

    output.into()
}
