// Challenge 4.3 - Method Signatures and Ownership
//
// Define `Buffer` and implement methods with different `self` forms:
// - `len(&self)`
// - `append(&mut self, text)`
// - `into_data(self)` (consumes)
// - `from_str` associated function

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    pub data: String,
}

impl Buffer {
    pub fn len(&self) -> usize {
        let _ = self;
        0
    }

    pub fn append(&mut self, text: &str) {
        let _ = (self, text);
    }

    pub fn into_data(self) -> String {
        let _ = self;
        String::new()
    }

    pub fn from_str(s: &str) -> Buffer {
        let _ = s;
        Buffer {
            data: String::new(),
        }
    }
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
    use super::Buffer;

    #[test]
    fn matches_prompt_flow() {
        let mut buf = Buffer::from_str("hello");
        let len1 = buf.len();

        buf.append(" world");
        let len2 = buf.len();
        let extracted = buf.into_data();

        assert_eq!(len1, 5, "Initial length should be 5 for 'hello'. Got {len1}.");
        assert_eq!(
            len2, 11,
            "Length after appending ' world' should be 11. Got {len2}."
        );
        assert_eq!(
            extracted, "hello world",
            "into_data should consume buffer and return 'hello world'. Got '{extracted}'."
        );
    }

    #[test]
    fn append_changes_buffer_in_place() {
        let mut buf = Buffer::from_str("");
        buf.append("abc");
        buf.append("def");

        assert_eq!(buf.len(), 6, "After appending 'abc' and 'def', length should be 6.");
        assert_eq!(buf.data, "abcdef", "Buffer data should be 'abcdef'. Got '{}'.", buf.data);
    }
}
