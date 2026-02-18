// Challenge 2.1 - Immutable References
//
// Rewrite the loop idea from Tier 1.4 using borrowing:
// - No ownership transfer required.
// - Function parameter should be `&String`.
// - No return of the owned String needed.
//
// For testability:
// - `describe_iteration` returns one line: "Iteration N: <text>"
// - `borrowing_loop_messages` returns all lines.

pub fn describe_iteration(iteration: u32, text: &String) -> String {
    let _ = (iteration, text);
    String::new()
}

pub fn borrowing_loop_messages(iterations: u32, text: &String) -> Vec<String> {
    let _ = (iterations, text);
    Vec::new()
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
    use super::{borrowing_loop_messages, describe_iteration};

    #[test]
    fn describe_iteration_formats_line() {
        let text = String::from("hello");
        let line = describe_iteration(2, &text);

        assert_eq!(
            line, "Iteration 2: hello",
            "Expected exact line 'Iteration 2: hello'. Got '{line}'."
        );
        assert_eq!(
            text, "hello",
            "Immutable borrowing should not consume or mutate the original String."
        );
    }

    #[test]
    fn borrowing_loop_matches_prompt_shape() {
        let text = String::from("hello");
        let lines = borrowing_loop_messages(3, &text);
        let expected = vec![
            "Iteration 1: hello".to_string(),
            "Iteration 2: hello".to_string(),
            "Iteration 3: hello".to_string(),
        ];

        assert_eq!(
            lines, expected,
            "Borrowing loop output is incorrect.\nExpected: {:?}\nActual:   {:?}",
            expected, lines
        );
        assert_eq!(
            text, "hello",
            "After loop, original String should still be usable and unchanged."
        );
    }
}
