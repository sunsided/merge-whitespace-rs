#[cfg(test)]
mod tests {
    use merge_whitespace::*;

    const OUTPUT: &str = merge_whitespace!("This   is   an\r\n  example  \t string.");

    #[test]
    fn test_const() {
        assert_eq!(OUTPUT, "This is an example string.");
    }

    #[test]
    fn test_complex() {
        const QUERY: &str = merge_whitespace!(
            r#"
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
                "#
        );
        assert_eq!(QUERY, "query { users (limit: 1) { id name todos(order_by: {created_at: desc}, limit: 5) { id title } } }");
    }

    #[test]
    fn test_quoted() {
        let output =
            merge_whitespace!("Hello     World!\r\n      \"How        are\"         you?", '"');
        assert_eq!(output, r#"Hello World! "How        are" you?"#);

        let output = merge_whitespace!("\"Nothing to  see   here    \"", '"');
        assert_eq!(output, r#""Nothing to  see   here    ""#);

        let output = merge_whitespace!(" \"Nothing to  see   here    \" ", '"');
        assert_eq!(output, r#""Nothing to  see   here    ""#);

        let output = merge_whitespace!("Test:\"Nothing to  see   here    \" ", '"');
        assert_eq!(output, r#"Test:"Nothing to  see   here    ""#);

        let output = merge_whitespace!("Test: \"Nothing to  see   here    \" ", '"');
        assert_eq!(output, r#"Test: "Nothing to  see   here    ""#);

        let output =
            merge_whitespace!("Hello     World!\r\n      'How        are'         you?");
        assert_eq!(output, "Hello World! 'How are' you?");
    }

    #[test]
    fn test_unquoted() {
        let output =
            merge_whitespace!("Hello     World!\r\n      \"How        are\"         you?");
        assert_eq!(output, r#"Hello World! "How are" you?"#);

        let output =
            merge_whitespace!("Hello     World!\r\n      'How        are'         you?");
        assert_eq!(output, "Hello World! 'How are' you?");
    }
}
