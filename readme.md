# Peg Board Game Solver

You know those 15 peg gameboards?

They were on the tables at a leadership offsite I attended.

So, yeah, I had to write a solver for it.

This program brute-forces all games (1.6M) for all the unique starting positions.

Bottom-line: start with the middle-side peg empty, it is your best chance to win.

Here is the output:
```
No Peg 0 (corner)
  Pegs: 1, Games: 26727
  Pegs: 2, Games: 127451
  Pegs: 3, Games: 229445
  Pegs: 4, Games: 109526
  Pegs: 5, Games: 13241
  Pegs: 6, Games: 697
  Pegs: 7, Games: 309
  Pegs: 8, Games: 6
  Total games: 507402
No Peg 1 (side, one in)
  Pegs: 1, Games: 11947
  Pegs: 2, Games: 54656
  Pegs: 3, Games: 117859
  Pegs: 4, Games: 53532
  Pegs: 5, Games: 7551
  Pegs: 6, Games: 381
  Pegs: 7, Games: 141
  Pegs: 8, Games: 9
  Total games: 246076
No Peg 3 (middle side)
  Pegs: 1, Games: 58592
  Pegs: 2, Games: 214816
  Pegs: 3, Games: 365833
  Pegs: 4, Games: 167899
  Pegs: 5, Games: 19047
  Pegs: 6, Games: 1339
  Pegs: 7, Games: 310
  Pegs: 8, Games: 37
  Total games: 827873
No Peg 4 (third row middle)
  Pegs: 1, Games: 649
  Pegs: 2, Games: 12514
  Pegs: 3, Games: 34332
  Pegs: 4, Games: 24224
  Pegs: 5, Games: 2684
  Pegs: 6, Games: 290
  Pegs: 7, Games: 66
  Pegs: 8, Games: 7
  Pegs: 10, Games: 2
  Total games: 74768
Total games: 1656119
Total time:  190.5111ms
```