use crate::error::{NbconvertError, Result};
use crate::markdown::parse_markdown;
use crate::typst_content::TypstContent;
use crate::media::process_media;

use nbformat::v4::Output;
/// 解析Jupyter Notebook.
use nbformat::{parse_notebook, v4, legacy, Notebook};

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

/// The Jupyter Notebook's language.
static LANGUAGE: OnceLock<String> = OnceLock::new();

/// 读入给定目录的Notebook
pub fn read_notebook<P: AsRef<Path>>(path: P) -> Result<Notebook> {
    let file_content = fs::read_to_string(path)?;
    let notebook = parse_notebook(&file_content)?;
    Ok(notebook)
}

/// Convert a notebook to Typst.
pub fn convert_notebook(notebook: &Notebook) -> Result<TypstContent> {

    match notebook {
        Notebook::V4(notebook) => convert_v4_notebook(notebook),
        Notebook::Legacy(notebook) => convert_legacy_notebook(notebook),
    }

}

/// Parse a V4 Version notebook.
pub fn convert_v4_notebook(notebook: &v4::Notebook) -> Result<TypstContent> {
    let langugae = match &notebook.metadata.language_info {
        Some(info) => info.name.clone(),
        None => "text".to_owned()
    };

    LANGUAGE.set(langugae)
            .map_err(|v| NbconvertError::OnceLockError(v))?;


    let mut result = String::new();

    for cell in &notebook.cells {
        // Wrap every code and its ouputs in blocks.
        result += "#block[\n";
        match cell {
            v4::Cell::Code { id: _, metadata: _, execution_count, source, outputs } => {
                result += &parse_code(source, execution_count);
                result += &parse_output(outputs);
            }
            v4::Cell::Markdown { id:_ , metadata: _, source, attachments } => {
                result += &parse_markdown(source, attachments);
            }
            v4::Cell::Raw { id: _, metadata: _, source } => {
                result += source.join("\n").as_str();
            }
        }
        result += "]\n";
    }

    println!("{}\n", result);

    Ok(TypstContent { content: result })
}


/// Parse a legacy version notebook.
pub fn convert_legacy_notebook(notebook: &legacy::Notebook) -> Result<TypstContent> {
    let langugae = match &notebook.metadata.language_info {
        Some(info) => info.name.clone(),
        None => "text".to_owned()
    };

    LANGUAGE.set(langugae)
            .map_err(|v| NbconvertError::OnceLockError(v))?;

    let mut result = String::new();

    for cell in &notebook.cells {
        match cell {
            legacy::Cell::Code { id: _, metadata: _, execution_count, source, outputs } => {
                result += &parse_code(source, execution_count);
                result += &parse_output(outputs);
            }
            legacy::Cell::Markdown { id: _, metadata: _, source, attachments } => {
                result += &parse_markdown(source, attachments);
            }
            legacy::Cell::Raw { id: _, metadata: _, source } => {
                result += source.join("\n").as_str();
            }
        }
    }


    Ok(TypstContent { content: result })
}


/// Parse the given code. Place it in the style of code blocks.
fn parse_code(code: &Vec<String>, count: &Option<i32>) -> String {
    let mut result = String::new();

    // Refer to the [template.typ] to see the def. of code-block.
    result += "#code-block(";
    result += format!(
        "\"{}\", ", code.join("").as_str() 
    ).as_str();
    match count {
        Some(count) => {
            result += format!(
                "lang: \"{}\", count: {})\n",
                LANGUAGE.get().unwrap(),
                count
            ).as_str();
        }
        None => {
            result += format!(
                "lang: \"{}\", count: none)\n",
                LANGUAGE.get().unwrap(),
            ).as_str();
        }
    }
    result
}

/// Parse an ouput of a given code block.
fn parse_output(outputs: &Vec<Output>) -> String {
    let mut result = String::new();

    for output in outputs {
        match output {
            v4::Output::DisplayData(data) => {
                result += &process_media(&data.data);
            }
            v4::Output::ExecuteResult(data) => {
                result += &process_media(&data.data);
            }
            v4::Output::Stream { name: _, text } => {
                result += format!(
                    "#output-block(\"{}\")\n",
                    text.0
                ).as_str();
            }
            v4::Output::Error(error) => {
                result += format!(
                    "#output-block(\"{}\")\n",
                    error.traceback.join("\n")
                ).as_str();
            }
        }
    }

    result

}