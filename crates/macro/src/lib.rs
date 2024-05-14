use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn deduplicate_whitespace(input: TokenStream) -> TokenStream {
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
