// Challenge 4.1 - Array Statistics
//
// Implement:
// - `sum(arr)` -> total of elements
// - `min(arr)` -> smallest element
// - `max(arr)` -> largest element
// - `mean(arr)` -> average as f64
//
// Use loops over slices (`&[i32]`).

pub fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;

    for i in arr {
        total += i;
    }

    total
}

pub fn min(arr: &[i32]) -> i32 {
    let mut smallest: i32 = i32::MAX;

    for &i in arr {
        if i < smallest {
            smallest = i;
        }
    }

    smallest
}

pub fn max(arr: &[i32]) -> i32 {
    let mut largest: i32 = i32::MIN;

    for &i in arr {
        if i > largest {
            largest = i;
        }
    }

    largest
}

pub fn mean(arr: &[i32]) -> f64 {
    if arr.is_empty() {
        0.0
    } else {
        sum(arr) as f64 / arr.len() as f64
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
    use super::{max, mean, min, sum};

    #[test]
    fn matches_prompt_array_results() {
        let data = [23, 82, 6, 51, 44, 17, 93, 38, 65, 10];

        assert_eq!(sum(&data), 429, "Expected sum 429 for the prompt array.");
        assert_eq!(min(&data), 6, "Expected min 6 for the prompt array.");
        assert_eq!(max(&data), 93, "Expected max 93 for the prompt array.");

        let actual_mean = mean(&data);
        let expected_mean = 42.9_f64;
        let diff = (actual_mean - expected_mean).abs();
        assert!(
            diff < 1e-10,
            "Expected mean {expected_mean}, got {actual_mean} (diff {diff})."
        );
    }

    #[test]
    fn handles_negative_and_mixed_values() {
        let data = [-5, 0, 5, 10, -10];

        assert_eq!(sum(&data), 0, "Expected sum 0 for [-5, 0, 5, 10, -10].");
        assert_eq!(min(&data), -10, "Expected min -10 for mixed values.");
        assert_eq!(max(&data), 10, "Expected max 10 for mixed values.");

        let actual_mean = mean(&data);
        let expected_mean = 0.0_f64;
        let diff = (actual_mean - expected_mean).abs();
        assert!(
            diff < 1e-10,
            "Expected mean {expected_mean}, got {actual_mean} (diff {diff})."
        );
    }
}
