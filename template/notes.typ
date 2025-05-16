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