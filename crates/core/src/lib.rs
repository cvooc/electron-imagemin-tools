pub mod compress;
pub mod config;
pub mod history;

pub use compress::{compress_image, compress_images, CompressError, CompressResult};
pub use config::{Config, OutputFormat, OutputMode, Quality, ThemeMode};
pub use history::{History, HistoryEntry, HistoryResult};
