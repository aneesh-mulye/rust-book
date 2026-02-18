// Challenge 1.2 - Move Into a Function
//
// Implement:
// - `print_and_return(s: String) -> String`
// - `move_into_function_demo()` that simulates:
//   1. Create `String::from("ownership")`
//   2. Pass it into `print_and_return`
//   3. Use returned value again
//
// For testability, `move_into_function_demo` should return a tuple:
// `(inside_function_value, back_in_main_value)`.

pub fn print_and_return(s: String) -> String {
    println!("{s}");
    s
}

pub fn move_into_function_demo() -> (String, String) {
    let s = String::from("ownership");
    let olds = s.clone();
    (olds, print_and_return(s))
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
    use super::{move_into_function_demo, print_and_return};

    #[test]
    fn print_and_return_gives_ownership_back() {
        let returned = print_and_return(String::from("ownership"));
        assert_eq!(
            returned, "ownership",
            "print_and_return should give back the same String so ownership returns to caller. Got '{returned}'."
        );
    }

    #[test]
    fn demo_matches_prompt_values() {
        let (inside, back) = move_into_function_demo();
        assert_eq!(
            inside, "ownership",
            "Inside-function value should be 'ownership'. Got '{inside}'."
        );
        assert_eq!(
            back, "ownership",
            "Back-in-main value should still be 'ownership' via returned ownership. Got '{back}'."
        );
    }
}
