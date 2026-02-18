// Challenge 2.2 - Mutable References
//
// Implement `append_exclamation(s: &mut String)` so each call appends "!".

pub fn append_exclamation(s: &mut String) {
    let _ = s;
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
    use super::append_exclamation;

    #[test]
    fn appends_three_times_for_prompt_scenario() {
        let mut text = String::from("hello");
        append_exclamation(&mut text);
        append_exclamation(&mut text);
        append_exclamation(&mut text);

        assert_eq!(
            text, "hello!!!",
            "After three appends, expected 'hello!!!'. Got '{text}'."
        );
    }

    #[test]
    fn works_with_empty_string() {
        let mut text = String::new();
        append_exclamation(&mut text);

        assert_eq!(
            text, "!",
            "Appending to empty String should yield '!'. Got '{text}'."
        );
    }
}
