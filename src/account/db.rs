use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
    pub language: String,
    pub birth_year: i32,
    pub region: String,
}

#[derive(Resource, Default)]
pub struct AccountManager;

impl AccountManager {
    pub fn register(&self, username: &str, password: &str, language: &str, birth_year: i32, region: &str) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        let _account = Account {
            username: username.to_string(),
            password_hash: hash,
            language: language.to_string(),
            birth_year,
            region: region.to_string(),
        };
        // TODO: store account in MongoDB
    }

    pub fn verify(&self, _username: &str, _password: &str) -> bool {
        // TODO: verify password against MongoDB entry with Redis rate limiting
        true
    }
}
