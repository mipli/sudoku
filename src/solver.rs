use super::{Grid, Cell, SudokuError};

fn assign(grid: &Grid, x: i32, y: i32, value: u8) -> Result<Grid, SudokuError> {
    let to_remove: Vec<u8> = match grid.get(x, y) {
        Cell::Known(n) if *n == value => vec![],
        Cell::Known(_) => return Err(SudokuError::AssigningToKnownCell),
        Cell::Options(nums) => {
            nums.iter().filter(|n| *n != &value).map(|n| *n).collect()
        }
    };
    let mut grid = grid.clone();
    for num in to_remove {
        grid = eliminate(grid, x, y, num)?;
    }
    Ok(grid)
}

fn eliminate(mut grid: Grid, x: i32, y: i32, value: u8) -> Result<(Grid), SudokuError> {
    let original_cell = grid.get(x, y).clone();
    match original_cell {
        Cell::Known(_) => return Ok(grid),
        Cell::Options(ref nums) if !nums.contains(&value) => return Ok(grid),
        Cell::Options(_) => {}
    }
    let cell = grid.get(x, y).eliminate(value)?;
    grid.set(x, y, cell.clone());
    match cell {
        Cell::Known(num) => {
            let peers: Vec<(i32, i32)> = grid.peers(x, y).iter().map(|(pos, _)| *pos).collect();
            for (i, j) in peers {
                grid = eliminate(grid, i, j, num)?;
            }
        },
        Cell::Options(_) => {}
    }
    Ok(grid)
}

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
                    grid = eliminate(grid, x, y, num)?;
                    search(grid)
                }
            }
        }
    }
}

fn try_assign(grid: &Grid, x: i32, y: i32, num: u8) -> Result<Grid, SudokuError> {
    let new_grid = assign(grid, x, y, num)?;
    search(new_grid)
}
