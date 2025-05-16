use std::{collections::HashMap, fs::{self, File}, io, path::{Path, PathBuf}, sync::{LazyLock, RwLock}};

use base64::decode;
use markdown::{mdast::{self, Node}, to_mdast, Constructs, ParseOptions};
use reqwest::blocking;
use serde_json::Value;

use html2typst::parse_html;
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

static DOWNLOAD_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    // 这里指定下载目录，例如项目根下的 "downloads"
    PathBuf::from("downloads")
});

/// Name - Path in String.
static ATTACHMENTS: LazyLock<RwLock<HashMap<String, String>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));


/// Parse a given markdown to Typst contents.
pub fn parse_markdown(source: &Vec<String>, attachments: &Option<Value>) -> String {
    let mut result = String::new();

    let ast = to_mdast(
        source.join("\n").as_str(),
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

    insert_attachments(attachments);

    parse_definition(&ast);

    println!("{:#?}", ast);

    result += parse_ast(&ast).as_str();

    result
}


/// Recursively parse the markdown ast.
fn parse_ast(node: &Node) -> String {
    let mut result = String::new();

    match node {
        Node::Blockquote(node) => {
            // > a.
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child).as_str();
            }
            result += format!(
                "#block-quote(\"{}\")\n\n",
                children_result
            ).as_str();
        }
        Node::Break(_) => {
            // Breakline.
            result += "\\ \n";
        }
        Node::Code(node) => {
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
                children_result += parse_ast(child).as_str();
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
                children_result += parse_ast(child).as_str();
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
                children_result += parse_ast(child).as_str();
            }
            result += format!(
                "{} {}\n\n",
                "=".repeat(node.depth.into()),
                children_result
            ).as_str();
        }
        Node::Html(node) => {
            result += parse_html(node.value.as_str()).as_str();
        }
        Node::Image(node) => {
            // ![alpha](https://example.com/favicon.ico "bravo")
            match Url::parse(&node.url) {
                Ok(url) => {
                    match url.scheme() {
                        "http" | "https" => result += format!(
                            "#figure(align(center, image(\"{}\", width: 50%)))",
                            download(&url)
                        ).as_str(),
                        _ => {
                            // In attachments with base64.
                            let filename = node.url.strip_prefix("attachment:").unwrap();

                            if let Some(filepath) = ATTACHMENTS.read().unwrap().to_owned().get(filename) {
                                result += format!(
                                    "#figure(align(center, image(\"{}\", width: 50%)))",
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
        Node::ImageReference(node) => {
            // todo
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
                children_result += parse_ast(child).as_str();
            }
            result += format!(
                "#link(\"{}\")[{}]",
                node.url,
                children_result
            ).as_str();
        }
        Node::LinkReference(node) => {
            // [a] which is defined before.
        }
        Node::List(node) => {
            // TODO!
        }
        Node::ListItem(node) => {

        }
        Node::Math(node) => {
            result += format!(
                "#mimath(`$$\n{}\n$$`)\n",
                node.value
            ).as_str();
        }
        Node::MdxFlowExpression(node) => {
            // {a}
        }
        Node::MdxJsxFlowElement(node) => {

        }
        Node::MdxJsxTextElement(node) => {

        }
        Node::MdxTextExpression(node) => {

        }
        Node::MdxjsEsm(node) => {

        }
        Node::Paragraph(node) => {
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child).as_str();
            }
            result += children_result.as_str();
            result += "\n\n";
        }
        Node::Root(node) => {
            // This is the root node representing a doc.
            for child in &node.children {
                result += parse_ast(child).as_str();
            }
        }
        Node::Strong(node) => {
            // **a**
            result += "*";
            for child in &node.children {
                result += parse_ast(child).as_str();
            }
            result += "*";
        }
        Node::Table(node) => {

        }
        Node::TableCell(node) => {

        }
        Node::TableRow(node) => {

        }
        Node::Text(node) => {
            result += node.value.as_str();
        }
        Node::ThematicBreak(_) => {
            // The long long line. --------
            result += "#line(length: 100%)\n";
        }
        Node::Toml(node) => {
            // TODO!
        }
        Node::Yaml(node) => {
            // TODO!
        }
        _ => unimplemented!()
    }

    result
}


pub fn parse_definition(node: &Node) {
    match node {
        Node::Definition(node) => {
            DEFINITION.write().unwrap().insert(node.identifier.clone(), node.url.clone());
        }
        Node::FootnoteDefinition(node) => {
            let mut children_result = String::new();
            for child in &node.children {
                children_result += parse_ast(child).as_str();
            }
            FOOTNOTE_DEFINITION.write().unwrap().insert(node.identifier.clone(), children_result);
        }
        _ => {}
    }
}


/// Download the file and return the file path.
pub fn download(url: &Url) -> String {

    let mut resp = blocking::get(url.clone()).unwrap();

    let filename = url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("downloaded");

    let ext_opt = Path::new(filename)
        .extension()
        .and_then(|os| os.to_str());

    let local_name = if let Some(ext) = ext_opt {
        format!("downloaded.{}", ext)       // 有后缀：downloaded.txt
    } else {
        filename.to_string()                // 无后缀：保持原名
    };

    let local_path = DOWNLOAD_DIR.join(local_name);

    let mut out = File::create(&local_path).unwrap();

    io::copy(&mut resp, &mut out).unwrap();

    local_path.display().to_string()

}

fn insert_attachments(attachments: &Option<Value>) {
    let obj = match attachments {
        Some(Value::Object(map)) => map,
        _ => return,
    };

    let mut guard = ATTACHMENTS.write().unwrap();

    for (filename, bundle) in obj {
        // bundle 是另一个对象，键是 MIME 类型，值是 Base64 字符串
        if let Value::Object(inner) = bundle {
            // 拿第一个 MIME 类型的 Base64 数据（例如 "image/png" → "iVBOR..."）
            if let Some(Value::String(data_b64)) = inner.values().next() {
                let bytes = BASE64_STANDARD.decode(data_b64).unwrap();
                let local_path = DOWNLOAD_DIR.join(filename);
                fs::write(&local_path, &bytes).unwrap();
                guard.insert(filename.clone(), local_path.display().to_string());
            }
        }
    }
}