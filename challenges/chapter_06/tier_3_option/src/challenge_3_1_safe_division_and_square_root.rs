// Challenge 3.1 - Safe Division and Square Root
//
// Implement:
// - `safe_divide(a, b) -> Option<f64>`
// - `safe_sqrt(x) -> Option<f64>`
//
// `safe_divide` should return `None` when `b == 0.0`.
// `safe_sqrt` should return `None` when `x < 0.0`.

pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    let _ = (a, b);
    None
}

pub fn safe_sqrt(x: f64) -> Option<f64> {
    let _ = x;
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
    use super::{safe_divide, safe_sqrt};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn safe_divide_handles_success_and_zero_division() {
        assert!(
            approx_eq(safe_divide(10.0, 3.0).unwrap_or(-1.0), 3.3333333333333335),
            "10 / 3 should return Some(3.3333333333333335)."
        );
        assert_eq!(
            safe_divide(10.0, 0.0),
            None,
            "10 / 0 should return None for division by zero."
        );
    }

    #[test]
    fn safe_sqrt_handles_success_and_negative_input() {
        assert!(
            approx_eq(safe_sqrt(25.0).unwrap_or(-1.0), 5.0),
            "sqrt(25) should return Some(5.0)."
        );
        assert_eq!(
            safe_sqrt(-4.0),
            None,
            "sqrt(-4) should return None for negative input."
        );
    }
}
