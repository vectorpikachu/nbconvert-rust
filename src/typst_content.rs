
pub struct TypstContent {
  pub content: String,
}

pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub affiliation: Option<String>,
}

pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

/// Escape all the special characters in code (raw string) to typst format.
/// But the parameter is a &Vec<String>
pub fn escape_vec_code(code: &Vec<String>) -> String {
    let mut result = String::new();
    for line in code {
        // Escape the special characters.
        let escaped_line = escape_content(line);
        result += &escaped_line;
    }
    result
}


/// Escape all the special characters in code (raw string) to typst format.
pub fn escape_code(code: &String) -> String {
    let mut result = String::new();
    for line in code.lines() {
        // Escape the special characters.
        let escaped_line = escape_content(line);
        result += &escaped_line;
    }
    result
}


/// Escape special characters in content to be used in typst format.
/// This function replaces:
/// - Backslashes (`\`) with double backslashes (`\\`)
/// - Double quotes (`"`) with escaped double quotes (`\"`)
/// - Backticks (`` ` ``) with escaped backticks (`\\``)
pub fn escape_content(content: &str) -> String {
    content
        .replace("\\", "\\\\") // Escape backslashes
        .replace("\"", "\\\"") // Escape double quotes
        .replace("`", "\\`")   // Escape backticks
}


impl TypstContent {
    /// Add the preface in the beginning of the content.
    pub fn add_preface(&mut self, title: &str, authors: &Vec<Author>, date: Option<&Date>) {
        let mut preface = String::new();
        preface += "#import \"template.typ\": *";
        preface += "\n\n";
        preface += "#show: project.with(\n";
        preface += format!("  title: \"{}\",\n", title).as_str();
        preface += "authors: (";
            
        for author in authors {
            preface += format!("(name: \"{}\", ", author.name).as_str();
            if let Some(email) = author.email.as_ref() {
                preface += format!("email: \"{}\", ", email).as_str();
            } else {
                preface += "email: none, ";
            }
            if let Some(affiliation) = author.affiliation.as_ref() {
                preface += format!("affiliation: \"{}\"", affiliation).as_str();
            } else {
                preface += "affiliation: none";
            }
            preface += "), ";
        }

        preface += "),\n";
        
        if let Some(date) = date {
            preface += format!("date: datetime(year: {}, month: {}, day: {})", date.year, date.month, date.day).as_str();
        } else {
            preface += "date: datetime.today()";
        }

        preface += ".display(\"[year]年[month padding:space]月[day padding:space]日\"),\n";
        preface += ")\n\n";

        self.content.insert_str(0, &preface);
    }
  }
