mod cell;
use cell::{Cell};

mod grid;
use grid::{Grid};

mod solver;

#[derive(Debug)]
pub enum SudokuError {
    AssigningToKnownCell,
    InvalidGrid,
    RemovingKnownValue
}

fn search(mut grid: Grid) -> Result<Grid, SudokuError> {
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
            Ok(grid)
        },
        Cell::Options(nums) => {
            let num = nums[0];
            match try_assign(&grid, x, y, num) {
                Ok(g) => Ok(g),
                Err(_) => {
                    grid.eliminate(x, y, num)?;
                    search(grid)
                }
            }
        }
    }
}

fn try_assign(grid: &Grid, x: i32, y: i32, num: u8) -> Result<Grid, SudokuError> {
    let new_grid = grid.assign(x, y, num)?;
    search(new_grid)
}

pub fn solve(data: &str) -> Result<Grid, SudokuError> {
    let grid = Grid::from_string(data)?;
    solver::search(grid)
}

fn main() {
    // match solve("85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.") {
    match solve(".....6....59.....82....8....45........3........6..3.54...325..6..................") { // hard
    // match solve(".....5.8....6.1.43..........1.5........1.6...3.......553.....61........4.........") { // impossible
        Ok(grid) => println!("Solved:\n{:?}", grid),
        Err(err) => eprintln!("{:?}", err)
    }
}

#[cfg(test)]
mod tests {
    use super::{solve};

    #[test]
    fn test_solve() {
        match solve("85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.") {
            Ok(grid) => assert!(grid.is_solved() && grid.is_valid()),
            Err(_) => assert!(false)
        }
    }
}
