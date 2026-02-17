// Challenge 1.1 - Shadow Arithmetic
//
// Implement `shadow_arithmetic_values` so it mirrors the challenge steps:
// 1. Start with immutable `x = 5`.
// 2. Shadow `x` with `x * 3`.
// 3. Inside an inner scope, shadow again with `x + 10`.
// 4. Return `(inner_x, outer_x)` where `inner_x` is the value inside the scope
//    and `outer_x` is the value after the scope ends.
//
// Expected return value: `(25, 15)`.

pub fn shadow_arithmetic_values() -> (i32, i32) {
    let x = 5;
    let x = x * 3;
    (x + 10, x)
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
    use super::shadow_arithmetic_values;

    #[test]
    fn returns_expected_inner_and_outer_values() {
        let (inner_x, outer_x) = shadow_arithmetic_values();

        assert_eq!(
            inner_x, 25,
            "Inner value should be 25 after shadowing: 5 -> 15 -> 25. Got {inner_x}."
        );
        assert_eq!(
            outer_x, 15,
            "Outer value should remain 15 after the inner scope ends. Got {outer_x}."
        );
    }
}
