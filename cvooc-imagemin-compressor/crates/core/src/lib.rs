pub mod compress;
pub mod config;

pub use compress::{compress_image, CompressResult};
pub use config::{Config, Quality};
