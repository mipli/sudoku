use super::{Grid, Value, SudokuError};

pub fn search(mut grid: Grid) -> Result<Grid, SudokuError> {
    if !grid.is_valid() {
        return Err(SudokuError::InvalidGrid);
    }
    if grid.is_solved() {
        return Ok(grid);
    }

    let (x, y) = grid.get_lowest_pos();
    let cell = grid.get(x, y).clone();
    if cell.is_known() {
        Ok(grid)
    } else {
        let num = cell.nums()[0];
        match try_assign(&grid, x, y, num) {
            Ok(g) => Ok(g),
            Err(_) => {
                grid.eliminate(x, y, Value::from(num))?;
                search(grid)
            }
        }
    }
}

fn try_assign(grid: &Grid, x: i32, y: i32, num: u32) -> Result<Grid, SudokuError> {
    let new_grid = grid.with_assigned(x, y, num)?;
    search(new_grid)
}
