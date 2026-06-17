use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image error: {0}")]
    Image(String),
}

#[derive(Debug, Clone)]
pub struct CompressResult {
    pub original_size: u64,
    pub compressed_size: u64,
    pub output_path: String,
}

pub fn compress_image(_input: &str, _output: &str, _quality: &crate::config::Quality) -> Result<CompressResult, CompressError> {
    todo!()
}
