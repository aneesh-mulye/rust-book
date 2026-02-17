// Challenge 5.4 - The Gauntlet
//
// Implement:
// - `const MAX: u32 = 50`
// - `is_prime(n)` using loops
// - `digit_sum(n)` using a while loop
// - `gauntlet_numbers(max)` returning all numbers where:
//   1) number is prime
//   2) digit sum is also prime
//
// Return each match as `(number, digit_sum)`.

pub const MAX: u32 = 50;

pub fn is_prime(n: u32) -> bool {
    let _ = n;
    false
}

pub fn digit_sum(mut n: u32) -> u32 {
    let _ = &mut n;
    0
}

pub fn gauntlet_numbers(max: u32) -> Vec<(u32, u32)> {
    let _ = max;
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
    use super::{digit_sum, gauntlet_numbers, is_prime, MAX};

    #[test]
    fn prime_checker_handles_basics() {
        assert!(!is_prime(0), "0 is not prime.");
        assert!(!is_prime(1), "1 is not prime.");
        assert!(is_prime(2), "2 is prime.");
        assert!(is_prime(47), "47 is prime.");
        assert!(!is_prime(49), "49 is 7*7, so not prime.");
    }

    #[test]
    fn digit_sum_handles_examples() {
        assert_eq!(digit_sum(0), 0, "digit_sum(0) should be 0.");
        assert_eq!(digit_sum(41), 5, "digit_sum(41) should be 5.");
        assert_eq!(digit_sum(999), 27, "digit_sum(999) should be 27.");
    }

    #[test]
    fn gauntlet_output_matches_prompt() {
        let actual = gauntlet_numbers(MAX);
        let expected = vec![
            (2, 2),
            (3, 3),
            (5, 5),
            (7, 7),
            (11, 2),
            (23, 5),
            (29, 11),
            (41, 5),
            (43, 7),
            (47, 11),
        ];

        assert_eq!(
            actual, expected,
            "Gauntlet results for MAX=50 do not match expected values.\nExpected: {:?}\nActual:   {:?}",
            expected, actual
        );
        assert_eq!(
            actual.len(),
            10,
            "Expected total count 10, got {}.",
            actual.len()
        );
    }
}
