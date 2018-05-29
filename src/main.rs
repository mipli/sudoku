mod cell;
use cell::{Cell};

mod grid;
use grid::{Grid};

#[derive(Debug)]
pub enum SudokuError {
    AssigningToKnownCell,
    InvalidGrid,
    RemovingKnownValue
}

fn search(grid: Grid) -> Result<Grid, SudokuError> {
    if !grid.is_valid() {
        return Err(SudokuError::InvalidGrid);
    }
    if grid.is_solved() {
        return Ok(grid);
    }

    let (x, y) = grid.get_lowest_pos();
    let cell = grid.get(x, y).clone();
    match cell {
        Cell::Known(_) => {
            Ok(grid.clone())
        },
        Cell::Options(nums) => {
            let num = nums[0];
            match try_assign(&grid, x, y, num) {
                Ok(g) => Ok(g),
                Err(_) => search(grid.eliminate(x, y, num).unwrap())
            }
        }
    }
}

fn try_assign(grid: &Grid, x: i32, y: i32, num: u8) -> Result<Grid, SudokuError> {
    let g = grid.assign(x, y, num)?;
    search(g)
}

fn solve() -> Result<Grid, SudokuError> {
    let grid = Grid::from_string("85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.").unwrap();
    search(grid)
}

fn main() {
    match solve() {
        Ok(grid) => println!("Solved:\n{:?}", grid),
        Err(err) => eprintln!("{:?}", err)
    }
}

