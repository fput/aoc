NB. Advent of Code 2021 - Day 4
NB.       Florentin Putz

in=. 'm' fread 'in/04.txt'        NB. Read input file.
]draws =: ".;._1 ',' , {. in      NB. List of drawn numbers.
]boards =: }."_1 (_6 [\ ". }. in) NB. List of bingo boards.

NB. Bingo on marked board y? (T/F)
NB.   y is marked bingo board      (rank 2 bool)
NB.   Returns true or false        (rank 0 bool)
win =: [: +./ *./"1 , *./
NB.  Two forks: (AND each row) OR (AND each col)

NB. The winning turn for each bingo board.
]winturns =: >:(|:win"2 boards&e.\ draws) i."1 (1)
NB. boards&e. draws              NB. Mark each board at the state after all draws.
NB. boards&e.\ draws             NB. Mark each board, for EACH individual draw.
NB. win"2 boards&e.\ draws       NB. For each board and draw, determine if the board has bingo.
NB. >:( ... ) i."1 (1)           NB. Determine the (first) draw each board wins.

NB. Calculates the score for a winning board on turn y.
NB. The score is defined as the sum of unmarked numbers on
NB. the bingo board, multiplied with the number drawn in
NB. winning turn.
NB.   y is a turn number           (rank 0 int)
NB.   Returns the score            (rank 0 int)
score =: monad define
  wb =: boards {~ winturns i. y  NB. Get the winning board.
  d=. y {. draws                 NB. Get all draws until the winning draw
  (+/ d -.~ ,wb) * {: d          NB. (sum of unmarked) * last draw
  NB.  (d -.~ ,wb) is the set of numbers on the board not contained in d
)

echo score <./winturns           NB. Part 1
echo score >./winturns           NB. Part 2
