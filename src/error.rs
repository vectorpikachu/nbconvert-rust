use thiserror::Error;


#[derive(Error, Debug)]
pub enum NbconvertError {
  #[error("Failed to read notebook file: {0}")]
  IOError(#[from] std::io::Error),
    
  #[error("Failed to parse notebook: {0}")]
  ParseError(#[from] nbformat::NotebookError),

  #[error("Image processing error: {0}")]
  ImageError(#[from] image::ImageError),
  
  #[error("OnceLock already initialized: {0}")]
  OnceLockError(String),
}

pub type Result<T> = std::result::Result<T, NbconvertError>;
