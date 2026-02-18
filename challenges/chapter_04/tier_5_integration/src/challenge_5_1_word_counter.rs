// Challenge 5.1 - Word Counter
//
// Implement:
// - `word_count(text: &str) -> u32`
// - `char_count(text: &str) -> u32` (non-space chars only)
// - `average_word_length(text: &str) -> f64`

pub fn word_count(text: &str) -> u32 {
    let _ = text;
    0
}

pub fn char_count(text: &str) -> u32 {
    let _ = text;
    0
}

pub fn average_word_length(text: &str) -> f64 {
    let _ = text;
    0.0
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
    use super::{average_word_length, char_count, word_count};

    #[test]
    fn matches_prompt_counts() {
        let text = "the rust programming language is great";

        let words = word_count(text);
        let chars = char_count(text);
        let avg = average_word_length(text);
        let expected_avg = 32.0 / 6.0;
        let diff = (avg - expected_avg).abs();

        assert_eq!(words, 6, "Expected 6 words in prompt sentence. Got {words}.");
        assert_eq!(chars, 32, "Expected 32 non-space characters. Got {chars}.");
        assert!(
            diff < 1e-10,
            "Expected average word length {} (about 5.3), got {} (diff {}).",
            expected_avg,
            avg,
            diff
        );
    }

    #[test]
    fn handles_extra_spaces_without_counting_empty_words() {
        let text = "  rust   book  ";

        assert_eq!(
            word_count(text),
            2,
            "Consecutive/edge spaces should not create empty words. Expected 2 words."
        );
        assert_eq!(
            char_count(text),
            8,
            "Non-space character count for '  rust   book  ' should be 8."
        );
    }

    #[test]
    fn empty_input_has_zero_average() {
        let text = "";
        assert_eq!(word_count(text), 0, "Empty text should have 0 words.");
        assert_eq!(char_count(text), 0, "Empty text should have 0 non-space chars.");
        assert_eq!(
            average_word_length(text),
            0.0,
            "Average word length for empty text should be 0.0 to avoid division-by-zero behavior."
        );
    }
}
