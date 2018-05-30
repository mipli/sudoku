#![feature(test)]
extern crate getopts;
extern crate test;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn get_data(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut sudoku_data = String::new();
    file.read_to_string(&mut sudoku_data)?;
    Ok(sudoku_data.to_string())
}

fn get_batch_data(file: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(file)?;
    let mut sudoku_data = String::new();
    file.read_to_string(&mut sudoku_data)?;
    let lines = sudoku_data.lines().map(|line| line.to_string()).collect::<Vec<String>>();
    Ok(lines)
}

fn solve(sudoku_data: &str, ref_mode: bool) -> Result<Grid, SudokuError>{
    let grid = Grid::from_string(&sudoku_data)?;
    if ref_mode {
        ref_solver::search(grid)
    } else {
        move_solver::search(grid)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("b", "batch", "input file contains a batch of puzzles");
    opts.optflag("r", "ref", "use reference based solver");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { 
            println!("{}", f.to_string());
            print_usage(&program, opts);
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let ref_mode = matches.opt_present("r");
    let input_file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    if matches.opt_present("b") {
        match get_batch_data(&input_file) {
            Ok(sudoku_data) => {
                for data in &sudoku_data {
                    match solve(&data, ref_mode) {
                        Ok(_) => {},
                        Err(_) => {}
                    }
                }
                println!("Done");
            }, 
            Err(_) => { }
        }
    } else {
        match get_data(&input_file) {
            Ok(sudoku_data) => {
                match solve(&sudoku_data, ref_mode) {
                    Ok(grid) => println!("Solved\n{:?}", grid),
                    Err(err) => eprintln!("Error: {:?}", err),
                }
            }, 
            Err(_) => { }
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::{get_data, solve};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let data = get_data("data/easy").unwrap();
        b.iter(|| {
            let _ = solve(&data, true);
        });
    }
}
