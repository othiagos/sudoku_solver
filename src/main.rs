mod sudoku_parse;

use std::{fs::File, io::Read};

fn main() {

    let mut file_handle = File::open("input/input1.txt").expect("Can not open the file");

    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content).expect("Can not be read the content of the file");

    let mut sudoku_lines: Vec<&str> = file_content.split_terminator(" ").collect();
    let sudoku_line_option = sudoku_parse::parse(sudoku_lines);
    sudoku_lines = sudoku_line_option.expect("Informed Sudoku is not valid");

    println!("{:?}", sudoku_lines);
    
}
