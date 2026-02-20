// Challenge 3.3 - Methods That Consume `self`
//
// Define `EmailDraft` and implement consuming builder methods:
// - `new`
// - `with_subject(self, ..)`
// - `with_body(self, ..)`
// - `send(self)`
//
// For testability, `send(self)` returns the rendered message String.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailDraft {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl EmailDraft {
    pub fn new(to: &str) -> EmailDraft {
        let _ = to;
        EmailDraft {
            to: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }

    pub fn with_subject(self, subject: &str) -> EmailDraft {
        let _ = (self, subject);
        EmailDraft {
            to: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }

    pub fn with_body(self, body: &str) -> EmailDraft {
        let _ = (self, body);
        EmailDraft {
            to: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }

    pub fn send(self) -> String {
        let _ = self;
        String::new()
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
    use super::EmailDraft;

    #[test]
    fn chaining_builds_expected_email() {
        let rendered = EmailDraft::new("alice@example.com")
            .with_subject("Hello")
            .with_body("How are you?")
            .send();

        let expected = "To: alice@example.com\nSubject: Hello\nBody: How are you?";
        assert_eq!(
            rendered, expected,
            "Rendered email content mismatch.\nExpected:\n{expected}\nActual:\n{rendered}"
        );
    }

    #[test]
    fn with_methods_preserve_existing_fields() {
        let draft = EmailDraft::new("a@b.com").with_subject("S").with_body("B");

        assert_eq!(draft.to, "a@b.com", "`to` should remain the originally supplied address.");
        assert_eq!(draft.subject, "S", "Subject should be set by with_subject.");
        assert_eq!(draft.body, "B", "Body should be set by with_body.");
    }
}
