use core::panic;
use std::{collections::{HashMap, VecDeque}, fs::{self, File}, io, path::Path, sync::{LazyLock, RwLock}};

use markdown::{mdast::{self, Node}, to_mdast, Constructs, ParseOptions};
use reqwest::blocking;
use serde_json::Value;

// use html2typst::parse_html;
use url::Url;
use base64::prelude::*;

/// The markdown definition.
static DEFINITION: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});
/// The markdown footnote definition.
static FOOTNOTE_DEFINITION: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});


/// Name - Path in String.
static ATTACHMENTS: LazyLock<RwLock<HashMap<String, String>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));


/// Parse a given markdown to Typst contents.
pub fn parse_markdown(source: &Vec<String>, attachments: &Option<Value>, download_dir: &Path) -> String {
    let mut result = String::new();

    let ast = to_mdast(
        source.join("").as_str(),
        &ParseOptions {
            constructs: Constructs {
                math_flow: true,
                math_text: true,
                ..Constructs::gfm() // GitHub Flavored Markdown.
            },
            ..Default::default()
        },
    )
    .unwrap();

    insert_attachments(attachments, download_dir);

    let mut html_queue: VecDeque<&str> = VecDeque::new();

    parse_definition(&ast, &mut html_queue, download_dir);

    result += parse_ast(&ast, &mut html_queue, download_dir).as_str();

    result
}


