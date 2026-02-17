// Challenge 2.3 - Multi-Return with Tuples
//
// Implement `divide(dividend, divisor)` to return `(quotient, remainder)`.
// If `divisor == 0`, return `(0, 0)`.

pub fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    let _ = (dividend, divisor);
    (0, 0)
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
    use super::divide;

    #[test]
    fn handles_examples_from_prompt() {
        assert_eq!(
            divide(17, 5),
            (3, 2),
            "17 / 5 should be quotient 3 with remainder 2."
        );
        assert_eq!(
            divide(10, 3),
            (3, 1),
            "10 / 3 should be quotient 3 with remainder 1."
        );
        assert_eq!(
            divide(7, 0),
            (0, 0),
            "Division by zero should return (0, 0) per challenge requirements."
        );
    }

    #[test]
    fn follows_rust_integer_division_sign_rules() {
        assert_eq!(
            divide(-17, 5),
            (-3, -2),
            "In Rust, -17 / 5 is -3 and remainder is -2."
        );
        assert_eq!(
            divide(17, -5),
            (-3, 2),
            "In Rust, 17 / -5 is -3 and remainder keeps dividend sign, so 2."
        );
    }
}
