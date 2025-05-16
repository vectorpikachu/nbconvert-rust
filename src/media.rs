use jupyter_protocol::{Media, MediaType, TabularDataResource};
use serde_json::Value;


/// Process the given media.
pub fn process_media(media: &Media) -> String {
    // 如果没有内容，返回空字符串
    if media.content.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    result
    
}