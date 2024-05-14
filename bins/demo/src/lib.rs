#[cfg(test)]
mod tests {
    use merge_whitespace::merge_whitespace;

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
}
