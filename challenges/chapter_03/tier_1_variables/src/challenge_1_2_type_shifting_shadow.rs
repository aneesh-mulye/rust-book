// Challenge 1.2 - Type-Shifting Shadow
//
// Implement `type_shifting_shadow` to follow the challenge steps:
// 1. Start with the string literal "1024".
// 2. Parse it as `u32`.
// 3. Convert to `f64`.
// 4. Divide by `3.0`.
// 5. Return the final floating-point value.
//
// Expected return value: `341.3333333333333` (approximately).

pub fn type_shifting_shadow() -> f64 {
    let data = "1024";
    let data = data.parse::<u32>().unwrap();
    let data = data as f64;
    data / 3.0
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
    use super::type_shifting_shadow;

    #[test]
    fn computes_expected_floating_point_result() {
        let value = type_shifting_shadow();
        let expected = 341.3333333333333_f64;
        let diff = (value - expected).abs();

        assert!(
            diff < 1e-12,
            "Expected approximately {expected} after parse->cast->divide, got {value} (diff {diff})."
        );
    }

    #[test]
    fn keeps_fractional_component() {
        let value = type_shifting_shadow();
        assert!(
            value.fract() != 0.0,
            "Result should keep a fractional component after dividing by 3.0; got {value}."
        );
    }
}
