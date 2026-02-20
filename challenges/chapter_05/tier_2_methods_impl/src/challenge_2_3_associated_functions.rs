// Challenge 2.3 - Associated Functions (Constructors)
//
// Add constructors to `Counter`:
// - `Counter::new(min, max)` -> value starts at min
// - `Counter::zero_to(max)` -> shorthand with min=0

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counter {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

impl Counter {
    pub fn new(min: i32, max: i32) -> Counter {
        let _ = (min, max);
        Counter {
            value: 0,
            min: 0,
            max: 0,
        }
    }

    pub fn zero_to(max: i32) -> Counter {
        let _ = max;
        Counter {
            value: 0,
            min: 0,
            max: 0,
        }
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
    use super::Counter;

    #[test]
    fn new_sets_value_to_min() {
        let c = Counter::new(-2, 7);
        assert_eq!(c.min, -2, "Counter::new should set min to -2. Got {}.", c.min);
        assert_eq!(c.max, 7, "Counter::new should set max to 7. Got {}.", c.max);
        assert_eq!(
            c.value, -2,
            "Counter::new should initialize value at min. Expected -2, got {}.",
            c.value
        );
    }

    #[test]
    fn zero_to_sets_min_zero_and_value_zero() {
        let c = Counter::zero_to(5);
        assert_eq!(c.min, 0, "Counter::zero_to should set min=0. Got {}.", c.min);
        assert_eq!(c.max, 5, "Counter::zero_to should set max=5. Got {}.", c.max);
        assert_eq!(
            c.value, 0,
            "Counter::zero_to should initialize value=0. Got {}.",
            c.value
        );
    }
}
