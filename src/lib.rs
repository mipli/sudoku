//! Sudoku Solver
//!
//! A simple sudoku solver. 
//!
//! Solves sudokus supplied as a string. Digits between `1` and `9` are interpreted as cell value, while `0` and `.` are interpreted as an empty cell. All other characters are ignored. That means that the two follow sudoku data strings result in the same board:
//! ```
//! let sudoku = "
//! 85.|..2|4..
//! 72.|...|..9
//! ..4|...|...
//! -----------
//! ...|1.7|..2
//! 3.5|...|9..
//! .4.|...|...
//! -----------
//! ...|.8.|.7.
//! .17|...|...
//! ...|.36|.4.";
//! ```
//! and
//! ```
//! let sudoku = "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.";
//! ```

#![feature(test)]
extern crate test;

mod cell;
use cell::{Cell};

mod grid;
use grid::{Grid};

mod move_solver;
mod ref_solver;

#[derive(Debug)]
pub enum SudokuError {
    AssigningToKnownCell,
    InvalidGrid,
    RemovingKnownValue
}

/// Solves the sudoku specified in `sudoku_data`. Returns a result with either the solved grid, or
/// a SudokuError
///
/// `ref_mode` is used to toggle between a solver using references internally in the grid, or one
/// that moves the grid around to the various scopes where it's needed. Purpose of these two modes
/// was to find any possible speed differences, and possible design issues with either of them
///
/// # Example
/// ```
/// # extern crate sudoku;
/// let grid = sudoku::solve("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......", false);
/// ```
pub fn solve(sudoku_data: &str, ref_mode: bool) -> Result<Grid, SudokuError>{
    let grid = Grid::from_string(&sudoku_data)?;
    if ref_mode {
        ref_solver::search(grid)
    } else {
        move_solver::search(grid)
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::solve;

    #[bench]
    fn bench_solve_ref(b: &mut Bencher) {
        let data = "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.";
        b.iter(|| {
            let _ = solve(&data, true);
        });
    }

    #[bench]
    fn bench_solve_move(b: &mut Bencher) {
        let data = "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.";
        b.iter(|| {
            let _ = solve(&data, false);
        });
    }
}
