use crate::error::Result;


/// 解析Jupyter Notebook.
use nbformat::{parse_notebook, Notebook};

use std::fs;
use std::path::Path;


/// 读入给定目录的Notebook
pub fn read_notebook<P: AsRef<Path>>(path: P) -> Result<Notebook> {
    let file_content = fs::read_to_string(path)?;
    let notebook = parse_notebook(&file_content)?;
    Ok(notebook)
}