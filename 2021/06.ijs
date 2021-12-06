in=. 'm' fread 'in/06.txt'
f=. +/"1 (i.9) =/ ".;._1 ',' , {. in
adv=. ((6=i.9) * {.) + 1&|. NB. Advance to next day
echo +/f80=.adv^:80 f       NB. Part 1: 80 days
echo +/adv^:(256-80) f80    NB. Part 2: 256 days
exit''
