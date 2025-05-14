
use thiserror::Error;


#[derive(Error, Debug)]
pub enum NbconvertError {
  #[error("Failed to read notebook file: {0}")]
  IOError(#[from] std::io::Error),
    
  #[error("Failed to parse notebook: {0}")]
  ParseError(#[from] nbformat::NotebookError),
    
  #[error("Invalid notebook format: {0}")]
  NotebookFormatError(String),

  #[error("Html generation error: {0}")]
  HtmlError(String),  
  
  #[error("PDF generation error: {0}")]
  PdfError(String),
    
  #[error("Image processing error: {0}")]
  ImageError(#[from] image::ImageError),
    
  #[error("Markdown processing error: {0}")]
  MarkdownError(String),
    
  #[error("Unknown error: {0}")]
  Unknown(String),
}

pub type Result<T> = std::result::Result<T, NbconvertError>;
