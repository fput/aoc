t=:>{.in=: 'b' fread 'in/14.txt'
u=:~.t,,i ['p i'=: |:0 3 {"1 ;: >2}.in NB. unique elements, pairs, insert
pi=: {{ p -:"1 y}} NB. One-Index of given pair y in p
p2p=:(pi"1 (,.{."1 p),"1 i) + (pi"1 (i,"1 ,.{:"1 p)) NB. Transfer function: Pairs -> Pairs
iter=: {{+/p2p * y}} NB. Build next step
p2e=:u e."(1 0) {."1 p NB. One-Index of elements in given pair y
echo (>./-<./)(u e.{:t)++/p2e*"(1 0) i10=:iter^:10 (+/2 pi\ t)
echo (>./-<./)(u e.{:t)++/p2e*"(1 0)      iter^:30 i10
exit''
