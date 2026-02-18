// Challenge 3.3 - Why `&str` Over `&String`
//
// Implement `starts_with_r(s: &str) -> bool` that returns true when first byte
// is `b'r'` or `b'R'`.
//
// It should work with:
// - string literals
// - `String` values via `&string`
// - `String` slices like `&string[1..]`

pub fn starts_with_r(s: &str) -> bool {
    let _ = s;
    false
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
    use super::starts_with_r;

    #[test]
    fn accepts_literal_string_and_returns_expected_value() {
        assert!(
            starts_with_r("rust"),
            "Expected true for literal 'rust'."
        );
    }

    #[test]
    fn accepts_string_and_slice_inputs() {
        let owned = String::from("Rust");
        let shifted = &owned[1..];

        assert!(
            starts_with_r(&owned),
            "Expected true for owned String 'Rust' passed as &str."
        );
        assert!(
            !starts_with_r(shifted),
            "Expected false for slice 'ust' because it no longer starts with r/R."
        );
    }

    #[test]
    fn empty_string_is_false() {
        assert!(
            !starts_with_r(""),
            "Empty string should return false because there is no first byte."
        );
    }
}
