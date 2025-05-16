use markdown::{mdast::{self, Node}, to_mdast, Constructs, ParseOptions};
use serde_json::Value;

/// Parse a given markdown to Typst contents.
pub fn parse_markdown(source: &Vec<String>, attachments: &Option<Value>) -> String {
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

    

    println!("{:#?}", ast);

    result
}


/// Recursively parse the markdown ast.
fn parse_ast(node: Node) -> String {
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
                node.lang.unwrap_or("text".to_string()),
                node.value
            ).as_str();
        }
        Node::Definition(node) => {
            // Defintion, like the Link in Typst.
            result += format!(
                "#link(\"{}\")[{}]\n",
                node.url,
                node.identifier
            ).as_str();
        }
        Node::Delete(node) => {
            // Delete Line.
            let mut children_result = String::new();
            for child in node.children {
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
            for child in node.children {
                children_result += parse_ast(child).as_str();
            }
            result += format!(
                "#emph[{}]",
                children_result
            ).as_str();
        }
        _ => unimplemented!()
    }

    result
}
