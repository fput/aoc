oxy =: -:@# <: +/
echo (#.@:-.*#.) oxy in=. "."0 'm' fread 'in/03.txt'

co =: (-:@# > +/)`{.@.(<./=>./)
filter =: {{ y #~ a {~"1 {. I. (#>+/) a=.(="1 u"1&|:) y }}
echo (#. (oxy filter)^:_ in) * (#. (co filter)^:_ in)

exit''
