// Challenge 5.2 - Fibonacci with Tuples
//
// Implement `fibonacci(n)` (0-indexed):
// - fib(0) = 0
// - fib(1) = 1
//
// Use loop + tuple state updates (`(a, b)`).

pub fn fibonacci(n: u32) -> u64 {
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
    use super::fibonacci;

    #[test]
    fn handles_base_cases() {
        assert_eq!(fibonacci(0), 0, "fib(0) should be 0.");
        assert_eq!(fibonacci(1), 1, "fib(1) should be 1.");
    }

    #[test]
    fn matches_known_values() {
        let known = [
            (2_u32, 1_u64),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (10, 55),
            (19, 4181),
        ];

        for (n, expected) in known {
            let actual = fibonacci(n);
            assert_eq!(
                actual, expected,
                "fib({n}) should be {expected}, got {actual}."
            );
        }
    }

    #[test]
    fn sequence_obeys_recurrence() {
        for n in 2..20 {
            let lhs = fibonacci(n);
            let rhs = fibonacci(n - 1) + fibonacci(n - 2);
            assert_eq!(
                lhs, rhs,
                "Fibonacci recurrence broken at n={n}: fib(n) should equal fib(n-1)+fib(n-2)."
            );
        }
    }
}
