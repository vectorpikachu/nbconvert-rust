use std::{fs, path::Path};

use base64::{prelude::BASE64_STANDARD, Engine};
use jupyter_protocol::{Media, MediaType};
use uuid::Uuid;

/// Process the given media.
pub fn process_media(media: &Media, download_dir: &Path) -> String {
    // 如果没有内容，返回空字符串
    if media.content.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    for data in &media.content {
        result += parse_media(data, download_dir).as_str();
    }

    result
}

/// Parse given type of media. 
fn parse_media(data: &MediaType, download_dir: &Path) -> String {

    let mut result = String::new();
    match data {
        MediaType::DataTable(_data) => {
            // TODO!
        }
        MediaType::Latex(data) => {
            result += format!(
                "#mimath(`{}`)",
                data
            ).as_str();
        }
        // Image data is all base64 encoded. These variants could all accept <Vec<u8>> as the
        // data. However, not all users of this library will need immediate decoding of the data.
        MediaType::Png(data) => {
            result += format!(
                "#image(\"{}\")",
                write_figure(data, "png", download_dir)
            ).as_str();
        }
        MediaType::Jpeg(data) => {
            result += format!(
                "#image(\"{}\")",
                write_figure(data, "jpeg", download_dir)
            ).as_str();
        }
        MediaType::Svg(data) => {
            result += format!(
                "#image(\"{}\")",
                write_figure(data, "svg", download_dir)
            ).as_str();
        }
        MediaType::Gif(data) => {
            result += format!(
                "#image(\"{}\")",
                write_figure(data, "gif", download_dir)
            ).as_str();
        }
        MediaType::Plain(data) => {
            result += format!(
                "#```{}```",
                data
            ).as_str();
            // It's a plain text, so we can just use the code block.
        }
        MediaType::Html(_) => {
            println!("Html is not supported yet, skipping.");
        }
        _ => unimplemented!()
    }

    result

}


/// Write the Base64 encoded figure data to a file and return the file path.
/// The file will be saved in the media directory.
fn write_figure(data: &String, ext: &str, download_dir: &Path) -> String {

    // Generate a unique file name based on the content hash or timestamp
    let file_name = format!("figure_{}.{}", Uuid::new_v4(), ext);
    let file_path = download_dir.join(&file_name);

    // Write the Base64 data to the file
    if let Ok(decoded_data) = BASE64_STANDARD.decode(data) {
        fs::write(&file_path, decoded_data).expect("Failed to write media file");
    } else {
        panic!("Failed to decode Base64 data for media");
    }

    format!("./downloads/{}", file_name)
}