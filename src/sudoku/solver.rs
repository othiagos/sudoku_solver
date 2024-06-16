use crate::sudoku::algorithms;
use std::{collections::HashSet, time::Instant};

pub fn get_available_moves(sudoku: &Vec<Vec<u8>>, pos: (usize, usize)) -> Option<Vec<u8>> {
    let (i, j) = pos;

    if sudoku[i][j] != 0 {
        return None;
    }

    let mut set: HashSet<u8> = (1..10).collect();

    let line = &sudoku[i];
    set.retain(|&k| !line.contains(&k));

    let mut column = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for k in 0..sudoku.len() {
        column[k] = sudoku[k][j];
    }
    set.retain(|&k| !column.contains(&k));

    let x = i / 3 * 3;
    let y = j / 3 * 3;

    let mut m = 0;
    let mut block = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in x..x + 3 {
        for j in y..y + 3 {
            block[m] = sudoku[i][j];
            m += 1;
        }
    }
    set.retain(|&k| !block.contains(&k));

    Some(set.into_iter().collect())
}

pub fn solve(_alg: char, sudoku: Vec<Vec<u8>>) -> (u128, Option<(u64, Vec<Vec<u8>>)>) {
    let start = Instant::now();
    let sudoku_option = algorithms::uniform_cost_search::solve(sudoku);

    (start.elapsed().as_millis(), sudoku_option)
}
