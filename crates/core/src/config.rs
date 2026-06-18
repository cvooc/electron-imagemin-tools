use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cvooc-imagemin-compressor")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn output_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("retrocode_io")
            .join("imagemin")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            let config = Self::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let dir = Self::config_dir();
        std::fs::create_dir_all(&dir).ok();
        let path = Self::config_path();
        let content = toml::to_string_pretty(self).unwrap();
        std::fs::write(&path, content).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.quality.jpeg, 80);
        assert_eq!(config.quality.png, 80);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let loaded: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(loaded.quality.jpeg, config.quality.jpeg);
        assert_eq!(loaded.quality.png, config.quality.png);
    }

    #[test]
    fn test_output_dir() {
        let dir = Config::output_dir();
        assert!(dir.ends_with("retrocode_io/imagemin"));
    }
}
