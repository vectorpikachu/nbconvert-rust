use jupyter_protocol::{Media, MediaType, TabularDataResource};
use serde_json::Value;


/// Process the given media.
pub fn process_media(media: &Media) -> String {
    // 如果没有内容，返回空字符串
    if media.content.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    for data in &media.content {
        result += parse_media(data).as_str();
    }

    result
}

/// Parse given media.
fn parse_media(data: &MediaType) -> String {

    let mut result = String::new();


    match data {
        MediaType::DataTable(data) => {

        }
        MediaType::Latex(data) => {
            result += format!(
                "#mimath(`{}`)",
                data
            ).as_str();
        }
        MediaType::Png(data) => {
            
        }
        _ => unimplemented!()
    }

    result

}