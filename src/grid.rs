use std::fmt;

use super::{Value, Cell, SudokuError};

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
                Some(v) if v > 0 => {
                    let x = index % 9;
                    let y = (index as f32 / 9.0).floor() as i32;
                    grid = match grid.with_assigned(x, y, v as u32) {
                        Ok(g) => g,
                        Err(_) => return Err(SudokuError::InvalidGrid)
                    };
                    index += 1;
                },
                Some(v) if v == 0 => {
                    index += 1;
                },
                None if c == '.' => {
                    index += 1;
                },
                Some(_) | None => {}
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

    pub fn with_assigned(&self, x: i32, y: i32, value: u32) -> Result<Grid, SudokuError>{
        let cell = self.get(x, y);
        if cell.is_known() && !cell.value.intersects(Value::from(value)) {
            return Err(SudokuError::AssigningToKnownCell)
        }

        println!("cell nums: {:?}", cell.nums());
        let to_remove: Vec<u32> = cell.nums().iter().filter(|n| *n != &value).map(|n| *n).collect();
        println!("To remove: {:?}", to_remove);
        let mut grid = self.clone();
        for num in to_remove {
            grid.eliminate(x, y, Value::from(num))?;
        }
        Ok(grid)
    }

    pub fn eliminate(&mut self, x: i32, y: i32, value: Value) -> Result<(), SudokuError> {
        let original_cell = self.get(x, y).clone();
        if original_cell.count() == 1 {
            println!("cell {:?}, {}, {}", original_cell, x, y);
            return Ok(());
        }
        if !original_cell.value.intersects(value) {
            return Ok(());
        }

        let mut cell = self.get(x, y).clone();
        cell.eliminate(value);
        self.set(x, y, cell.clone());
        if cell.count() == 1 {
            let peers: Vec<(i32, i32)> = self.peers(x, y).iter().map(|(pos, _)| *pos).collect();
            for (i, j) in peers {
                self.eliminate(i, j, cell.value)?;
            }
        }
        self.set(x, y, cell);
        Ok(())
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| {
            cell.count() == 1
        })
    }

    pub fn is_valid(&self) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                let cell = self.get(x, y);
                if cell.count() == 1 {
                    let collision = self.peers(x, y).iter().any(|(_, peer)| {
                        if peer.is_known() {
                            return cell.value.intersects(peer.value);
                        }
                        return false;
                    });
                    return !collision;
                }
            }
        }
        return true;
    }

    pub fn get_lowest_pos(&self) -> (i32, i32) {
        let test_cell = self.cells.iter().enumerate()
            .min_by(|a, b| {
                use std::cmp::Ordering;

                let a_val = match a.1.count() {
                    1 => 10,
                    v => v + 10
                };

                let b_val = match b.1.count() {
                    1 => 10,
                    v => v + 10
                };

                if a_val < b_val {
                    Ordering::Less
                } else if b_val < a_val {
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
                line.push(format!("{:?}", self.get(x, y)));
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
    use super::{Value, Cell};

    #[test]
    fn grid_peers() {
        let grid = Grid::default();
        let peers = grid.peers(6, 0);
        assert_eq!(peers.len(), 20);
    }

    #[test]
    fn grid_with_assigned() {
        let mut grid = Grid::default();
        grid = grid.with_assigned(0, 0, 4).unwrap();
        let cell = grid.get(0, 0);
        assert!(cell.is_known());
        assert!(cell.value.intersects(Value::FOUR));
    }


    #[test]
    fn grid_with_assigned_row() {
        let mut grid = Grid::default();
        grid = grid.with_assigned(0, 0, 8).unwrap();
        grid = grid.with_assigned(1, 0, 5).unwrap();
        grid = grid.with_assigned(2, 0, 9).unwrap();
        grid = grid.with_assigned(3, 0, 6).unwrap();
        grid = grid.with_assigned(4, 0, 1).unwrap();
        grid = grid.with_assigned(5, 0, 2).unwrap();
        grid = grid.with_assigned(6, 0, 4).unwrap();
        grid = grid.with_assigned(7, 0, 3).unwrap();
        grid = grid.with_assigned(8, 0, 7).unwrap();
        let cell = grid.get(0, 1);
        assert_eq!(cell.count(), 6);
        assert_eq!(cell.nums(), vec![1, 2, 3, 4, 6, 7]);
    }

    #[test]
    fn grid_from_string() {
        match Grid::from_string("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......") {
            Ok(grid) => {
                let c00 = grid.get(0, 0);
                assert_eq!(c00.count(), 1);
                let c01 = grid.get(0, 1);
                assert_eq!(c01.count(), 5);
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn grid_eliminate_one() {
        match Grid::from_string("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......") {
            Ok(mut grid) => {
                grid.set(0, 0, Cell::default());
                grid.eliminate(0, 0, Value::from(1)).unwrap();
                let cell = grid.get(0, 0);
                println!("{:?}", cell);
                assert_eq!(cell.count(), 8);
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn grid_eliminate_to_known() {
        match Grid::from_string("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......") {
            Ok(mut grid) => {
                grid.set(0, 0, Cell::default());
                grid.eliminate(0, 0, Value::from(1)).unwrap();
                grid.eliminate(0, 0, Value::from(3)).unwrap();
                grid.eliminate(0, 0, Value::from(4)).unwrap();
                grid.eliminate(0, 0, Value::from(5)).unwrap();
                grid.eliminate(0, 0, Value::from(6)).unwrap();
                grid.eliminate(0, 0, Value::from(7)).unwrap();
                grid.eliminate(0, 0, Value::from(8)).unwrap();
                grid.eliminate(0, 0, Value::from(9)).unwrap();
                let cell = grid.get(0, 0);
                println!("{:?}", cell);
                assert!(cell.is_known());
                assert!(cell.value.intersects(Value::TWO));
                assert_eq!(cell.nums(), vec![2]);
            },
            Err(_) => assert!(false)
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
        match Grid::from_string(".................................................................................") {
            Ok(mut grid) => {
                grid.eliminate(1, 1, Value::from(1)).unwrap();
                grid.eliminate(1, 1, Value::from(2)).unwrap();
                grid.eliminate(1, 1, Value::from(3)).unwrap();
                grid.eliminate(2, 2, Value::from(1)).unwrap();
                grid.eliminate(2, 2, Value::from(2)).unwrap();
                let lowest = grid.get_lowest_pos();
                assert_eq!(lowest, (1, 1));
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

    #[bench]
    fn bench_grid_with_assigned_row(b: &mut Bencher) {
        b.iter(|| {
            let mut grid = Grid::default();
            grid = grid.with_assigned(0, 0, 8).unwrap();
            grid = grid.with_assigned(1, 0, 5).unwrap();
            grid = grid.with_assigned(2, 0, 9).unwrap();
            grid = grid.with_assigned(3, 0, 6).unwrap();
            grid = grid.with_assigned(4, 0, 1).unwrap();
            grid = grid.with_assigned(5, 0, 2).unwrap();
            grid = grid.with_assigned(6, 0, 4).unwrap();
            grid = grid.with_assigned(7, 0, 3).unwrap();
            let _ = grid.with_assigned(8, 0, 7).unwrap();
        });
    }
}
