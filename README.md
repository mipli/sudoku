**Sudoku Solver**

A simple Sudoku solver, based upon the [Solving Every Sudoku Puzzle](http://norvig.com/sudoku.html) by Peter Norvig.

CLI supports commands:
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

Benchmarks run at [fa2d510](https://github.com/mipli/sudoku/commit/fa2d510) with `bitflags` implemented, allowing grid and cell structures to be copied
```
test grid::tests::bench_grid_with_assigned_row ... bench:       5,215 ns/iter (+/- 769)
test tests::bench_solve                        ... bench:   2,170,605 ns/iter (+/- 144,235)
```

Removed the `cell::eliminate` benchmark with the implementation of `bitflags`, since it the new version is a straight XOR'ing of the values and no need to benchmark that.

Benchmarks run at [2015acb](https://github.com/mipli/sudoku/commit/2015acb) before `bitflags` was used and we were still cloning the grid
```
test cell::tests::bench_cell_eliminate         ... bench:         115 ns/iter (+/- 6)
test grid::tests::bench_grid_with_assigned_row ... bench:      64,296 ns/iter (+/- 4,488)
test tests::bench_solve_move                   ... bench:  21,243,889 ns/iter (+/- 844,765)
test tests::bench_solve_ref                    ... bench:  18,349,852 ns/iter (+/- 561,355)
```
