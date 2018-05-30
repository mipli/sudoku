use super::{Grid, Cell, SudokuError};

pub fn search(mut grid: Grid) -> Result<Grid, SudokuError> {
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