/// Recursively parse the markdown ast.
fn parse_ast(node: &Node, html_queue: &mut VecDeque<&str>, download_dir: &Path) -> String {
    let mut result = String::new();
    

    match node {
        Node::Blockquote(node) => {
            // > a.
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += format!(
                "#block-quote[{}]\n\n",
                children_result
            ).as_str();
        }
        Node::Break(_) => {
            // Breakline.
            result += "\\ \n";
        }
        Node::Code(node) => {
            // We use the code block in Typst.
            // So its special characters should not be escaped 
            result += format!(
                "```{}\n{}\n```\n\n",
                node.lang.clone().unwrap_or("text".to_string()),
                node.value
            ).as_str();
        }
        Node::Definition(_) => {
            // Defintion, like the Link in Typst.
            // [x]: y, we will use [x] to create a link later.
            // Pre-processed before.
        }
        Node::Delete(node) => {
            // Delete Line.
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += format!(
                "#strike[{}]",
                children_result
            ).as_str();
        }
        Node::Emphasis(node) => {
            // Enphasis.
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += format!(
                "#emph[{}]",
                children_result
            ).as_str();
        }
        Node::FootnoteDefinition(_) => {
            // Like the definition.
            // Pre-processed.
        }
        Node::FootnoteReference(node) => {
            if let Some(link) = FOOTNOTE_DEFINITION.read().unwrap().to_owned().get(node.identifier.as_str()) {
                result += format!(
                    "#footnote(\"{}\")[{}]",
                    link,
                    node.identifier.clone()
                ).as_str();
            }
        }
        Node::Heading(node) => {
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += format!(
                "\n\n{} {}\n\n",
                "=".repeat(node.depth.into()),
                children_result
            ).as_str();
        }
        Node::Html(node) => {
            // We need to record all the HTML nodes.
            // That means, record the first <br>
            // When meeting </br>, apply the effect.
            if node.value == "<br>" {
                // This is a break line.
                result += "\\ \n";
                return result;
            } else {
                result += &parse_html(node.value.as_str(), html_queue);
            }
        }
        Node::Image(node) => {
            // ![alpha](https://example.com/favicon.ico "bravo")
            match Url::parse(&node.url) {
                Ok(url) => {
                    match url.scheme() {
                        "http" | "https" => result += format!(
                            "#figure(align(center, image(\"{}\", width: 100%)))",
                            download(&url, download_dir)
                        ).as_str(),
                        _ => {
                            // In attachments with base64.
                            let filename = node.url.strip_prefix("attachment:").unwrap();

                            if let Some(filepath) = ATTACHMENTS.read().unwrap().to_owned().get(filename) {
                                result += format!(
                                    "#figure(align(center, image(\"{}\", width: 100%)))",
                                    filepath
                                ).as_str();
                            }
                        }
                    }
                }
                _ => {
                    // Baisc file.
                    result += format!(
                        "#figure(align(center, image(\"{}\", width: 50%)))",
                        node.url
                    ).as_str();
                }
            }
        }
        Node::ImageReference(_node) => {
            panic!("Image reference encountered, directly specify it in Jupyter notebook.");
            // !DO not use image reference.
        }
        Node::InlineCode(node) => {
            result += format!(
                "`{}`",
                node.value
            ).as_str();
        }
        Node::InlineMath(node) => {
            result += format!(
                "#mi(`{}`)",
                node.value
            ).as_str();
        }
        Node::Link(node) => {
            // [a](b)
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += format!(
                "#link(\"{}\")[{}]",
                node.url,
                children_result
            ).as_str();
        }
        Node::LinkReference(_node) => {
            // [a] which is defined before.
            panic!("Link reference encountered, directly specify it in Jupyter notebook.");
        }
        Node::List(node) => {
            for child in &node.children {
                // 判断是 enum 还是 list.
                result += if node.ordered { "+ " } else { "- " };
                // 难点在于如何处理嵌套的 List.
                let mut list_item = parse_ast(child, html_queue, download_dir);
                list_item = list_item.trim_end_matches("\n").replace("\n", "\n  ");
                list_item += "\n";
                result += list_item.as_str();
            }
            
            // result += "\n";
        }
        Node::ListItem(node) => {
            // Node 是有一些Markdown content组成的.
            for child in &node.children {
                result += parse_ast(child, html_queue, download_dir).as_str();
            }
        }
        Node::Math(node) => {
            result += format!(
                "#mimath(`$$\n{}\n$$`)",
                node.value
            ).as_str();
        }
        Node::MdxFlowExpression(_) => {
            // {a}
        }
        Node::MdxJsxFlowElement(_) => {

        }
        Node::MdxJsxTextElement(_) => {

        }
        Node::MdxTextExpression(_) => {

        }
        Node::MdxjsEsm(_) => {

        }
        Node::Paragraph(node) => {
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += children_result.as_str();
            result += "\n";
        }
        Node::Root(node) => {
            // This is the root node representing a doc.
            for child in &node.children {
                result += parse_ast(child, html_queue, download_dir).as_str();
                result += "\n"; // Separating the paragraph.
            }
        }
        Node::Strong(node) => {
            // **a**
            result += "*";
            for child in &node.children {
                result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += "*";
        }
        Node::Table(node) => {
            // Typst 里表格的语法：
            /* #table(
                columns: 3,
                align: (left, center, auto, ),
                table.header([ggg], [sss], [sss]), // header cells.
                [x], [y], [z]
                ) */
            result += format!(
                "#table(
  columns: {},
  align: ({}),\n",
                node.align.len(),
                node.align.iter()
                          .map(|a| {
                            match a {
                                mdast::AlignKind::Center => "center".to_string(),
                                mdast::AlignKind::Left => "left".to_string(),
                                mdast::AlignKind::Right => "right".to_string(),
                                mdast::AlignKind::None => "auto".to_string(),
                            }
                          })
                          .collect::<Vec<String>>()
                          .join(", ")
            ).as_str();
            let mut children = node.children.clone();
            // The first row is title.
            let mut table_header = parse_ast(&children.remove(0), html_queue, download_dir); 
            table_header.pop(); // Delete the newline char.
            result += format!(
                "  table.header(
  {}
  ),\n",
                table_header
            ).as_ref();
            // The following rows are contents.
            let mut children_result = String::new();
            for child in &children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += children_result.as_str();
            result += ")\n\n";
        }
        Node::TableCell(node) => {
            // 处理Table Cell.
            result += "[";
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += children_result.as_str();
            result += "], ";
        }
        Node::TableRow(node) => {
            result += "  ";
            // Child of row: Cell.
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            result += children_result.as_str();
            result += "\n";
        }
        Node::Text(node) => {
            result += node.value.as_str();
        }
        Node::ThematicBreak(_) => {
            // The long long line. --------
            result += "#line(length: 100%)\n";
        }
        Node::Toml(_) => {
            // TODO!
        }
        Node::Yaml(_) => {
            // TODO!
        }
    }

    result
}


