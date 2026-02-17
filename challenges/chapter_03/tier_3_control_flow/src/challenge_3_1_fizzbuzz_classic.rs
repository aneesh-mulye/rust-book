// Challenge 3.1 - FizzBuzz Classic (with for and if)
//
// Implement `fizzbuzz_1_to_30` so it returns 30 lines:
// - "FizzBuzz" for multiples of 15
// - "Fizz" for multiples of 3
// - "Buzz" for multiples of 5
// - otherwise the number itself
//
// This mirrors what would normally be printed from `main`.

pub fn fizzbuzz_1_to_30() -> Vec<String> {
    Vec::new()
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
    use super::fizzbuzz_1_to_30;

    #[test]
    fn returns_30_entries_for_1_through_30() {
        let output = fizzbuzz_1_to_30();
        assert_eq!(
            output.len(),
            30,
            "Expected exactly 30 entries for numbers 1 through 30, got {} entries.",
            output.len()
        );
    }

    #[test]
    fn applies_fizzbuzz_rules_at_key_positions() {
        let output = fizzbuzz_1_to_30();
        assert!(
            output.len() >= 30,
            "Need at least 30 entries to validate FizzBuzz positions, got {}.",
            output.len()
        );

        assert_eq!(
            output[0], "1",
            "Entry 1 should be '1' because it is not divisible by 3 or 5."
        );
        assert_eq!(
            output[2], "Fizz",
            "Entry 3 should be 'Fizz' because 3 is divisible by 3."
        );
        assert_eq!(
            output[4], "Buzz",
            "Entry 5 should be 'Buzz' because 5 is divisible by 5."
        );
        assert_eq!(
            output[14], "FizzBuzz",
            "Entry 15 should be 'FizzBuzz' because 15 is divisible by both 3 and 5."
        );
        assert_eq!(
            output[29], "FizzBuzz",
            "Entry 30 should be 'FizzBuzz' because 30 is divisible by both 3 and 5."
        );
    }
}
