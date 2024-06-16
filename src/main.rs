mod sudoku;

use std::env;
use std::{fs::File, io::Read};

fn main() {

    let file_path: Vec<String> = env::args().collect();

    if file_path.len() < 2 {
        panic!("Insufficient arguments");
    }

    let mut file_handle = File::open(file_path[1].to_string()).expect("Cannot open the file");

    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content).expect("Cannot read the content of the file");

    let sudoku_lines: Vec<&str> = file_content.split_terminator(" ").collect();
    let sudoku_line_option = sudoku::input_parse::parse(sudoku_lines);
    let sudoku_lines = sudoku_line_option.expect("The provided Sudoku is not valid");

    let (elapse_time, sudoku_option) = sudoku::solver::solve('U', sudoku_lines);

    println!("elapse_time: {}", elapse_time);

    let sudoku = match sudoku_option {
        Some(sudoku) => sudoku,
        None => panic!("Can not find a solution")
    };

    println!("expanded_nodes: {}", sudoku.0);
    for line in sudoku.1 {
        println!("{:?}", line)
    }
    println!("")
    
}
