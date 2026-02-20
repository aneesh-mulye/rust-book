// Challenge 2.2 - Mutating Methods
//
// Define `Counter` with `value`, `min`, `max` and implement:
// - increment (clamp at max)
// - decrement (clamp at min)
// - reset (to min)
// - is_maxed

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counter {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

impl Counter {
    pub fn increment(&mut self) {
        let _ = self;
    }

    pub fn decrement(&mut self) {
        let _ = self;
    }

    pub fn reset(&mut self) {
        let _ = self;
    }

    pub fn is_maxed(&self) -> bool {
        let _ = self;
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
    use super::Counter;

    #[test]
    fn increment_clamps_to_max_as_prompt_describes() {
        let mut counter = Counter {
            value: 0,
            min: 0,
            max: 3,
        };

        let mut observed = Vec::new();
        for _ in 0..5 {
            counter.increment();
            observed.push(counter.value);
        }

        assert_eq!(
            observed,
            vec![1, 2, 3, 3, 3],
            "After 5 increments from 0 with max=3, expected [1,2,3,3,3]. Got {:?}.",
            observed
        );
        assert!(counter.is_maxed(), "Counter should report maxed after clamping at max.");
    }

    #[test]
    fn decrement_and_reset_respect_bounds() {
        let mut counter = Counter {
            value: 1,
            min: 0,
            max: 3,
        };

        counter.decrement();
        assert_eq!(counter.value, 0, "1 decrement from 1 should reach 0.");

        counter.decrement();
        assert_eq!(counter.value, 0, "Decrement below min should clamp at 0.");

        counter.value = 3;
        counter.reset();
        assert_eq!(counter.value, 0, "Reset should set value to min (0).");
    }
}
