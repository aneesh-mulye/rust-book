// Challenge 1.4 - Tuple Structs as Distinct Types
//
// Define:
// - `Celsius(f64)`
// - `Fahrenheit(f64)`
//
// Implement:
// - `to_fahrenheit(c: &Celsius) -> Fahrenheit`
// - `to_celsius(f: &Fahrenheit) -> Celsius`

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

pub fn to_fahrenheit(c: &Celsius) -> Fahrenheit {
    let _ = c;
    Fahrenheit(0.0)
}

pub fn to_celsius(f: &Fahrenheit) -> Celsius {
    let _ = f;
    Celsius(0.0)
}

pub fn tuple_struct_assignment_allowed() -> bool {
    true
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
    use super::{to_celsius, to_fahrenheit, tuple_struct_assignment_allowed, Celsius};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn converts_prompt_example() {
        let c = Celsius(100.0);
        let f = to_fahrenheit(&c);
        let c_back = to_celsius(&f);

        assert!(
            approx_eq(f.0, 212.0),
            "100°C should convert to 212°F. Got {}°F.",
            f.0
        );
        assert!(
            approx_eq(c_back.0, 100.0),
            "212°F should convert back to 100°C. Got {}°C.",
            c_back.0
        );
    }

    #[test]
    fn tuple_struct_types_are_distinct() {
        assert!(
            !tuple_struct_assignment_allowed(),
            "`let temp: Celsius = Fahrenheit(72.0);` should NOT compile because tuple structs are distinct types."
        );
    }
}
