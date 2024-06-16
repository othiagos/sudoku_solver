enum Number {
    ZERO = 0x30,
    NINE = 0x39,
}

fn is_digit_in_range(number: u8) -> bool {
    number >= Number::ZERO as u8 && number <= Number::NINE as u8
}

fn check_numbers<'a>(numbers: &'a &'a str) -> Option<Vec<u8>> {
    let mut new_vec: Vec<u8> = Vec::new();
    for n in numbers.chars() {
        if !is_digit_in_range(n as u8) {
            return None;
        }

        new_vec.push(n as u8 - Number::ZERO as u8);
    }

    Some(new_vec)
}

fn check_lines(sudoku_lines: Vec<&str>) -> Option<Vec<Vec<u8>>> {
    let mut new_sudoku: Vec<Vec<u8>> = Vec::new();

    for line in sudoku_lines.iter() {
        if line.len() != 9 {
            return None;
        }

        let sub_vec = check_numbers(line)?;
        new_sudoku.push(sub_vec);
    }

    Some(new_sudoku)
}

pub fn parse(sudoku_lines: Vec<&str>) -> Option<Vec<Vec<u8>>> {
    if sudoku_lines.len() != 9 {
        return None;
    }

    check_lines(sudoku_lines)
}
