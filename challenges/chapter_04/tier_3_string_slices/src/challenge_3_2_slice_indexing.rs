// Challenge 3.2 - Slice Indexing
//
// Implement `slice_parts` to return:
// 1. first 3 bytes/characters
// 2. last 4 bytes/characters
// 3. everything except the first and last byte/character
//
// This challenge assumes ASCII input so byte indices are valid character boundaries.

pub fn slice_parts(s: &str) -> (&str, &str, &str) {
    (&s[..3], &s[(s.len() - 4)..], &s[1..(s.len() - 1)])
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
    use super::slice_parts;

    #[test]
    fn matches_prompt_example() {
        let (first3, last4, middle) = slice_parts("Rustacean");

        assert_eq!(first3, "Rus", "First 3 for 'Rustacean' should be 'Rus'.");
        assert_eq!(last4, "cean", "Last 4 for 'Rustacean' should be 'cean'.");
        assert_eq!(
            middle, "ustacea",
            "Middle for 'Rustacean' should be 'ustacea'."
        );
    }

    #[test]
    fn works_for_another_ascii_input() {
        let (first3, last4, middle) = slice_parts("abcdefg");

        assert_eq!(first3, "abc", "First 3 for 'abcdefg' should be 'abc'.");
        assert_eq!(last4, "defg", "Last 4 for 'abcdefg' should be 'defg'.");
        assert_eq!(middle, "bcdef", "Middle for 'abcdefg' should be 'bcdef'.");
    }
}
