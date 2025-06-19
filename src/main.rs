mod error;
mod notebook;
mod media;
mod typst_content;
mod markdown;

use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, process::Command};

use notebook::convert_notebook;

use crate::{error::Result, typst_content::Author};

use clap::Parser;

/// CLI Arguments
#[derive(Parser, Debug)]
#[command(author, version, about = "Convert a Jupyter Notebook to Typst, and compile it to PDF.", long_about = "This tool reads a Jupyter Notebook (.ipynb) file, converts it to Typst format, and compiles it to a PDF document. You can specify the title, authors, emails, affiliations, and date of the document. The output will be saved as a Typst file (.typ) and a PDF file.

Tips: You need to install the typst command line tool. You also need to install several fonts, including: \"New Computer Modern\", \"SimSun\", \"KaiTi\", \"Maple Mono NF\".
")]
struct Args {
    /// Input notebook path (.ipynb)
    #[arg(short, long)]
    input: PathBuf,

    /// Output pdf file path (.typ)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Title of the document
    #[arg(long, default_value = "Untitled Notebook")]
    title: String,

    /// Author list, e.g. "Zhang San, Si Li"
    #[arg(long, default_value = "Anonymous")]
    authors: String,

    /// Emails for authors, split by ',', or empty
    #[arg(long, default_value = "")]
    emails: String,

    /// Affiliations for authors, split by ',', or empty
    #[arg(long, default_value = "")]
    affiliations: String,

    /// Date in format YYYY-MM-DD
    #[arg(long)]
    date: Option<String>,
}


fn main() -> Result<()> {

    let args = Args::parse();


    let title = args.title;
    
    let mut authors: Vec<Author> = Vec::new();
    for ((author, email), affiliation) in args.authors.split(',').zip(args.emails.split(',')).zip(args.affiliations.split(',')) {
        authors.push(Author {
            name: author.trim().to_string(),
            email: if email.trim().is_empty() { None } else { Some(email.trim().to_string()) },
            affiliation: if affiliation.trim().is_empty() { None } else { Some(affiliation.trim().to_string()) },
        });
    }

    let date = args.date.as_ref().map(|d| {
        let parts: Vec<&str> = d.split('-').collect();
        if parts.len() == 3 {
            Some(typst_content::Date {
                year: parts[0].parse().unwrap_or(2023),
                month: parts[1].parse().unwrap_or(1),
                day: parts[2].parse().unwrap_or(1),
            })
        } else {
            None
        }
    }).flatten();

    let nb = notebook::read_notebook(&args.input)?;

    let mut typst_content = convert_notebook(&nb)?;

    // Add preface to the typst content
    typst_content.add_preface(&title, &authors, date.as_ref());

    let pdf_output = if let Some(output) = &args.output {
        output.with_extension("pdf")
    } else {
        // Default output path is the same as input, but with .typ extension
        args.input.with_extension("pdf")
    };

    create_template(&pdf_output);

    let typ_output = pdf_output.with_extension("typ");
    let mut file = File::create(&typ_output)?;
    file.write_all(typst_content.content.as_bytes())?;

    // Compile the typst file to PDF
    let status = Command::new("typst")
        .arg("compile")
        .arg(&typ_output)
        .arg(&pdf_output)
        .status()
        .expect("Failed to execute typst compiler");

    if status.success() {
        println!("PDF successfully compiled: {}", pdf_output.display());
    } else {
        eprintln!("PDF compilation failed");
    }

    Ok(())
}


/// Ensure template.typ exists in the output directory.
/// Return the full path to the created or existing template.
fn create_template(output_path: &Path) -> PathBuf {
    let dir = output_path
        .parent()
        .expect("Output path must have a parent directory");

    let template_path = dir.join("template.typ");

    if !template_path.exists() {
        fs::write(&template_path, r#"#import "@preview/ansi-render:0.8.0": * // Render a terminal-like output.

#import "@preview/mitex:0.2.5": * // LaTex Support for Typst.

#import "@preview/cuti:0.3.0": show-cn-fakebold // Fake bold for CJK

#let radius = 3pt
#let inset = 8pt

// Form a code block, with execution count to its left.
#let code-block(body, lang: "python", count: none) = context {
  block(
    raw(body, lang: lang),
    fill: luma(230),
    inset: inset,
    radius: radius,
    width: 100%
  )
  v(0pt, weak: true)
    let c = if count == none { raw("[ ]:") } else { raw("[" + str(count) + "]:") }
  let size = measure(c)
  box(height: 0pt, move(dx: -size.width, dy: -size.height - inset, c))
}

#let output-block(body) = {
  v(0pt, weak: true)
  ansi-render(
    body,
    radius: radius,
    inset: inset,
    width: 100%,
    font: ("Maple Mono NF")
  )
}

#let block-quote(body) = context {
  let size = measure(body)
  grid(
    columns: (4pt, auto),
    rows: auto,
    gutter: 0pt,
    rect(
      fill: luma(180),
      height: size.height + 2 * inset,
      radius: (left: radius),
    ),
    block(
      fill: luma(240),
      height: size.height + 2 * inset,
      inset: inset,
      radius: (right: radius),
      width: 100%,
      body,
    ),
  )
}

// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(title: "", authors: (), date: none, body) = {
  // Set the document's basic properties.
  set document(author: authors.map(a => a.name), title: title)
  set page(numbering: "1", number-align: center)
  set text(font: ("New Computer Modern", "SimSun"), lang: "en")
  show raw: set text(font: ("Maple Mono NF"))
  show: show-cn-fakebold
  show emph: set text(font: ("New Computer Modern", "KaiTi"))


  // Title row.
  align(center)[
    #block(text(weight: 700, 1.75em, title))
  ]

  // Author information.
  pad(
    top: 0.5em,
    bottom: 0.5em,
    x: 2em,
    grid(
      columns: (1fr,) * calc.min(3, authors.len()),
      gutter: 1em,
      ..authors.map(author => align(center)[
        *#author.name*
        #if author.email != none { linebreak(); author.email }
        #if author.affiliation != none { linebreak(); author.affiliation }
      ]),
    ),
  )

  align(center)[
    #v(1em, weak: true)
    #date
  ]

  // Main body.
  set par(justify: true)

  body
}
"#)
            .expect("Failed to create template.typ");
        println!("Created template at: {}", template_path.display());
    } else {
        println!("Using existing template: {}", template_path.display());
    }

    template_path
}