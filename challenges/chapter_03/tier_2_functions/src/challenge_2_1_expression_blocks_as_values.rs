// Challenge 2.1 - Expression Blocks as Values
//
// Implement `classify_age(age)` to return:
// - "child" for ages 0..=12
// - "teen" for ages 13..=19
// - "adult" for ages 20..=64
// - "senior" for ages 65+
//
// The challenge asks you to use an expression block pattern.

pub fn classify_age(age: u32) -> &'static str {
    let _ = age;
    ""
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
    use super::classify_age;

    #[test]
    fn classifies_boundaries_correctly() {
        assert_eq!(
            classify_age(0),
            "child",
            "Age 0 should be classified as child."
        );
        assert_eq!(
            classify_age(12),
            "child",
            "Age 12 should still be child."
        );
        assert_eq!(
            classify_age(13),
            "teen",
            "Age 13 should start teen range."
        );
        assert_eq!(
            classify_age(19),
            "teen",
            "Age 19 should still be teen."
        );
        assert_eq!(
            classify_age(20),
            "adult",
            "Age 20 should start adult range."
        );
        assert_eq!(
            classify_age(64),
            "adult",
            "Age 64 should still be adult."
        );
        assert_eq!(
            classify_age(65),
            "senior",
            "Age 65 should start senior range."
        );
    }
}
