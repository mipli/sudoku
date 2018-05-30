#![feature(test)]
extern crate test;

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

pub fn get_data(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut sudoku_data = String::new();
    file.read_to_string(&mut sudoku_data)?;
    Ok(sudoku_data.to_string())
}

pub fn get_batch_data(file: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(file)?;
    let mut sudoku_data = String::new();
    file.read_to_string(&mut sudoku_data)?;
    let lines = sudoku_data.lines().map(|line| line.to_string()).collect::<Vec<String>>();
    Ok(lines)
}

pub fn solve(sudoku_data: &str, ref_mode: bool) -> Result<Grid, SudokuError>{
    let grid = Grid::from_string(&sudoku_data)?;
    if ref_mode {
        ref_solver::search(grid)
    } else {
        move_solver::search(grid)
    }
}
