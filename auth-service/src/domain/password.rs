#[derive(Debug, PartialEq)]
pub struct Password(String);

impl Password {
    /// Parses a string input to ensure it's a valid password.
    pub fn parse(s: String) -> Result<Self, String> {
        if s.len() < 8 {
            Err(format!("{} is not a valid password", s))
        } else {
            Ok(Password(s))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_should_parse_valid_input() {
        let password = Password::parse("asdfasdf".to_string()).unwrap();
        assert_eq!(password, Password("asdfasdf".to_string()));
    }

    #[test]
    fn password_should_error_on_invalid_input() {
        let test_cases = ["asdf"];

        for test_case in test_cases.iter() {
            let result = Password::parse(test_case.to_string());

            assert_eq!(
                result,
                Err(format!("{} is not a valid password", test_case))
            );
        }
    }
}
