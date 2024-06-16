mod sudoku;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Insufficient arguments");
    }

    let sudoku_lines = sudoku::input::read(&args[1]);
    let sudoku_lines = sudoku::input::parse(sudoku_lines);
    let sudoku_solution = sudoku::solver::solve('U', sudoku_lines);
    sudoku::out::print(sudoku_solution);
}
