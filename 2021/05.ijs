in=. 'm' fread 'in/05.txt'
p2=: ;/ (2,2)$"1 ".> 0 2 5 7 {"1 ;: in
p1=: p2 #~ -.*./"1|*-/"2 > p2
ex=: {{(0{y) +"1 |: (*l) * i."0  (0~:*l)*>:|l=. --/y}}
dp=: {{#~.(-.~:f) # f=. ;ex&.>y}}
echo dp p1
echo dp p2
exit''

