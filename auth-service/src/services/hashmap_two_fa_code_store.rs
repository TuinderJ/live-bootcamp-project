use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<String, (LoginAttemptId, TwoFACode)>,
}

impl HashmapTwoFACodeStore {
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        if self.codes.contains_key(email.as_ref()) && self.remove_code(&email).await.is_err() {
            return Err(TwoFACodeStoreError::UnexpectedError);
        }
        self.codes
            .insert(email.as_ref().to_string(), (login_attempt_id, code));
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email.as_ref()) {
            Some(val) => Ok(val.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email.as_ref()) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut two_fa_code_store = HashmapTwoFACodeStore::new();
        let email = Email::parse("valid@mail.com".to_string()).unwrap();
        let login_attempt_id =
            LoginAttemptId::parse("e9e07c9d-8d78-4eed-b9ec-11ca00dff241".to_string()).unwrap();
        let _ = two_fa_code_store
            .add_code(
                email.clone(),
                login_attempt_id,
                TwoFACode::parse("123456".to_string()).unwrap(),
            )
            .await;

        assert!(two_fa_code_store.codes.contains_key(email.as_ref()));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut two_fa_code_store = HashmapTwoFACodeStore::new();
        let email = Email::parse("valid@mail.com".to_string()).unwrap();
        let login_attempt_id =
            LoginAttemptId::parse("e9e07c9d-8d78-4eed-b9ec-11ca00dff241".to_string()).unwrap();

        two_fa_code_store.codes.insert(
            email.as_ref().to_string(),
            (
                login_attempt_id,
                TwoFACode::parse("123456".to_string()).unwrap(),
            ),
        );

        assert!(two_fa_code_store.get_code(&email).await.is_ok());
        assert!(two_fa_code_store
            .get_code(&Email::parse("invalid@mail.com".to_string()).unwrap())
            .await
            .is_err());
    }
}
