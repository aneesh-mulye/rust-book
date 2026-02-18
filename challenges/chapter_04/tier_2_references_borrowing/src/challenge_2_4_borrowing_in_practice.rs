// Challenge 2.4 - Borrowing in Practice
//
// Implement `longest_word_length(text: &String) -> usize` by iterating over
// `text.as_bytes()` and counting contiguous non-space bytes.

pub fn longest_word_length(text: &String) -> usize {
    let _ = text;
    0
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
    use super::longest_word_length;

    #[test]
    fn matches_prompt_examples() {
        let a = String::from("the quick brown fox");
        let b = String::from("rust");

        assert_eq!(
            longest_word_length(&a),
            5,
            "Expected longest word length 5 for 'the quick brown fox'."
        );
        assert_eq!(
            longest_word_length(&b),
            4,
            "Expected longest word length 4 for 'rust'."
        );
        assert_eq!(
            a, "the quick brown fox",
            "Function should borrow immutably and leave original text unchanged."
        );
    }

    #[test]
    fn handles_multiple_spaces_and_edges() {
        let text = String::from("  rust   book  ");
        assert_eq!(
            longest_word_length(&text),
            4,
            "Multiple spaces should not create fake words. Expected 4 for longest word in '  rust   book  '."
        );

        let empty = String::new();
        assert_eq!(
            longest_word_length(&empty),
            0,
            "Empty string should produce longest length 0."
        );
    }
}
