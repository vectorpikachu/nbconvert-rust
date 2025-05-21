#import "template.typ": *

#show: project.with(
  title: "Was",
  authors: (
    (name: "VectorPikachu", email: "lvhangzhou503@126.com", affiliation: "Peking University"),
    (name: "Pikachu", email: "oiakkakak@333.com", affiliation: "EECS"),
  ),
  date: "May 16, 2025",
)


#code-block("x = 5
print(x)", lang: "python", count: 1)
#output-block("5
")
#underline[]hhh is a num?
#figure(align(center, image("../tests/logo.jpg", width: 50%)))
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

kkkk
taysdhasadasf
- g
  - hhh
- hh
- 中文

+ 222
+ sadfsdf
+ dsgdfg

- safdsf
- dsfsdf
- dsfsdfs