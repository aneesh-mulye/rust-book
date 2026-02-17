// Challenge 5.1 - Prime Sieve
//
// Implement a sieve-based solution with two helper functions:
// - `sieve_up_to(limit)`: returns a bool vector where index `i` is true if `i` is prime.
// - `collect_primes(sieve)`: returns all prime indices.
//
// Keep it simple and loop-based.

pub fn sieve_up_to(limit: usize) -> Vec<bool> {
    let _ = limit;
    Vec::new()
}

pub fn collect_primes(sieve: &[bool]) -> Vec<usize> {
    let _ = sieve;
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
    use super::{collect_primes, sieve_up_to};

    #[test]
    fn finds_expected_primes_up_to_100() {
        let sieve = sieve_up_to(100);
        let primes = collect_primes(&sieve);

        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
            79, 83, 89, 97,
        ];

        assert_eq!(
            primes, expected,
            "Prime list up to 100 is incorrect.\nExpected: {:?}\nActual:   {:?}",
            expected, primes
        );
    }

    #[test]
    fn marks_zero_and_one_as_not_prime() {
        let sieve = sieve_up_to(10);

        assert!(
            sieve.len() > 1,
            "Sieve for limit=10 should contain indices 0 through 10."
        );
        assert!(!sieve[0], "Index 0 must be false (not prime).");
        assert!(!sieve[1], "Index 1 must be false (not prime).");
    }

    #[test]
    fn marks_composites_as_false() {
        let sieve = sieve_up_to(30);
        assert!(
            sieve.len() >= 31,
            "Sieve for limit=30 should contain indices 0 through 30; got len {}.",
            sieve.len()
        );

        for &composite in &[4_usize, 6, 8, 9, 10, 12, 15, 21, 25, 27] {
            assert!(
                !sieve[composite],
                "Composite number {composite} should be marked false in the sieve."
            );
        }
    }
}
