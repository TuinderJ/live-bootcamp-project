use crate::domain::data_stores::UserStore;
use std::collections::HashMap;

use crate::domain::{data_stores::UserStoreError, user::User};

#[derive(Default, Debug, PartialEq)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.users.get(email);
        if user.is_none() {
            return Err(UserStoreError::UserNotFound);
        };
        if user.unwrap().password != password {
            return Err(UserStoreError::InvalidCredentials);
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut test_user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let user = User::new("asdf@asdf.com".to_string(), "password123".to_string(), true);
        test_user_store.users.insert(user.email.clone(), user);

        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let user = User::new("asdf@asdf.com".to_string(), "password123".to_string(), true);
        user_store.add_user(user).await.unwrap();

        assert_eq!(user_store, test_user_store, "Failed");
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let user = User::new("asdf@asdf.com".to_string(), "password123".to_string(), true);
        user_store.users.insert(user.email.clone(), user);

        let user = user_store.get_user("asdf@asdf.com").await.unwrap();
        let test_user = User::new("asdf@asdf.com".to_string(), "password123".to_string(), true);
        assert_eq!(user, &test_user, "Failed to get valid user");

        let user = user_store.get_user("asdf").await;
        assert_eq!(
            user,
            Err(UserStoreError::UserNotFound),
            "Retrieved user with invalid input."
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };
        let user = User::new("asdf@asdf.com".to_string(), "password123".to_string(), true);
        user_store.users.insert(user.email.clone(), user);

        let validate = user_store
            .validate_user("asdf@asdf.com", "password123")
            .await;
        assert_eq!(validate, Ok(()), "Failed to validate a valid user.");

        let validate = user_store.validate_user("asdf", "password123").await;
        assert_eq!(
            validate,
            Err(UserStoreError::UserNotFound),
            "Didn't receive UserNotFound error. Received: {:?}",
            validate
        );

        let validate = user_store.validate_user("asdf@asdf.com", "password").await;
        assert_eq!(
            validate,
            Err(UserStoreError::InvalidCredentials),
            "Didn't receive InvalidCredentials error. Received: {:?}",
            validate
        );
    }
}
