#import "template.typ": *

#show: project.with(
  title: "Untitled Notebook",
authors: ((name: "Anonymous", email: none, affiliation: none), ),
date: datetime.today().display("[year]年[month padding:space]月[day padding:space]日"),
)

#code-block("x = 5
print(x)", lang: "python", count: 1)

#output-block("5")

hhh is a num?

#figure(align(center, image("./logo.jpg", width: 50%)))

[x]

= Hello World


下面是一个表格。

#table(
  columns: 3,
  align: (auto, auto, auto),
  table.header(
    [Column A], [Column B], [Column C], 
  ),
  [A1], [B1], [C1], 
  [A2], [B2], [C2], 
  [A3], [B3], [C3], 
)


[x]

kkkk

taysdhasadasf

- g
  - hhh\ 
    h\ 
    h
- hh
- 中文


+ 222
+ sadfsdf
+ dsgdfg


- safdsf
- dsfsdf
- dsfsdfs


