mod error;
mod notebook;
mod media;
mod typst_content;
mod markdown;

use std::sync::OnceLock;

use notebook::{convert_notebook, convert_v4_notebook};

use crate::error::Result;

/// Where to find or load the images.
static IMG_PATH: OnceLock<String> = OnceLock::new();

fn main() -> Result<()> {
    let nb = notebook::read_notebook("./tests/hello.ipynb")?;
    println!("{:#?}", nb);

    let typst_content = convert_notebook(&nb)?;

    

    Ok(())
}
