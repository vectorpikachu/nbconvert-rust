use std::{collections::HashMap, sync::{LazyLock, RwLock}};

use markdown::{mdast::{self, Node}, to_mdast, Constructs, ParseOptions};
use serde_json::Value;

use html2typst::parse_html;

/// The markdown definition.
static DEFINITION: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});
/// The markdown footnote definition.
static FOOTNOTE_DEFINITION: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});


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
            unimplemented!();
        }
        Node::Break(_) => {
            // Breakline.
            result += "\\ ";
        }
        Node::Code(node) => {
            result += format!(
                "```{}\n{}\n```\n",
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
            
        }
        Node::ImageReference(node) => {
            
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