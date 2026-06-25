pub mod compress;
pub mod config;

pub use compress::{compress_image, compress_images, CompressError, CompressResult};
pub use config::{Config, OutputMode, Quality};
