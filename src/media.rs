use jupyter_protocol::{Media, MediaType, TabularDataResource};
use serde_json::Value;


pub fn process_data(media: Media) -> String {
    // 如果没有内容，返回空字符串
    if media.content.is_empty() {
        return String::new();
    }
    
    // 按优先级处理不同媒体类型
    
    // 1. 首先检查是否有HTML内容
    if let Some(html) = find_media_type(&media.content, |mt| matches!(mt, MediaType::Html(_))) {
        return html;
    }
    
    // 2. 处理图像类型 (SVG优先，因为它是矢量图，然后是其他光栅图像)
    // 2.1. SVG
    if let Some(svg) = find_media_type(&media.content, |mt| matches!(mt, MediaType::Svg(_))) {
        return svg;
    }
    
    // 2.2. PNG, JPEG, GIF 作为base64编码的图像
    for media_type in &media.content {
        match media_type {
            MediaType::Png(data) => {
                return format!("<img src=\"data:image/png;base64,{}\">", data);
            },
            MediaType::Jpeg(data) => {
                return format!("<img src=\"data:image/jpeg;base64,{}\">", data);
            },
            MediaType::Gif(data) => {
                return format!("<img src=\"data:image/gif;base64,{}\">", data);
            },
            _ => continue,
        }
    }
    
    // 3. Markdown - 需要转换为HTML
    if let Some(markdown) = find_media_type(&media.content, |mt| matches!(mt, MediaType::Markdown(_))) {
        return markdown::to_html(&markdown);
    }
    
    // 4. LaTeX - 简单包装为HTML (在PDF转换时可能需要更复杂的处理)
    if let Some(latex) = find_media_type(&media.content, |mt| matches!(mt, MediaType::Latex(_))) {
        return format!("<div class=\"latex\">{}</div>", latex);
    }
    
    // 5. 处理特殊的JSON类型 (Plotly, VegaLite等)
    for media_type in &media.content {
        match media_type {
            MediaType::Plotly(json) | 
            MediaType::VegaLiteV2(json) | MediaType::VegaLiteV3(json) | 
            MediaType::VegaLiteV4(json) | MediaType::VegaLiteV5(json) | 
            MediaType::VegaLiteV6(json) |
            MediaType::VegaV3(json) | MediaType::VegaV4(json) | MediaType::VegaV5(json) => {
                // 对于可视化数据，我们可以创建一个简单的占位符
                // 注意：真正的渲染需要JavaScript，这在PDF中通常不可用
                return format!(
                    "<div class=\"visualization-placeholder\">
                        <p><strong>Interactive Visualization (not rendered in PDF)</strong></p>
                        <pre>{}</pre>
                    </div>",
                    serde_json::to_string_pretty(json).unwrap_or_default()
                );
            },
            MediaType::DataTable(table) => {
                // 尝试渲染表格数据
                return render_data_table(table);
            },
            _ => continue,
        }
    }
    
    // 6. 最后，回落到纯文本 (通常总是可用的)
    if let Some(text) = find_media_type(&media.content, |mt| matches!(mt, MediaType::Plain(_))) {
        return format!("<pre>{}</pre>", escape_html(&text));
    }
    
    // 7. 如果到这里还没有找到合适的表示，尝试渲染第一个可用的内容
    match &media.content[0] {
        MediaType::Other((mime_type, value)) => {
            return format!(
                "<div class=\"unknown-media\" data-mime-type=\"{}\">
                    <pre>{}</pre>
                </div>",
                mime_type,
                escape_html(&value.to_string())
            );
        },
        MediaType::Javascript(js) => {
            return format!("<div class=\"javascript-code\"><pre>{}</pre></div>", escape_html(js));
        },
        MediaType::Json(json) => {
            return format!(
                "<div class=\"json-data\">
                    <pre>{}</pre>
                </div>",
                escape_html(&serde_json::to_string_pretty(json).unwrap_or_default())
            );
        },
        _ => String::new(),
    }
}

// 辅助函数：从媒体内容中查找特定类型
fn find_media_type(content: &[MediaType], predicate: impl Fn(&MediaType) -> bool) -> Option<String>
{
    for media_type in content {
        if predicate(media_type) {
            match media_type {
                MediaType::Plain(text) if predicate(media_type) => {
                    return Some(text.clone());
                },
                MediaType::Html(html) if predicate(media_type) => {
                    return Some(html.clone());
                },
                MediaType::Markdown(md) if predicate(media_type) => {
                    return Some(md.clone());
                },
                MediaType::Latex(latex) if predicate(media_type) => {
                    return Some(latex.clone());
                },
                MediaType::Svg(svg) if predicate(media_type) => {
                    return Some(svg.clone());
                },
                _ => {}
            }
        }
    }
    None
}

// 辅助函数：HTML转义
fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

// 辅助函数：渲染数据表格
fn render_data_table(table: &TabularDataResource) -> String {
    let mut html = String::from("<table class=\"data-table\">");
    
    // 添加表头（如果可用）
    html.push_str("<thead><tr>");
    for field in &table.schema.fields {
      html.push_str(&format!("<th>{}</th>", escape_html(&field.name)));
    }
    html.push_str("</tr></thead>");
    // 添加表格数据
     html.push_str("<tbody>");
    if let Some(data) = &table.data {
        // 处理数据行
        for row_value in data {
            html.push_str("<tr>");
            
            match row_value {
                Value::Object(obj) => {
                    // 如果是对象，按照schema中的字段顺序渲染
                    for field in &table.schema.fields {
                        let cell_value = obj.get(&field.name).unwrap_or(&Value::Null);
                        html.push_str(&format!("<td>{}</td>", escape_html(&cell_value.to_string())));
                    }
                },
                Value::Array(arr) => {
                    // 如果是数组，直接渲染每个元素
                    for value in arr {
                        html.push_str(&format!("<td>{}</td>", escape_html(&value.to_string())));
                    }
                },
                // 如果不是对象或数组，则将整个值作为单个单元格渲染
                _ => {
                    html.push_str(&format!("<td colspan=\"{}\">
                        {}</td>", table.schema.fields.len(), 
                        escape_html(&row_value.to_string())
                    ));
                }
            }
            
            html.push_str("</tr>");
        }
    }
    html.push_str("</tbody></table>");
    
    html
}