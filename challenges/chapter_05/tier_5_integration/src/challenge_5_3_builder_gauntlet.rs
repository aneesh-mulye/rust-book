// Challenge 5.3 - The Builder Gauntlet
//
// Define `Form` and `FormBuilder` with consuming builder methods.
// `build` should return a default placeholder form when validation fails.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Form {
    pub username: String,
    pub age: u32,
    pub accepted_terms: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormBuilder {
    pub username: String,
    pub age: u32,
    pub accepted_terms: bool,
    pub errors: u32,
}

impl FormBuilder {
    pub fn new() -> FormBuilder {
        FormBuilder {
            username: String::new(),
            age: 0,
            accepted_terms: false,
            errors: 0,
        }
    }

    pub fn username(self, name: &str) -> FormBuilder {
        let _ = name;
        self
    }

    pub fn age(self, age: u32) -> FormBuilder {
        let _ = age;
        self
    }

    pub fn accept_terms(self) -> FormBuilder {
        self
    }

    pub fn build(self) -> Form {
        let _ = self;
        Form {
            username: String::new(),
            age: 0,
            accepted_terms: false,
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
    use super::{Form, FormBuilder};

    #[test]
    fn valid_builder_chain_produces_form() {
        let form = FormBuilder::new()
            .username("aneesh")
            .age(28)
            .accept_terms()
            .build();

        assert_eq!(
            form,
            Form {
                username: String::from("aneesh"),
                age: 28,
                accepted_terms: true,
            },
            "Valid builder chain should produce populated form. Got {:?}.",
            form
        );
    }

    #[test]
    fn invalid_input_returns_placeholder_form() {
        let form = FormBuilder::new().username("").age(0).build();

        assert_eq!(
            form,
            Form {
                username: String::new(),
                age: 0,
                accepted_terms: false,
            },
            "Invalid builder chain should return placeholder default form. Got {:?}.",
            form
        );
    }

    #[test]
    fn out_of_range_age_is_rejected() {
        let form = FormBuilder::new()
            .username("ok")
            .age(151)
            .accept_terms()
            .build();

        assert_eq!(
            form,
            Form {
                username: String::new(),
                age: 0,
                accepted_terms: false,
            },
            "Age > 150 should be rejected and produce placeholder form. Got {:?}.",
            form
        );
    }
}
