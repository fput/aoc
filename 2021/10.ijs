in=: 'b' fread 'in/10.txt'
reduce=: {{(-.+./(+. |.!.0"1) (4 2$'[](){}<>') E."1 y)#y}}
o=: ( >r=: reduce^:_ each in) i."1 ')]}>' NB. Occurences in reduced.
echo +/(3 57 1197 25137)*+/0<(* (l=:{:$>r)&>"0) (]*]=<./)"1 o

s=: monad define"1 NB. Calculate score for completion of line y.
  {{x + 5*y}}/ +/(>:i.4)*'([{<' ="(0 1) y
)
echo (<.-:#ss){/:~ ss=.s >(I.*./"1 (l=o)) { r
exit''
