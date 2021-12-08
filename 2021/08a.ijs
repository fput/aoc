echo +/,-.5 6 e.~ #&> _4 {."1 in=. ;:'m' fread 'in/08.txt'

DS=: {{>x{~I.y=#&>x}} NB. Digits with y segments from set x.
I=: {{x{~I.x e. y}}   NB. Set intersection.
decode=: 3 :0"1       NB. Decode a single entry.
  S=. (10{.y)&DS      NB. Digits with y segments from current entry.
  a=. (acf=. {. S 3) -. cf=. {. S 2 
  d=. (adg=. I/S 5) I bcdf=. {. S 4
  b=. bcdf -. d,f,c=. cf -. f=. I/cf, S 6
  e=. ({. S 7) -. a,b,c,d,f,g=. adg -. a,d
  con=. /:~ each (acf,b,e,g);cf;(adg,c,e);(adg,cf);bcdf;(adg,b,f);(adg,b,e,f);acf;(bcdf,a,e,g);(a,bcdf,g)
  10 #., >con&{{I.>((/:~y)&-:) &.> x}} each _4 {.y
)
echo +/decode in

exit''
