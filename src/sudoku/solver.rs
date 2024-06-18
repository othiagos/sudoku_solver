use crate::sudoku::algorithms;
use std::{collections::HashSet, time::Instant};

pub struct Solution {
    expand_nodes: u128,
    solution: Option<(u64, Vec<Vec<u8>>)>,
}

impl Solution {
    fn new(expand_nodes: u128, solution: Option<(u64, Vec<Vec<u8>>)>) -> Self {
        Self {
            expand_nodes,
            solution,
        }
    }

    pub fn get_expand_nodes(&self) -> u128 {
        self.expand_nodes
    }

    pub fn get_solution(&self) -> &Option<(u64, Vec<Vec<u8>>)> {
        &self.solution
    }
}

pub fn get_available_moves(sudoku: &[Vec<u8>], pos: (usize, usize)) -> Vec<u8> {
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
    
    for vec in sudoku.iter().skip(x).take(3) {
        for value in vec.iter().skip(y).take(3) {
            block[m] = *value;
            m += 1;
        }
    }
    set.retain(|k| !block.contains(k));

    set.into_iter().collect()
}

pub fn solve(alg: char, sudoku: Vec<Vec<u8>>) -> Solution {
    let start = Instant::now();
    let sudoku_option = match alg {
        'U' => algorithms::uniform_cost_search::solve(sudoku),
        _ => panic!("Invalid option"),
    };

    Solution::new(start.elapsed().as_millis(), sudoku_option)
}
