// Challenge 3.1 - First Word
//
// Implement `first_word(s: &str) -> &str`:
// - Return slice up to first space.
// - If no space, return full input.

pub fn first_word(s: &str) -> &str {
    let _ = s;
    ""
}


// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn matches_prompt_examples() {
        assert_eq!(
            first_word("hello world"),
            "hello",
            "Expected first word 'hello' from 'hello world'."
        );
        assert_eq!(
            first_word("rust"),
            "rust",
            "Expected full string when no space exists in 'rust'."
        );
        assert_eq!(
            first_word("one two three"),
            "one",
            "Expected first word 'one' from 'one two three'."
        );
    }

    #[test]
    fn handles_edge_positions_for_spaces() {
        assert_eq!(
            first_word(" leading"),
            "",
            "If first character is a space, first word should be empty slice."
        );
        assert_eq!(
            first_word("trail "),
            "trail",
            "Trailing space should not affect first word extraction."
        );
    }
}
