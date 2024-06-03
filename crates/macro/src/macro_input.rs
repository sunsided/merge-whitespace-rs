use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, LitStr, Token};

/// Input for the whitespace merging macro.
pub struct MacroInput {
    /// The input string to merge whitespaces in.
    pub string: LitStr,
    /// The optional quote character to use.
    pub quote_char: Option<char>,
    /// The optional escape character to use.
    pub escape_char: Option<char>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let string = input.parse()?;
        let mut quote_char = None;
        let mut escape_char = None;

        while !input.is_empty() {
            input.parse::<Token![,]>()?;

            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                match &*ident.to_string() {
                    "quote_char" => {
                        input.parse::<Token![=]>()?;
                        let expr: Expr = input.parse()?;
                        if let Expr::Lit(expr_lit) = expr {
                            if let syn::Lit::Char(lit_char) = expr_lit.lit {
                                quote_char = Some(lit_char.value());
                            } else {
                                return Err(input.error("Expected a char literal for quote_char"));
                            }
                        } else {
                            return Err(input.error("Expected a char literal for quote_char"));
                        }
                    }
                    "escape_char" => {
                        input.parse::<Token![=]>()?;
                        let expr: Expr = input.parse()?;
                        if let Expr::Lit(expr_lit) = expr {
                            if let syn::Lit::Char(lit_char) = expr_lit.lit {
                                escape_char = Some(lit_char.value());
                            } else {
                                return Err(input.error("Expected a char literal for escape_char"));
                            }
                        } else {
                            return Err(input.error("Expected a char literal for escape_char"));
                        }
                    }
                    _ => {
                        return Err(input.error("Expected 'quote_char' or 'escape_char' identifier"))
                    }
                }
            } else {
                let expr: Expr = input.parse()?;
                if let Expr::Lit(expr_lit) = expr {
                    if quote_char.is_none() {
                        if let syn::Lit::Char(lit_char) = expr_lit.lit {
                            quote_char = Some(lit_char.value());
                        } else {
                            return Err(input.error("Expected a char literal for quote_char"));
                        }
                    } else if escape_char.is_none() {
                        if let syn::Lit::Char(lit_char) = expr_lit.lit {
                            escape_char = Some(lit_char.value());
                        } else {
                            return Err(input.error("Expected a char literal for escape_char"));
                        }
                    } else {
                        return Err(input.error("Unexpected additional positional argument"));
                    }
                } else {
                    return Err(input.error("Expected a char literal for positional argument"));
                }
            }
        }

        Ok(MacroInput {
            string,
            quote_char,
            escape_char,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_positional_quote_char() {
        let input: MacroInput = parse_str(r#""Test string", '"' "#).unwrap();
        assert_eq!(input.string.value(), "Test string");
        assert_eq!(input.quote_char, Some('"'));
        assert_eq!(input.escape_char, None);
    }

    #[test]
    fn test_named_quote_char() {
        let input: MacroInput = parse_str(r#""Test string", quote_char = '"'"#).unwrap();
        assert_eq!(input.string.value(), "Test string");
        assert_eq!(input.quote_char, Some('"'));
        assert_eq!(input.escape_char, None);
    }

    #[test]
    fn test_positional_quote_and_escape_char() {
        let input: MacroInput = parse_str(r#""Test string", '"', '\\'"#).unwrap();
        assert_eq!(input.string.value(), "Test string");
        assert_eq!(input.quote_char, Some('"'));
        assert_eq!(input.escape_char, Some('\\'));
    }

    #[test]
    fn test_named_quote_and_escape_char() {
        let input: MacroInput =
            parse_str(r#""Test string", quote_char = '"', escape_char = '\\'"#).unwrap();
        assert_eq!(input.string.value(), "Test string");
        assert_eq!(input.quote_char, Some('"'));
        assert_eq!(input.escape_char, Some('\\'));
    }

    #[test]
    fn test_named_escape_and_quote_char() {
        let input: MacroInput =
            parse_str(r#""Test string", escape_char = '\\', quote_char = '"'"#).unwrap();
        assert_eq!(input.string.value(), "Test string");
        assert_eq!(input.quote_char, Some('"'));
        assert_eq!(input.escape_char, Some('\\'));
    }

    #[test]
    fn test_named_escape_char_only() {
        let input: MacroInput = parse_str(r#""Test string", escape_char = '\\'"#).unwrap();
        assert_eq!(input.string.value(), "Test string");
        assert_eq!(input.quote_char, None);
        assert_eq!(input.escape_char, Some('\\'));
    }

    #[test]
    fn test_invalid_input() {
        // Invalid inputs with named arguments
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = var, escape_char = '\\'"#)
                .is_err()
        );
        assert!(parse_str::<MacroInput>(
            r#""Test string", quote_char = foo[0], escape_char = '\\'"#
        )
        .is_err());
        assert!(parse_str::<MacroInput>(
            r#""Test string", quote_char = "car", escape_char = '\\'"#
        )
        .is_err());
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = 12, escape_char = '\\'"#)
                .is_err()
        );
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = #, escape_char = '\\'"#)
                .is_err()
        );
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = '"', escape_char = var"#)
                .is_err()
        );
        assert!(parse_str::<MacroInput>(
            r#""Test string", quote_char = '"', escape_char = foo[0]"#
        )
        .is_err());
        assert!(parse_str::<MacroInput>(
            r#""Test string", quote_char = '"', escape_char = "quote""#
        )
        .is_err());
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = '"', escape_char = 12"#)
                .is_err()
        );

        // Invalid inputs with positional arguments
        assert!(parse_str::<MacroInput>(r#""Test string", "car", '\\'"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", 12, '\\'"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", #, '\\'"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", var, '\\'"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", foo[0], '\\'"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", a = b, '\\'"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", '"', "quote""#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", '"', 12"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", '"', var"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", '"', foo[0]"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", '"', a = b"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", foo = bar, a = b"#).is_err());
        assert!(parse_str::<MacroInput>(r#""Test string", "failure"#).is_err());

        // Missing comma
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = '"' escape_char = '\\'"#)
                .is_err()
        );
        assert!(
            parse_str::<MacroInput>(r#""Test string" quote_char = '"', escape_char = '\\'"#)
                .is_err()
        );

        // Invalid argument
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_chars = '"', escape_char = '\\'"#)
                .is_err()
        );
        assert!(
            parse_str::<MacroInput>(r#""Test string", quote_char = '"', escape_chars = '\\'"#)
                .is_err()
        );

        // Too many arguments
        assert!(parse_str::<MacroInput>(r#""Test string", '"', '\\', 42"#).is_err());
        assert!(parse_str::<MacroInput>(
            r#""Test string", quote_char = '"', escape_char = '\\', invalid = true"#
        )
        .is_err());
    }
}
