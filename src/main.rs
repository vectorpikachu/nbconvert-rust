mod error;
mod notebook;
mod media;


fn main() {
    let nb = notebook::read_notebook("./tests/hello.ipynb");
    println!("{:#?}", nb);
}
