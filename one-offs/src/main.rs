fn main() {
    println!("Hello, world!");
}

fn second_word(s: &str) -> &str {
    let mut begin: usize = 0;
    let sb = s.as_bytes();

    while begin < sb.len() && sb[begin] != b' ' {
        begin += 1;
    }
    if begin == sb.len() {
        return "";
    }
    let mut end = begin + 1;
    while end < sb.len() && sb[end] != b' ' {
        end += 1;
    }
    &s[begin + 1..end]
}
