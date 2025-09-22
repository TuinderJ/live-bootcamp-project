use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};
use std::collections::HashSet;

#[derive(Default, Debug, PartialEq)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

impl HashsetBannedTokenStore {
    pub fn new() -> Self {
        Self {
            tokens: HashSet::new(),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token);
        Ok(())
    }
    async fn contains_token(&self, token: String) -> bool {
        self.tokens.contains(&token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{Email, Password, User},
        utils::generate_auth_cookie,
    };

    #[tokio::test]
    async fn test_add_token() {
        let mut test_banned_token_store = HashsetBannedTokenStore::new();
        let user = User::new(
            Email::parse("valid@mail.com".to_string()).unwrap(),
            Password::parse("password123".to_string()).unwrap(),
            false,
        );

        let token = generate_auth_cookie(&user.email)
            .expect("Failed to generate cookie")
            .value()
            .to_string();

        test_banned_token_store
            .add_token(token.clone())
            .await
            .unwrap();

        assert!(test_banned_token_store.tokens.contains(&token));
    }

    #[tokio::test]
    async fn test_is_token_banned() {
        let mut test_banned_token_store = HashsetBannedTokenStore::new();
        let user = User::new(
            Email::parse("valid@mail.com".to_string()).unwrap(),
            Password::parse("password123".to_string()).unwrap(),
            false,
        );

        let token = generate_auth_cookie(&user.email)
            .expect("Failed to generate cookie")
            .value()
            .to_string();

        assert!(!test_banned_token_store.contains_token(token.clone()).await);

        test_banned_token_store.tokens.insert(token.to_string());

        assert!(test_banned_token_store.contains_token(token.clone()).await);
    }
}
