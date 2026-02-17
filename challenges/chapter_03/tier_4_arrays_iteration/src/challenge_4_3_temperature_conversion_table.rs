// Challenge 4.3 - Temperature Conversion Table
//
// Implement:
// - `f_to_c(f)` for Fahrenheit -> Celsius
// - `c_to_f(c)` for Celsius -> Fahrenheit

pub fn f_to_c(f: f64) -> f64 {
    let _ = f;
    0.0
}

pub fn c_to_f(c: f64) -> f64 {
    let _ = c;
    0.0
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
    use super::{c_to_f, f_to_c};

    fn assert_close(actual: f64, expected: f64, context: &str) {
        let diff = (actual - expected).abs();
        assert!(
            diff < 1e-10,
            "{context} Expected {expected}, got {actual} (diff {diff})."
        );
    }

    #[test]
    fn converts_fahrenheit_to_celsius_examples() {
        assert_close(f_to_c(0.0), -17.77777777777778, "0 F -> C.");
        assert_close(f_to_c(32.0), 0.0, "32 F -> C.");
        assert_close(f_to_c(100.0), 37.77777777777778, "100 F -> C.");
        assert_close(f_to_c(212.0), 100.0, "212 F -> C.");
    }

    #[test]
    fn converts_celsius_to_fahrenheit_examples() {
        assert_close(c_to_f(-40.0), -40.0, "-40 C -> F.");
        assert_close(c_to_f(0.0), 32.0, "0 C -> F.");
        assert_close(c_to_f(37.0), 98.6, "37 C -> F.");
        assert_close(c_to_f(100.0), 212.0, "100 C -> F.");
    }

    #[test]
    fn round_trip_is_consistent() {
        let values = [-40.0, -10.0, 0.0, 32.0, 98.6, 212.0];

        for f in values {
            let c = f_to_c(f);
            let f_back = c_to_f(c);
            assert_close(f_back, f, "Round-trip F -> C -> F mismatch.");
        }
    }
}
