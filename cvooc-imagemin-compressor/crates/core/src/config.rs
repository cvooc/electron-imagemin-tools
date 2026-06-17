use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quality {
    pub jpeg: u8,
    pub png: u8,
}

impl Default for Quality {
    fn default() -> Self {
        Self { jpeg: 80, png: 80 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub quality: Quality,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            quality: Quality::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::default()
    }
}
