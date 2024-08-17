mod sudoku;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Insufficient arguments");
    }

    let sudoku_lines = sudoku::io::read(&args[1]);
    let sudoku_lines = sudoku::io::parse(sudoku_lines);
    let sudoku_solution = sudoku::solver::solve('U', sudoku_lines);
    sudoku::io::print(sudoku_solution);
}
