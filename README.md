# nbconvert-rust

考虑到Python库`nbconvert`对于中文的支持很差, 而且转换为PDF需要`Tex`作为渲染引擎, 比较不方便.
所以我们现在使用Rust来实现一个把Jupyter notebook渲染为PDF的工具.

## Usage

```
This tool reads a Jupyter Notebook (.ipynb) file, converts it to Typst format, and compiles it to a PDF document. You can specify the title, authors, emails, affiliations, and date of the document. The output will be saved as a Typst file (.typ) and a PDF file.

Tips: You need to install the typst command line tool. You also need to install several fonts, including: "New Computer Modern", "SimSun", "KaiTi", "Maple Mono NF".


Usage: nbconvert-rust.exe [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>
          Input notebook path (.ipynb)

  -o, --output <OUTPUT>
          Output typst file and pdf file path (.typ)

      --title <TITLE>
          Title of the document

          [default: "Untitled Notebook"]

      --authors <AUTHORS>
          Author list, e.g. "Zhang San, Si Li"

          [default: Anonymous]

      --emails <EMAILS>
          Emails for authors, split by ',', or empty

          [default: ]

      --affiliations <AFFILIATIONS>
          Affiliations for authors, split by ',', or empty

          [default: ]

      --date <DATE>
          Date in format YYYY-MM-DD

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
