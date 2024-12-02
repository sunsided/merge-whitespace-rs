#![no_main]

use libfuzzer_sys::fuzz_target;
use merge_whitespace_utils::{merge_whitespace, merge_whitespace_with_quotes};

fuzz_target!(|data: &[u8]| {
    // Ensure input is long enough to extract quote and escape characters
    if data.len() < 2 {
        return;
    }

    // First byte: quote character (if printable)
    let quote_char = match data[0].is_ascii_graphic() {
        true => Some(data[0] as char),
        false => None,
    };

    // Second byte: escape character (if printable)
    let escape_char = match data[1].is_ascii_graphic() {
        true => Some(data[1] as char),
        false => None,
    };

    // Remaining bytes: string to process
    let string_input = &data[2..];

    // Ensure the remaining input is valid UTF-8
    if let Ok(input) = std::str::from_utf8(string_input) {
        // Test with dynamic quote and escape characters
        let _ = merge_whitespace_with_quotes(input, quote_char, escape_char);

        // Test with default behavior (no quote or escape chars)
        let _ = merge_whitespace(input);
    }
});
