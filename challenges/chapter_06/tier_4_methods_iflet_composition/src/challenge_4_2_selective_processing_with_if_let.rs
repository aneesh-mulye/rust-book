// Challenge 4.2 - Selective Processing with `if let`
//
// Implement:
// - `summarize_readings(readings) -> (valid_count, sum, average, failed_count)`
// - `first_valid(readings) -> Option<f64>`

pub fn summarize_readings(readings: &[Option<f64>]) -> (usize, f64, f64, usize) {
    let _ = readings;
    (0, 0.0, 0.0, 0)
}

pub fn first_valid(readings: &[Option<f64>]) -> Option<f64> {
    let _ = readings;
    None
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
    use super::{first_valid, summarize_readings};

    #[test]
    fn summarizes_valid_and_failed_values() {
        let readings = [
            Some(23.5),
            None,
            Some(22.1),
            Some(24.0),
            None,
            Some(21.8),
            None,
            Some(23.0),
        ];

        let (valid, sum, average, failed) = summarize_readings(&readings);

        assert_eq!(valid, 5, "There should be 5 valid readings.");
        assert!((sum - 114.4).abs() < 1e-10, "Sum of valid readings should be 114.4. Got {sum}.");
        assert!(
            (average - 22.88).abs() < 1e-10,
            "Average of valid readings should be 22.88. Got {average}."
        );
        assert_eq!(failed, 3, "There should be 3 failed readings.");
    }

    #[test]
    fn finds_first_some_value() {
        let readings = [None, None, Some(9.5), Some(10.0)];
        assert_eq!(first_valid(&readings), Some(9.5), "First valid reading should be 9.5.");
        assert_eq!(first_valid(&[None, None]), None, "All-None input should return None.");
    }
}