pub fn parse_definition(node: &Node, html_queue: &mut VecDeque<&str>, download_dir: &Path) {
    match node {
        Node::Definition(node) => {
            DEFINITION.write().unwrap().insert(node.identifier.clone(), node.url.clone());
        }
        Node::FootnoteDefinition(node) => {
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child, html_queue, download_dir).as_str();
            }
            FOOTNOTE_DEFINITION.write().unwrap().insert(node.identifier.clone(), children_result);
        }
        _ => {}
    }
}

/// Download the file and return the file path.
pub fn download(url: &Url, download_dir: &Path) -> String {

    let mut resp = blocking::get(url.clone()).unwrap();

    let filename = url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("downloaded");

    let ext_opt = Path::new(filename)
        .extension()
        .and_then(|os| os.to_str());

    let local_name = if let Some(ext) = ext_opt {
        format!("downloaded.{}", ext)       // With extension: use it.
    } else {
        filename.to_string()                // Without extension: use the name.
    };

    let local_path = download_dir.join(local_name.clone());

    let mut out = File::create(&local_path).unwrap();

    io::copy(&mut resp, &mut out).unwrap();

    format!("./downloads/{}", local_name)

}

fn insert_attachments(attachments: &Option<Value>, download_dir: &Path) {
    let obj = match attachments {
        Some(Value::Object(map)) => map,
        _ => return,
    };

    let mut guard = ATTACHMENTS.write().unwrap();

    for (filename, bundle) in obj {
        // bundle is a JSON object with MIME type as key and Base64 data as value.
        // E.g. {"image/png": "iVBORw0KGgoAAAANSUhEUgAA..."}
        if let Value::Object(inner) = bundle {
            // We only handle the first value in the object.
            // This is a simplification, as the notebook may have multiple MIME types.
            if let Some(Value::String(data_b64)) = inner.values().next() {
                let bytes = BASE64_STANDARD.decode(data_b64).unwrap();
                let local_path = download_dir.join(filename);
                fs::write(&local_path, &bytes).unwrap();
                guard.insert(filename.clone(), local_path.display().to_string());
            }
        }
    }
}



/// This function is to parse several simple HTML tags.
fn parse_html(html: &str, queue: &mut VecDeque<&str>) -> String {
    match html {
        "<b>" => {
            // Bold text.
            queue.push_back("<b>");
            "*".to_string()
        }
        "</b>" => {
            // End of bold text.
            if let Some(_) = queue.pop_front() {
                "*".to_string()
            } else {
                panic!("Unmatched </b> tag");
            }
        }
        "<i>" => {
            // Italic text.
            queue.push_back("<i>");
            "_".to_string()
        }
        "</i>" => {
            // End of italic text.
            if let Some(_) = queue.pop_front() {
                "_".to_string()
            } else {
                panic!("Unmatched </i> tag");
            }
        }
        "<u>" => {
            // Underline text.
            queue.push_back("<u>");
            "#underline[".to_string()
        }
        "</u>" => {
            // End of underline text.
            if let Some(_) = queue.pop_front() {
                "]".to_string()
            } else {
                panic!("Unmatched </u> tag");
            }
        }
        _ => {
            unimplemented!("HTML tag not implemented: {}", html);
        }
    }
}