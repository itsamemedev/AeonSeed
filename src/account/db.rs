use std::collections::HashMap;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
    pub language: String,
    pub birth_year: i32,
    pub region: String,
}

#[derive(Resource, Default)]
pub struct AccountManager {
    pub accounts: HashMap<String, Account>,
}

impl AccountManager {
    pub fn register(&mut self, username: &str, password: &str, language: &str, birth_year: i32, region: &str) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        let account = Account {
            username: username.to_string(),
            password_hash: hash,
            language: language.to_string(),
            birth_year,
            region: region.to_string(),
        };
        self.accounts.insert(username.to_string(), account);
    }

    pub fn verify(&self, username: &str, password: &str) -> bool {
        if let Some(account) = self.accounts.get(username) {
            if let Ok(hash) = PasswordHash::new(&account.password_hash) {
                let argon2 = Argon2::default();
                return argon2.verify_password(password.as_bytes(), &hash).is_ok();
            }
        }
        false
    }
}
