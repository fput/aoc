in=: 'm' fread 'in/04.txt'
draws=: ".;._1 ',' , {. in
boards=: }."_1 (_6 [\ ". }. in)
win=: [: +./ *./"1 , *./
winturns=: >:(|:win"2 boards&e.\ draws) i."1 (1)
score=: {{(+/d-.~,boards{~winturns i.y)*{:d=.y{.draws}}

echo score <./winturns  NB. Part 1
echo score >./winturns  NB. Part 2

exit''