use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputMode {
    /// 输出到 ~/retrocode_io/imagemin/<时间戳>/
    Timestamped,
    /// 输出到与输入文件相同的目录
    SameDir,
    /// 输出到用户自定义目录
    Custom,
}

impl Default for OutputMode {
    fn default() -> Self {
        Self::Timestamped
    }
}

fn default_jpeg_quality() -> u8 {
    80
}

fn default_png_quality() -> u8 {
    80
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quality {
    #[serde(default = "default_jpeg_quality")]
    pub jpeg: u8,
    #[serde(default = "default_png_quality")]
    pub png: u8,
}

impl Quality {
    /// 校验质量参数是否在合法范围内。
    ///
    /// 返回 `Ok(())` 表示有效，否则返回错误描述。
    pub fn validate(&self) -> Result<(), String> {
        if self.jpeg > 100 {
            return Err(format!("JPEG 质量必须在 0-100 之间，当前为 {}", self.jpeg));
        }
        if self.png > 100 {
            return Err(format!("PNG 质量必须在 0-100 之间，当前为 {}", self.png));
        }
        Ok(())
    }

    /// 将非法值裁剪到合法范围，用于兼容旧配置。
    pub fn clamp(&mut self) {
        self.jpeg = self.jpeg.min(100);
        self.png = self.png.min(100);
    }
}

impl Default for Quality {
    fn default() -> Self {
        Self {
            jpeg: default_jpeg_quality(),
            png: default_png_quality(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub quality: Quality,
    pub output_mode: OutputMode,
    pub custom_output_dir: Option<PathBuf>,
    /// PNG 是否使用纯无损优化。默认为 false，即对 RGB/RGBA 图片启用有损量化。
    pub png_lossless: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            quality: Quality::default(),
            output_mode: OutputMode::default(),
            custom_output_dir: None,
            png_lossless: false,
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

    pub fn base_output_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("retrocode_io")
            .join("imagemin")
    }

    /// 根据配置和输入文件路径计算最终输出目录。
    ///
    /// - `Timestamped`：在基础输出目录下创建时间戳子目录。
    /// - `SameDir`：返回输入文件所在目录（批量压缩时以第一个文件为准）。
    /// - `Custom`：返回用户自定义目录，若未设置则回退到 `Timestamped`。
    pub fn resolve_output_dir(&self, input_path: Option<&Path>) -> PathBuf {
        match self.output_mode {
            OutputMode::Timestamped => {
                let timestamp = chrono::Local::now().format("%Y-%m-%d-%H_%M_%S").to_string();
                Self::base_output_dir().join(timestamp)
            }
            OutputMode::SameDir => input_path
                .and_then(|p| p.parent())
                .map(Path::to_path_buf)
                .unwrap_or_else(|| Self::base_output_dir().join("same_dir")),
            OutputMode::Custom => self
                .custom_output_dir
                .clone()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or_else(|| {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H_%M_%S").to_string();
                    Self::base_output_dir().join(timestamp)
                }),
        }
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            let mut config: Self = toml::from_str(&content).unwrap_or_default();
            config.quality.clamp();
            config
        } else {
            let config = Self::default();
            let _ = config.save();
            config
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let dir = Self::config_dir();
        std::fs::create_dir_all(&dir)?;
        let path = Self::config_path();
        let content = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(&path, content)
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
        assert!(matches!(config.output_mode, OutputMode::Timestamped));
        assert!(config.custom_output_dir.is_none());
    }

    #[test]
    fn test_quality_validate_ok() {
        let q = Quality { jpeg: 80, png: 80 };
        assert!(q.validate().is_ok());
    }

    #[test]
    fn test_quality_validate_err() {
        let q = Quality { jpeg: 255, png: 80 };
        assert!(q.validate().is_err());
        let q = Quality { jpeg: 80, png: 255 };
        assert!(q.validate().is_err());
    }

    #[test]
    fn test_quality_clamp() {
        let mut q = Quality { jpeg: 255, png: 255 };
        q.clamp();
        assert_eq!(q.jpeg, 100);
        assert_eq!(q.png, 100);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let loaded: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(loaded.quality.jpeg, config.quality.jpeg);
        assert_eq!(loaded.quality.png, config.quality.png);
        assert_eq!(loaded.output_mode, config.output_mode);
        assert_eq!(loaded.png_lossless, config.png_lossless);
    }

    #[test]
    fn test_output_dir_timestamped() {
        let config = Config::default();
        let dir = config.resolve_output_dir(None);
        assert!(dir.to_string_lossy().contains("retrocode_io"));
        assert!(dir.to_string_lossy().contains("imagemin"));
    }

    #[test]
    fn test_output_dir_same_dir() {
        let config = Config {
            output_mode: OutputMode::SameDir,
            ..Default::default()
        };
        let input = PathBuf::from("/tmp/images/photo.jpg");
        let dir = config.resolve_output_dir(Some(&input));
        assert_eq!(dir, PathBuf::from("/tmp/images"));
    }

    #[test]
    fn test_output_dir_custom() {
        let config = Config {
            output_mode: OutputMode::Custom,
            custom_output_dir: Some(PathBuf::from("/custom/output")),
            ..Default::default()
        };
        let dir = config.resolve_output_dir(None);
        assert_eq!(dir, PathBuf::from("/custom/output"));
    }

    #[test]
    fn test_output_dir_custom_fallback() {
        let config = Config {
            output_mode: OutputMode::Custom,
            custom_output_dir: None,
            ..Default::default()
        };
        let dir = config.resolve_output_dir(None);
        assert!(dir.to_string_lossy().contains("retrocode_io"));
    }
}
