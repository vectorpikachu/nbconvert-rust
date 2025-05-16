#import "@preview/ansi-render:0.8.0": * // Render a terminal-like output.

#import "@preview/mitex:0.2.5": * // LaTex Support for Typst.

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
  let c = raw("[" + str(count) + "]:")
  let size = measure(c)
  box(height: 0pt, move(dx: -size.width, dy: -size.height - inset, c))
}

#let output-block(body) = context {
  v(0pt, weak: true)
  ansi-render(
    body,
    radius: radius,
    inset: inset,
    width: 100%
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
  set text(font: "Libertinus Serif", lang: "en")



  // Title row.
  align(center)[
    #block(text(weight: 700, 1.75em, title))
    #v(1em, weak: true)
    #date
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
        *#author.name* \
        #author.email \
        #author.affiliation
      ]),
    ),
  )

  // Main body.
  set par(justify: true)

  body
}