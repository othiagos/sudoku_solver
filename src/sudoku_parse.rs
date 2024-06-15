enum Number {
    One = 0x30,
    Nine = 0x39,
}

fn is_digit_in_range(number: u8) -> bool {
    number < Number::One as u8 || number > Number::Nine as u8
}

fn check_numbers<'a>(numbers: &'a &'a str) -> Option<&'a &'a str> {
    for n in numbers.bytes() {
        if is_digit_in_range(n) {
            return None;
        }
    }

    Some(numbers)
}

fn check_lines(sudoku_lines: Vec<&str>) -> Option<Vec<&str>> {
    for line in sudoku_lines.iter() {
        if line.len() != 9 {
            return None;
        }

        check_numbers(line)?;
    }

    Some(sudoku_lines)
}

pub fn parse(sudoku_lines: Vec<&str>) -> Option<Vec<&str>> {
    if sudoku_lines.len() != 9 {
        return None;
    }

    check_lines(sudoku_lines)
}
