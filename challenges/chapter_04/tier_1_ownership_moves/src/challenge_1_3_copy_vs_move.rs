// Challenge 1.3 - Copy vs Move
//
// Implement `fixed_copy_vs_move` so it demonstrates:
// - `i32` values are `Copy` (`a` and `b` both usable after assignment)
// - `String` values move by default, so to keep both values you need a fix
//   (for example cloning one side).
//
// Return `(a, b, s, t)` after fixing the scenario.

pub fn fixed_copy_vs_move() -> (i32, i32, String, String) {
    (0, 0, String::new(), String::new())
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
    use super::fixed_copy_vs_move;

    #[test]
    fn preserves_values_for_copy_and_string_cases() {
        let (a, b, s, t) = fixed_copy_vs_move();

        assert_eq!(a, 42, "Expected a=42 for the Copy example. Got {a}.");
        assert_eq!(b, 42, "Expected b=42 for the Copy example. Got {b}.");
        assert_eq!(s, "hello", "Expected s='hello' after fixing String ownership. Got '{s}'.");
        assert_eq!(t, "hello", "Expected t='hello' after fixing String ownership. Got '{t}'.");
    }

    #[test]
    fn string_results_are_independent_values() {
        let (_, _, mut s, t) = fixed_copy_vs_move();
        s.push('!');

        assert_eq!(
            t, "hello",
            "After mutating s, t should stay unchanged if you created an independent owned String. Got t='{t}'."
        );
    }
}
