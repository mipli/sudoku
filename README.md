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

I implemented the solver two ways. One where the grid is passed around using references, and a version where the grid is moved around. The main reason for this was to see how each worked, and to see if there was any performance difference. There does not seem to be any difference in performance at the moment.

Biggest slowdown at the moment seem to be caused by the `grid.clone` call done when assigning new values. Removing the `Vec` storage in the cells, and converting to using a bit mask instead should make it possible to get rid of that clone call.


Benchmarks run at [da13641](https://github.com/mipli/sudoku/commit/da13641b7cd7b7b05216e25ee69fa598d8e2b407)
```
test cell::tests::bench_cell_eliminate  ... bench:         152 ns/iter (+/- 10)
test grid::tests::bench_grid_assign_row ... bench:      63,542 ns/iter (+/- 5,287)
test tests::bench_solve                 ... bench:  19,157,626 ns/iter (+/- 471,592)
````
