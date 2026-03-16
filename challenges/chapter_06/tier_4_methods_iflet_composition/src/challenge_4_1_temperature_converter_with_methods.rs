// Challenge 4.1 - Temperature Converter with Methods
//
// Define `Temperature` with variants:
// - Celsius(f64)
// - Fahrenheit(f64)
// - Kelvin(f64)
//
// Implement:
// - `to_celsius`
// - `to_fahrenheit`
// - `is_freezing`
// - `warmer_than`

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    pub fn to_celsius(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn to_fahrenheit(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn is_freezing(&self) -> bool {
        let _ = self;
        false
    }

    pub fn warmer_than(&self, other: &Temperature) -> bool {
        let _ = (self, other);
        false
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
    use super::Temperature;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn conversions_work_across_mixed_units() {
        let t1 = Temperature::Celsius(100.0);
        let t2 = Temperature::Fahrenheit(98.6);
        let t3 = Temperature::Kelvin(233.15);

        assert!(approx_eq(t1.to_celsius(), 100.0), "100C should stay 100C.");
        assert!(approx_eq(t1.to_fahrenheit(), 212.0), "100C should convert to 212F.");
        assert!(approx_eq(t2.to_celsius(), 37.0), "98.6F should convert to about 37C.");
        assert!(approx_eq(t3.to_celsius(), -40.0), "233.15K should convert to -40C.");
    }

    #[test]
    fn freezing_and_comparison_methods_reuse_conversion_logic() {
        let hot = Temperature::Celsius(100.0);
        let body = Temperature::Fahrenheit(98.6);
        let cold = Temperature::Kelvin(233.15);

        assert!(!hot.is_freezing(), "100C should not be freezing.");
        assert!(cold.is_freezing(), "233.15K should be freezing because it is -40C.");
        assert!(hot.warmer_than(&body), "100C should be warmer than 98.6F.");
        assert!(!cold.warmer_than(&body), "-40C should not be warmer than body temperature.");
    }
}
