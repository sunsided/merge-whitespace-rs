#[cfg(test)]
mod tests {
    use const_whitespace_compressor::deduplicate_whitespace;

    const OUTPUT: &str = deduplicate_whitespace!("This   is   an  example   string.");

    #[test]
    fn test(){
        assert_eq!(OUTPUT, "This is an example string.");
    }
}
