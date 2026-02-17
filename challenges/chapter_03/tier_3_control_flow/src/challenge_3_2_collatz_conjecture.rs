// Challenge 3.2 - Collatz Conjecture with while
//
// Implement `collatz_steps(n)` to count how many updates are needed to reach 1:
// - if n is even: n = n / 2
// - if n is odd: n = 3 * n + 1
//
// Use a `while` loop.

pub fn collatz_steps(mut n: u64) -> u32 {
    let mut count = 0;

    while n != 1 {
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    count
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
    use super::collatz_steps;

    #[test]
    fn matches_prompt_examples() {
        assert_eq!(collatz_steps(6), 8, "collatz_steps(6) should be 8.");
        assert_eq!(collatz_steps(27), 111, "collatz_steps(27) should be 111.");
        assert_eq!(
            collatz_steps(1),
            0,
            "collatz_steps(1) should be 0 because it is already at 1."
        );
    }

    #[test]
    fn handles_small_values() {
        assert_eq!(collatz_steps(2), 1, "2 should reach 1 in one step: 2 -> 1.");
        assert_eq!(collatz_steps(3), 7, "3 should take 7 steps to reach 1.");
    }
}
