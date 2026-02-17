// Challenge 3.3 - Loop with Break-Value
//
// Implement `smallest_cube_exceeding(limit)` using `loop` + `break value`.
// Return the smallest positive integer `n` such that `n^3 > limit`.

pub fn smallest_cube_exceeding(limit: u64) -> u64 {
    let mut count: u64 = 0;

    loop {
        let cube = count.pow(3);
        if cube > limit {
            break count;
        }
        count += 1;
    }
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
    use super::smallest_cube_exceeding;

    #[test]
    fn matches_prompt_example() {
        assert_eq!(
            smallest_cube_exceeding(10_000),
            22,
            "Expected 22 because 21^3 = 9261 and 22^3 = 10648 (> 10000)."
        );
    }

    #[test]
    fn uses_strictly_greater_than_limit() {
        assert_eq!(
            smallest_cube_exceeding(26),
            3,
            "For limit 26, 3 is the first n with n^3 > 26."
        );
        assert_eq!(
            smallest_cube_exceeding(27),
            4,
            "For limit 27, answer should be 4 because 3^3 equals 27 (not greater)."
        );
    }
}
