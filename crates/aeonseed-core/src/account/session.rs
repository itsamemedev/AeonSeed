use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm};
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct SessionData {
    pub token: String,
    pub username: String,
}

#[derive(Resource, Default)]
pub struct SessionStore {
    pub data: Option<SessionData>,
}

impl SessionStore {
    fn path() -> PathBuf {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
            .join(".aeonseed").join("session.json")
    }

    pub fn load(&mut self) {
        let path = Self::path();
        if let Ok(buf) = fs::read(&path) {
            if let Some(data) = decrypt(&buf) {
                self.data = serde_json::from_slice(&data).ok();
            }
        }
    }

    pub fn save(&self) {
        if let Some(data) = &self.data {
            if let Ok(buf) = serde_json::to_vec(data) {
                if let Some(enc) = encrypt(&buf) {
                    let _ = fs::create_dir_all(Self::path().parent().unwrap());
                    let _ = fs::write(Self::path(), enc);
                }
            }
        }
    }
}

fn cipher() -> Aes256Gcm {
    Aes256Gcm::new_from_slice(&[0u8; 32]).unwrap()
}

fn encrypt(data: &[u8]) -> Option<Vec<u8>> {
    let cipher = cipher();
    let nonce = aes_gcm::Nonce::from_slice(&[0u8; 12]);
    cipher.encrypt(nonce, data).ok().map(|mut ct| {
        ct.extend_from_slice(nonce);
        ct
    })
}
fn decrypt(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 12 { return None; }
    let cipher = cipher();
    let (ct, nonce) = data.split_at(data.len()-12);
    cipher.decrypt(aes_gcm::Nonce::from_slice(nonce), ct).ok()
}
