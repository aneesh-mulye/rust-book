// Challenge 3.3 - Chained Optionality
//
// Implement:
// - `get(arr, index) -> Option<i32>`
// - `reciprocal(n) -> Option<f64>`
// - `classify(x) -> &'static str`
// - `lookup_and_classify(arr, index) -> String`
//
// The prompt text conflicts with its example. Follow the example output:
// reciprocal `0.2` should classify as `"small"`.

pub fn get(arr: &[i32], index: usize) -> Option<i32> {
    let _ = (arr, index);
    None
}

pub fn reciprocal(n: i32) -> Option<f64> {
    let _ = n;
    None
}

pub fn classify(x: f64) -> &'static str {
    let _ = x;
    ""
}

pub fn lookup_and_classify(arr: &[i32], index: usize) -> String {
    let _ = (arr, index);
    String::new()
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
    use super::{classify, get, lookup_and_classify, reciprocal};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn primitives_handle_bounds_and_zero() {
        let data = [0, 5, -3, 0, 8];

        assert_eq!(get(&data, 1), Some(5), "Index 1 should return Some(5).");
        assert_eq!(get(&data, 10), None, "Out-of-bounds access should return None.");
        assert_eq!(reciprocal(0), None, "Reciprocal of 0 should be None.");
        assert!(
            approx_eq(reciprocal(5).unwrap_or(-1.0), 0.2),
            "Reciprocal of 5 should be Some(0.2)."
        );
    }

    #[test]
    fn classify_matches_prompt_example_behavior() {
        assert_eq!(classify(0.2), "small", "0.2 should classify as 'small' per the prompt example.");
        assert_eq!(classify(2.0), "large", "A larger reciprocal like 2.0 should classify as 'large'.");
    }

    #[test]
    fn nested_match_messages_distinguish_failure_modes() {
        let data = [0, 5, -3, 0, 8];

        assert_eq!(
            lookup_and_classify(&data, 1),
            "Index 1: reciprocal of 5 is 0.2 (small)",
            "Index 1 should succeed through all three steps."
        );
        assert_eq!(
            lookup_and_classify(&data, 0),
            "Index 0: element is 0 - cannot compute reciprocal",
            "Index 0 should fail at the reciprocal step because value is zero."
        );
        assert_eq!(
            lookup_and_classify(&data, 10),
            "Index 10: index out of bounds",
            "Index 10 should fail at the lookup step."
        );
    }
}
