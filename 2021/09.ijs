in=:"."0 'm' fread 'in/09.txt'
echo +/,(+in&*)lp=.in<<./((,-)1 0,.0 1)|.!._  in

lows=:($in) $ (idx=.1+i.#p)  (p=.I.,lp)}(#,in)$0
extend=: {{ y+.(in<9)*+./((,-)1 0,.0 1)|.!.0 y }}
echo */3{.\:~ +/"1,"2 idx="(0 2) extend^:_ lows

exit''
