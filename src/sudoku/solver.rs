use crate::sudoku::algorithms;
use std::{collections::HashSet, time::Instant};

pub fn get_available_moves(sudoku: &Vec<Vec<u8>>, pos: (usize, usize)) -> Vec<u8> {
    let (i, j) = pos;

    if sudoku[i][j] != 0 {
        return Vec::new();
    }

    let mut set: HashSet<u8> = (1..10).collect();

    let line = &sudoku[i];
    set.retain(|k| !line.contains(k));

    let mut column = vec![0; sudoku.len()];
    for k in 0..sudoku.len() {
        column[k] = sudoku[k][j];
    }
    set.retain(|k| !column.contains(k));

    let x = i / 3 * 3;
    let y = j / 3 * 3;

    let mut m = 0;
    let mut block = vec![0; sudoku.len()];
    for i in x..x + 3 {
        for j in y..y + 3 {
            block[m] = sudoku[i][j];
            m += 1;
        }
    }
    set.retain(|k| !block.contains(k));

    set.into_iter().collect()
}

pub fn solve(alg: char, sudoku: Vec<Vec<u8>>) -> (u128, Option<(u64, Vec<Vec<u8>>)>) {
    let start = Instant::now();
    let sudoku_option = match alg {
        'U' => algorithms::uniform_cost_search::solve(sudoku),
        _ => panic!("Invalid option"),
    };

    (start.elapsed().as_millis(), sudoku_option)
}
