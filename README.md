**Sudoku Solver**

A simple Sudoku solver, based upon the [Solving Every Sudoku Puzzle](http://norvig.com/sudoku.html) by Peter Norvig.

CLI supports commands:
 * `--ref` to change solver to use references instead of moving objects arounds
 * `--batch` to specify that input file is batch file, one sudoku puzzle per line

The file reader reads sudoku files in most normal formats. Digits between `1` and `9` are interpreted as cell value, while `0` and `.` are interpreted as an empty cell. All other characters are ignored. That means that the two follow sudoku data strings result in the same board:
```
85.|..2|4..
72.|...|..9
..4|...|...
-----------
...|1.7|..2
3.5|...|9..
.4.|...|...
-----------
...|.8.|.7.
.17|...|...
...|.36|.4.
```
and
```
85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.
```

For batch reading of multiple sudoku files you need to specify one puzzle per line.

I implemented the solver two ways. One where the grid is passed around using references, and a version where the grid is moved around. The main reason for this was to see how each worked, and to see if there was any performance difference. The reference based solution seems to be about 16% faster.

Biggest slowdown at the moment seem to be caused by the `grid.clone` call done when assigning new values. Removing the `Vec` storage in the cells, and converting to using a bit mask instead should make it possible to get rid of that clone call.


Benchmarks run at [2015acb](https://github.com/mipli/sudoku/commit/2015acb)
```
test cell::tests::bench_cell_eliminate         ... bench:         115 ns/iter (+/- 6)
test grid::tests::bench_grid_with_assigned_row ... bench:      64,296 ns/iter (+/- 4,488)
test tests::bench_solve_move                   ... bench:  21,243,889 ns/iter (+/- 844,765)
test tests::bench_solve_ref                    ... bench:  18,349,852 ns/iter (+/- 561,355)
````
