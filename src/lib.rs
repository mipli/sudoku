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

#[macro_use]
extern crate bitflags;

mod cell;
use cell::{Cell, Value};

mod grid;
use grid::{Grid};

mod solver;

#[derive(Debug)]
pub enum SudokuError {
    AssigningToKnownCell,
    InvalidGrid,
    RemovingKnownValue
}

/// Solves the sudoku specified in `sudoku_data`. Returns a result with either the solved grid, or
/// a SudokuError
///
/// # Example
/// ```
/// # extern crate sudoku;
/// let grid = sudoku::solve("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......");
/// ```
pub fn solve(sudoku_data: &str) -> Result<Grid, SudokuError>{
    let grid = Grid::from_string(&sudoku_data)?;
    solver::search(grid)
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::solve;

    #[bench]
    fn bench_solve_ref(b: &mut Bencher) {
        let data = "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.";
        b.iter(|| {
            let _ = solve(&data);
        });
    }

    #[bench]
    fn bench_solve_move(b: &mut Bencher) {
        let data = "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.";
        b.iter(|| {
            let _ = solve(&data);
        });
    }
}
