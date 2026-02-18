// Challenge 1.4 - Ownership in a Loop
//
// Implement a deliberately awkward ownership loop:
// - `consume_and_return` takes ownership of a String and returns it.
// - `ownership_loop_messages` should call that repeatedly in a loop,
//   carrying ownership forward each iteration.
//
// Return one line per iteration in this format:
// "Iteration N: hello"

pub fn consume_and_return(s: String) -> String {
    let _ = s;
    String::new()
}

pub fn ownership_loop_messages(iterations: u32, text: String) -> Vec<String> {
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
    use super::ownership_loop_messages;

    #[test]
    fn matches_prompt_output_shape() {
        let lines = ownership_loop_messages(3, String::from("hello"));
        let expected = vec![
            "Iteration 1: hello".to_string(),
            "Iteration 2: hello".to_string(),
            "Iteration 3: hello".to_string(),
        ];

        assert_eq!(
            lines, expected,
            "Ownership loop output is incorrect.\nExpected: {:?}\nActual:   {:?}",
            expected, lines
        );
    }

    #[test]
    fn zero_iterations_produces_no_lines() {
        let lines = ownership_loop_messages(0, String::from("hello"));
        assert!(
            lines.is_empty(),
            "Expected no output lines when iterations=0. Got {:?}.",
            lines
        );
    }
}
