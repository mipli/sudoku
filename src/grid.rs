use std::fmt;

use super::{Cell, SudokuError};

#[derive(Clone)]
pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn from_string(data: &str) -> Result<Grid, SudokuError> {
        let mut grid = Grid::default();

        let mut index = 0;

        for c in data.chars() {
            match c.to_digit(10) {
                Some(v) => {
                    let x = index % 9;
                    let y = (index as f32 / 9.0).floor() as i32;
                    grid = match grid.assign(x, y, v as u8) {
                        Ok(g) => g,
                        Err(_) => return Err(SudokuError::InvalidGrid)
                    };
                    index += 1;
                },
                None if c == '.' => {
                    index += 1;
                },
                None => {}
            }
        }
        Ok(grid)
    }

    pub fn get(&self, x: i32, y: i32) -> &Cell {
        &self.cells[(x + y * 9) as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        self.cells[(x + y * 9) as usize] = cell;
    }

    pub fn peers(&self, x: i32, y: i32) -> Vec<((i32, i32), &Cell)> {
        let mut prs = vec![];
        for i in 0..9 {
            if i != y {
                prs.push(((x, i), self.get(x, i)));
            }
            if i != x {
                prs.push(((i, y), self.get(i, y)));
            }
        }

        // integer divison has implicit .floor()
        let x_offset = 3 * (x / 3);
        let y_offset = 3 * (y / 3);
        for i in 0..3 {
            for j in 0..3 {
                let nx = i + x_offset;
                let ny = j + y_offset;
                if nx != x && ny != y {
                    prs.push(((nx, ny), self.get(nx, ny)));
                }
            }
        }
        prs
    }

    pub fn assign(&self, x: i32, y: i32, value: u8) -> Result<Grid, SudokuError>{
        let to_remove: Vec<u8> = match self.get(x, y) {
            Cell::Known(n) if *n == value => vec![],
            Cell::Known(_) => return Err(SudokuError::AssigningToKnownCell),
            Cell::Options(nums) => {
                nums.iter().filter(|n| *n != &value).map(|n| *n).collect()
            }
        };
        let mut grid = self.clone();
        for num in to_remove {
            grid.eliminate(x, y, num)?;
        }
        Ok(grid)
    }

    pub fn eliminate(&mut self, x: i32, y: i32, value: u8) -> Result<(), SudokuError> {
        let original_cell = self.get(x, y).clone();
        match original_cell {
            Cell::Known(_) => return Ok(()),
            Cell::Options(ref nums) if !nums.contains(&value) => return Ok(()),
            Cell::Options(_) => {}
        }
        let cell = self.get(x, y).eliminate(value)?;
        self.set(x, y, cell.clone());
        match cell {
            Cell::Known(num) => {
                let peers: Vec<(i32, i32)> = self.peers(x, y).iter().map(|(pos, _)| *pos).collect();
                for (i, j) in peers {
                    self.eliminate(i, j, num)?;
                }
            },
            Cell::Options(_) => {}
        }
        Ok(())
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| {
            match cell {
                Cell::Known(_) => true,
                Cell::Options(_) => false
            }
        })
    }

    pub fn is_valid(&self) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                match self.get(x, y) {
                    Cell::Known(num) => {
                        let collision = self.peers(x, y).iter().any(|(_, peer)| {
                            match peer {
                                Cell::Known(n) if n == num => true,
                                _ => false
                            }
                        });
                        if collision {
                            return false;
                        }
                    },
                    Cell::Options(_) => {}
                }
            }
        }
        return true;
    }

    pub fn get_lowest_pos(&self) -> (i32, i32) {
        let test_cell = self.cells.iter().enumerate()
            .min_by(|a, b| {
                use std::cmp::Ordering;

                let a_value = match a.1 {
                    Cell::Known(_) => 10,
                    Cell::Options(nums) => nums.len()
                };
                let b_value = match b.1 {
                    Cell::Known(_) => 10,
                    Cell::Options(nums) => nums.len()
                };

                if a_value < b_value {
                    Ordering::Less
                } else if b_value < a_value {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
        match test_cell {
            Some((index, _)) => {
                let x = index % 9;
                let y = (index as f32 / 9.0).floor() as i32;
                (x as i32, y as i32)
            },
            None => unreachable!()
        }
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            cells: vec![Cell::default(); 81]
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = vec![];
        for y in 0..9 {
            if y > 0 && y % 3 == 0 {
                lines.push("-".repeat(11));
            }
            let mut line: Vec<String> = vec![];
            for x in 0..9 {
                if x > 0 && x % 3 == 0 {
                    line.push("|".to_string());
                }
                match self.get(x, y) {
                    Cell::Known(v) => {
                        line.push(format!("{:?}", v));
                    }, 
                    Cell::Options(nums) => {
                        line.push(format!("{:?}", nums));
                    }
                }
            }
            lines.push(line.join(""));
        }
        write!(f, "{}", lines.join("\n"))
    }
}
#[cfg(test)]
mod tests {
    use test::Bencher;

    use Grid;
    use super::{Cell};

    #[test]
    fn grid_peers() {
        let grid = Grid::default();
        let peers = grid.peers(6, 0);
        assert_eq!(peers.len(), 20);
    }

    #[test]
    fn grid_eliminate_to_known() {
        match Grid::from_string("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......") {
            Ok(mut grid) => {
                grid.set(0, 0, Cell::Options(vec![1, 2]));
                grid.eliminate(0, 0, 1 as u8).unwrap();
                match grid.get(0, 0) {
                    Cell::Known(n) => assert_eq!(2 as u8, *n),
                    Cell::Options(_) => assert!(false)
                }
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn grid_eliminate_first() {
        let mut grid = Grid::default();
        grid.eliminate(0, 0, 1 as u8).unwrap();
        match grid.get(0, 0) {
            Cell::Known(_) => assert!(false),
            Cell::Options(nums) => assert_eq!(*nums, [2, 3, 4, 5, 6, 7, 8, 9])
        }
    }

    #[test]
    fn grid_not_solved() {
        match Grid::from_string("......8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......") {
            Ok(grid) => {
                assert!(!grid.is_solved());
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn grid_get_lowest() {
        match Grid::from_string("8.....43.72.85.169.643.95.898..........26.914.41.9.786.32..16..61.42........3....") {
            Ok(grid) => {
                let lowest = grid.get_lowest_pos();
                assert_eq!(lowest, (1, 0));
                match grid.get(1, 0) {
                    Cell::Known(_) => assert!(false),
                    Cell::Options(nums) => assert_eq!(*nums, [5, 9])
                }
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn grid_solved() {
        match Grid::from_string("859612437723854169164379528986147352375268914241593786432981675617425893598736241") {
            Ok(grid) => assert!(grid.is_solved()),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn grid_assign() {
        let mut grid = Grid::default();
        grid = grid.assign(0, 0, 4 as u8).unwrap();
        match grid.get(0, 1) {
            Cell::Known(_) => assert!(false),
            Cell::Options(nums) => assert_eq!(8, nums.len())
        }
    }
    #[test]
    fn grid_assign_row() {
        let mut grid = Grid::default();
        grid = grid.assign(0, 0, 8 as u8).unwrap();
        grid = grid.assign(1, 0, 5 as u8).unwrap();
        grid = grid.assign(2, 0, 9 as u8).unwrap();
        grid = grid.assign(3, 0, 6 as u8).unwrap();
        grid = grid.assign(4, 0, 1 as u8).unwrap();
        grid = grid.assign(5, 0, 2 as u8).unwrap();
        grid = grid.assign(6, 0, 4 as u8).unwrap();
        grid = grid.assign(7, 0, 3 as u8).unwrap();
        grid = grid.assign(8, 0, 7 as u8).unwrap();
        match grid.get(0, 1) {
            Cell::Known(_) => assert!(false),
            Cell::Options(nums) => assert_eq!(*nums, [1, 2, 3, 4, 6, 7])
        }
    }

    #[bench]
    fn bench_grid_assign_row(b: &mut Bencher) {
        b.iter(|| {
            let mut grid = Grid::default();
            grid = grid.assign(0, 0, 8 as u8).unwrap();
            grid = grid.assign(1, 0, 5 as u8).unwrap();
            grid = grid.assign(2, 0, 9 as u8).unwrap();
            grid = grid.assign(3, 0, 6 as u8).unwrap();
            grid = grid.assign(4, 0, 1 as u8).unwrap();
            grid = grid.assign(5, 0, 2 as u8).unwrap();
            grid = grid.assign(6, 0, 4 as u8).unwrap();
            grid = grid.assign(7, 0, 3 as u8).unwrap();
            let _ = grid.assign(8, 0, 7 as u8).unwrap();
        });
    }
}
