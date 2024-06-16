use std::{fs::File, io::Read};

enum Number {
    Zero = 0x30,
    Nine = 0x39,
}

fn is_digit_in_range(number: u8) -> bool {
    number >= Number::Zero as u8 && number <= Number::Nine as u8
}

fn check_numbers(numbers: String) -> Option<Vec<u8>> {
    let mut new_vec: Vec<u8> = Vec::new();
    for n in numbers.bytes() {
        if !is_digit_in_range(n) {
            return None;
        }

        new_vec.push(n - Number::Zero as u8);
    }

    Some(new_vec)
}

fn check_lines(sudoku_lines: Vec<String>) -> Option<Vec<Vec<u8>>> {
    let mut new_sudoku: Vec<Vec<u8>> = Vec::new();

    for line in sudoku_lines {
        if line.len() != 9 {
            return None;
        }

        let sub_vec = check_numbers(line)?;
        new_sudoku.push(sub_vec);
    }

    Some(new_sudoku)
}

pub fn parse(sudoku_lines: Vec<String>) -> Vec<Vec<u8>> {
    let msg = String::from("The provided Sudoku is not valid");
    if sudoku_lines.len() != 9 {
        panic!("{}", msg);
    }

    match check_lines(sudoku_lines) {
        Some(vec) => return vec,
        None => panic!("{}", msg),
    }
}

pub fn read(file_path: &String) -> Vec<String> {
    let mut file_handle = File::open(file_path).expect("Cannot open the file");

    let mut file_content = String::new();
    file_handle
        .read_to_string(&mut file_content)
        .expect("Cannot read the content of the file");

    file_content
        .split_terminator(' ')
        .map(|s| s.to_string())
        .collect()
}
