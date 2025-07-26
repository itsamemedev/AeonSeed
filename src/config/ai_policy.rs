use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default)]
pub struct AiPolicy {
    pub allow_friendly_fire: bool,
}

impl AiPolicy {
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data).unwrap_or_default())
    }
}
