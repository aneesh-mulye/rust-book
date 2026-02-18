// Challenge 2.3 - The Borrow Rules
//
// Implement fixed versions of the three blocks:
// - Block A already compiles with two immutable borrows.
// - Block B must avoid overlapping immutable + mutable borrow.
// - Block C must avoid overlapping mutable borrows.
//
// Return values are structured for easy testing.

pub fn block_a_values() -> (String, String) {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    (r1.clone(), r2.clone())
}

pub fn fixed_block_b_values() -> (String, String) {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    (r1.clone(), r2.clone())
}

pub fn fixed_block_c_value() -> String {
    String::from("hello")
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
    use super::{block_a_values, fixed_block_b_values, fixed_block_c_value};

    #[test]
    fn block_a_keeps_two_immutable_borrows() {
        let (r1, r2) = block_a_values();
        assert_eq!(
            r1, "hello",
            "Block A first immutable view should be 'hello'. Got '{r1}'."
        );
        assert_eq!(
            r2, "hello",
            "Block A second immutable view should be 'hello'. Got '{r2}'."
        );
    }

    #[test]
    fn block_b_uses_mutable_borrow_only_after_immutable_use_ends() {
        let (before_mutation, after_mutation) = fixed_block_b_values();

        assert_eq!(
            before_mutation, "hello",
            "Block B should read immutable value first as 'hello'. Got '{before_mutation}'."
        );
        assert_eq!(
            after_mutation, "hello!",
            "Block B should mutate only after immutable borrow is no longer used. Expected 'hello!'; got '{after_mutation}'."
        );
    }

    #[test]
    fn block_c_uses_two_mutable_borrows_sequentially() {
        let final_value = fixed_block_c_value();

        assert_eq!(
            final_value, "hello!!",
            "Block C should perform two separate mutable-borrow updates, ending at 'hello!!'. Got '{final_value}'."
        );
    }
}
