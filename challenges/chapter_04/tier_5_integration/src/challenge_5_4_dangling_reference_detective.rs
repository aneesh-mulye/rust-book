// Challenge 5.4 - Dangling Reference Detective
//
// The original problems are intentionally invalid. Here you implement corrected
// alternatives:
// - Problem A fix: return owned String instead of `&String` to local data.
// - Problem B fix: mutate then return `&str` tied to input lifetime.
// - Problem C fix: mutate first, then take and return slice.

pub fn make_string_fixed() -> String {
    String::from("hello")
}

pub fn append_and_read_fixed(s: &mut String) -> &str {
    s.push_str(" world");
    s
}

pub fn first_and_mutate_fixed(s: &mut String) -> &str {
    // Not exactly what the original question intended, but whatever.
    s.push('!');
    &s[..1]
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
    use super::{append_and_read_fixed, first_and_mutate_fixed, make_string_fixed};

    #[test]
    fn problem_a_fix_returns_owned_value() {
        let s = make_string_fixed();
        assert_eq!(
            s, "hello",
            "Problem A fix should return owned String 'hello'. Got '{s}'."
        );
    }

    #[test]
    fn problem_b_fix_mutates_and_returns_valid_str_view() {
        let mut s = String::from("hello");
        let view = append_and_read_fixed(&mut s);

        assert_eq!(
            view, "hello world",
            "Returned &str should view updated contents 'hello world'. Got '{view}'."
        );
        let _ = view;
        assert_eq!(
            s, "hello world",
            "append_and_read_fixed should append ' world'. Got '{s}'."
        );
    }

    #[test]
    fn problem_c_fix_avoids_aliasing_violation() {
        let mut s = String::from("hello");
        let first = first_and_mutate_fixed(&mut s);

        assert_eq!(
            first, "h",
            "After safe reordering, returned first slice should be 'h'. Got '{first}'."
        );
        let _ = first;
        assert_eq!(
            s, "hello!",
            "first_and_mutate_fixed should append '!'. Got '{s}'."
        );
    }
}
