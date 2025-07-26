use std::collections::HashMap;

use sha2::{Digest, Sha256};
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
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
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
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let input_hash = format!("{:x}", hasher.finalize());
            return input_hash == account.password_hash;
        }
        false
    }
}
