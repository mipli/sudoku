use std::fmt;
use super::{SudokuError};

#[derive(Clone)]
pub enum Cell {
    Known(u8),
    Options(Vec<u8>)
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Options(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Known(v) => {
                write!(f, "{}", v)
            }, 
            Cell::Options(v) => {
                write!(f, "{:?}", v)
            }
        }
    }
}

impl Cell {
    pub fn eliminate(&self, num: u8) -> Result<Cell, SudokuError> {
        match self {
            Cell::Known(_) => {
                Err(SudokuError::RemovingKnownValue)
            },
            Cell::Options(nums) => {
                if nums.contains(&num) {
                    let remains = nums.iter().filter(|n| *n != &num).map(|n| *n).collect::<Vec<u8>>();
                    if remains.len() == 1 {
                        Ok(Cell::Known(remains[0]))
                    } else {
                        Ok(Cell::Options(remains))
                    }
                } else {
                    Ok(self.clone())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use Cell;

    #[test]
    fn cell_eliminate() {
        let cell = Cell::Options(vec![1,2,3,4]);
        let res = cell.eliminate(2).unwrap();
        match res {
            Cell::Known(_) => assert!(false),
            Cell::Options(nums) => assert_eq!(nums, [1, 3, 4])
        }
    }

    #[test]
    fn cell_eliminate_to_known() {
        let cell = Cell::Options(vec![1,2]);
        let res = cell.eliminate(2).unwrap();
        match res {
            Cell::Known(n) => assert_eq!(n, 1),
            Cell::Options(_) => assert!(false)
        }
    }
}
