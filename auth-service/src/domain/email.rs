use validator::validate_email;

#[derive(Debug, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Self, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_should_parse_valid_input() {
        let email = Email::parse("asdf@asdf.com".to_string()).unwrap();
        assert_eq!(email, Email("asdf@asdf.com".to_string()));
    }

    #[test]
    fn email_should_error_with_invalid_input() {
        let test_cases = ["@asdf.com", "asdf.com", "asdf@"];

        for test_case in test_cases.iter() {
            let result = Email::parse(test_case.to_string());

            assert_eq!(result, Err(format!("{} is not a valid email.", test_case)));
        }
    }
}
