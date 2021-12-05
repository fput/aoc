NB. Advent of Code 2021 - Day 5
NB.       Florentin Putz

in=. 'm' fread 'in/05.txt'
paths=: ".> 0 2 5 7 {"1 ;: in     NB. Extract the coordinates.
p2=: ;/ (2,2)$"1 paths            NB. Box all paths.
p1=: p2 #~ -.*./"1|*-/"2 > p2     NB. Remove diagonal paths.

NB. Expand paths y to a list of coordinates.
ex=: monad define
  dir=. --/y                      NB. Vector from (x1,y1) -> (x2,y2),
  idir=. (0 ~: *dir) * (>: | dir) NB. Increment nonzero dirs.
  rsteps=. (*dir) * (i."0 idir)   NB. Relative steps along the path,
  asteps=. (0{y) +"1 |: rsteps    NB. Absolute steps along the path.
)

NB. Count coordinates appearing two or more times.
dp=: monad define
  coords=. ;ex&.>y                NB. Expand each path to coordinates.
  rdup=. (-.~:coords) # coords    NB. Remove unique coordinates.
  cu=. #~.rdup                    NB. Count remaining set.
  (#~.)-# coords
)

echo dp p1                        NB. Part 1
echo dp p2                        NB. Part 2