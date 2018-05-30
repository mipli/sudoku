#![feature(test)]
extern crate test;

extern crate getopts;
extern crate sudoku;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
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
        match sudoku::get_batch_data(&input_file) {
            Ok(sudoku_data) => {
                for data in &sudoku_data {
                    match sudoku::solve(&data, ref_mode) {
                        Ok(_) => {},
                        Err(_) => {}
                    }
                }
                println!("Done");
            }, 
            Err(_) => { }
        }
    } else {
        match sudoku::get_data(&input_file) {
            Ok(sudoku_data) => {
                match sudoku::solve(&sudoku_data, ref_mode) {
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
    use sudoku::{get_data, solve};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let data = get_data("data/easy").unwrap();
        b.iter(|| {
            let _ = solve(&data, true);
        });
    }
}
