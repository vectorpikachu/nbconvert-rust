#import "template.typ": *

#show: project.with(
  title: "Was",
  authors: (
    (name: "VectorPikachu", email: "lvhangzhou503@126.com", affiliation: "Peking University"),
    (name: "Pikachu", email: "oiakkakak@333.com", affiliation: "EECS"),
  ),
  date: "May 16, 2025",
)

#block[
#code-block("x = 5
print(x)", lang: "python", count: 1)
#output-block("5
")
]
#block[
= Let's try Latex

#mimath(`$$
\begin{aligned}
\int_0^{10} x^2 \mathrm{d}x
\end{aligned}
$$`)
#mi(`x`) is a number?

#underline[underline]

#underline[

hhhh

]

]
#block[
= Hello World

]
