// Challenge 2.2 - Statements vs Expressions
//
// Implement `last_digit_squared(n)` so it:
// 1. Extracts the last digit using `% 10`.
// 2. Uses absolute value so negatives work.
// 3. Returns the square of that digit.
//
// Challenge note: try to keep the body as a single expression.

pub fn last_digit_squared(n: i32) -> i32 {
    let _ = n;
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
    use super::last_digit_squared;

    #[test]
    fn handles_examples_from_prompt() {
        assert_eq!(
            last_digit_squared(1984),
            16,
            "1984 ends in 4, so expected 4^2 = 16."
        );
        assert_eq!(
            last_digit_squared(-37),
            49,
            "-37 ends in 7 (by absolute value), so expected 7^2 = 49."
        );
        assert_eq!(
            last_digit_squared(100),
            0,
            "100 ends in 0, so expected 0^2 = 0."
        );
    }

    #[test]
    fn handles_additional_edge_cases() {
        assert_eq!(
            last_digit_squared(-1),
            1,
            "-1 should produce 1 because last digit magnitude is 1."
        );
        assert_eq!(
            last_digit_squared(0),
            0,
            "0 should produce 0."
        );
    }
}
