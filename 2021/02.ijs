'a b' =. |: ;: 'm' fread 'in/02.txt'
b =. ".b

s =. ({."1 a)=]
horizontal =. (b * s'f')
vertical =. b * (s'd') - (s'u')
echo (+/horizontal) * (+/vertical)
echo (+/horizontal) * +/(horizontal * +/\vertical)

exit''