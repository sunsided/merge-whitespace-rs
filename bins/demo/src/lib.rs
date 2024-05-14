#[cfg(test)]
mod tests {
    use merge_whitespace::merge_whitespace;

    const OUTPUT: &str = merge_whitespace!("This   is   an\r\n  example  \t string.");

    #[test]
    fn test() {
        assert_eq!(OUTPUT, "This is an example string.");
    }
}
