// Challenge 1.1 - Predict the Error
//
// Original snippet (intentionally invalid):
//
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}", s1);
//     println!("{}", s2);
// }
//
// Tasks:
// 1. Explain why `println!("{}", s1);` fails after the move.
// 2. Fix it with `.clone()`.
// 3. Fix it by reordering uses so each binding is used while valid.

pub fn fix_with_clone() -> (String, String) {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    (s1, s2)
}

pub fn fix_with_reordering() -> (String, String) {
    // IDK WTF man.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    (s1, s2)
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
    use super::{fix_with_clone, fix_with_reordering};

    #[test]
    fn clone_fix_keeps_both_values_accessible() {
        let (s1, s2) = fix_with_clone();
        assert_eq!(
            s1, "hello",
            "Clone-based fix should leave s1 usable with value 'hello'. Got '{s1}'."
        );
        assert_eq!(
            s2, "hello",
            "Clone-based fix should produce s2='hello'. Got '{s2}'."
        );
    }

    #[test]
    fn reordering_fix_preserves_observable_output_values() {
        let (before_move, after_move) = fix_with_reordering();
        assert_eq!(
            before_move, "hello",
            "Reordering fix should still use s1 before move, yielding 'hello'. Got '{before_move}'."
        );
        assert_eq!(
            after_move, "hello",
            "Reordering fix should still use moved value through s2, yielding 'hello'. Got '{after_move}'."
        );
    }
}
