pub mod compress;
pub mod config;

pub use compress::{compress_image, compress_images, CompressResult};
pub use config::{Config, Quality};
