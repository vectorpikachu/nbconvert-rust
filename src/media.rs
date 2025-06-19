use jupyter_protocol::{Media, MediaType};


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

/// Parse given type of media. 
fn parse_media(data: &MediaType) -> String {

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
        MediaType::Png(_data) => {
            // TODO!
        }
        _ => unimplemented!()
    }

    result

}