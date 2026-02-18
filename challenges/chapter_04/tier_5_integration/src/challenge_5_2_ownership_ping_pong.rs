// Challenge 5.2 - Ownership Ping-Pong
//
// Implement this chain:
// 1. `create_greeting(name: &str) -> String` => "Hello, {name}!"
// 2. `make_loud(s: String) -> String` => uppercase
// 3. `add_punctuation(s: &mut String)` => append " Welcome!!!"
// 4. `print_final(s: &String)` => for testability, return the final line as String
//
// Also implement `ping_pong_message(name)` to run the full chain and return final text.

pub fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn make_loud(s: String) -> String {
    s.to_uppercase()
}

pub fn add_punctuation(s: &mut String) {
    s.push_str(" Welcome!!!");
}

pub fn print_final(s: &String) -> String {
    println!("{}", s);
    s.to_string()
}

pub fn ping_pong_message(name: &str) -> String {
    let mut s = make_loud(create_greeting(name));
    add_punctuation(&mut s);
    print_final(&s)
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
    use super::{add_punctuation, create_greeting, make_loud, ping_pong_message, print_final};

    #[test]
    fn individual_steps_transform_correctly() {
        let greeting = create_greeting("Aneesh");
        assert_eq!(
            greeting, "Hello, Aneesh!",
            "Greeting step should build 'Hello, Aneesh!'. Got '{greeting}'."
        );

        let mut loud = make_loud(greeting);
        assert_eq!(
            loud, "HELLO, ANEESH!",
            "make_loud should uppercase the whole greeting. Got '{loud}'."
        );

        add_punctuation(&mut loud);
        assert_eq!(
            loud, "HELLO, ANEESH! Welcome!!!",
            "add_punctuation should append ' Welcome!!!'. Got '{loud}'."
        );

        let rendered = print_final(&loud);
        assert_eq!(
            rendered, "HELLO, ANEESH! Welcome!!!",
            "print_final should return final printable line unchanged. Got '{rendered}'."
        );
    }

    #[test]
    fn chain_matches_prompt_output() {
        let final_text = ping_pong_message("Aneesh");
        assert_eq!(
            final_text, "HELLO, ANEESH! Welcome!!!",
            "Full ownership chain should produce exact expected final output. Got '{final_text}'."
        );
    }
}
