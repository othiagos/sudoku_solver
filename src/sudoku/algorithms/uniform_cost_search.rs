use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::sudoku::solver::get_available_moves;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UniformCostSearch {
    lines: Vec<Vec<u8>>,
    zero_count: u8,
    curr_pos: (usize, usize),
}

impl UniformCostSearch {
    fn new(sudoku: Vec<Vec<u8>>) -> Self {
        let mut n = 0;

        for i in 0..sudoku.len() {
            for j in 0..sudoku.len() {
                if sudoku[i][j] == 0 {
                    n += 1;
                }
            }
        }

        Self {
            lines: sudoku,
            zero_count: n,
            curr_pos: (0, 0),
        }
    }

    fn add_number(&mut self, number: u8) {
        let (i, j) = self.curr_pos;
        self.lines[i][j] = number;
        self.zero_count -= 1;
    }

    fn get_curr_pos(&mut self) -> Option<(usize, usize)> {
        let (start_row, start_col) = self.curr_pos;
        for i in 0..self.lines.len() {
            let mut start_col_range = start_col;
            if i != start_row {
                start_col_range = 0
            }

            for j in start_col_range..self.lines.len() {
                if self.lines[i][j] == 0 {
                    self.curr_pos = (i, j);
                    return Some(self.curr_pos);
                }
            }
        }
        None
    }

    fn solve(&self) -> Option<(u64, Vec<Vec<u8>>)> {
        let mut heap: BinaryHeap<Reverse<UniformCostSearch>> = BinaryHeap::new();
        heap.push(Reverse(UniformCostSearch::new(self.lines.clone())));

        let mut expanded_nodes: u64 = 0;
        while !heap.is_empty() {
            let mut curr = heap.pop().unwrap().0;

            let curr_option = curr.get_curr_pos();

            let pos = match curr_option {
                Some(pos) => pos,
                None => return Some((expanded_nodes, curr.lines)),
            };

            for n in get_available_moves(&curr.lines, pos) {
                expanded_nodes += 1;

                let mut next_sudoku = curr.clone();
                next_sudoku.add_number(n);

                if next_sudoku.zero_count == 0 {
                    return Some((expanded_nodes, next_sudoku.lines));
                }

                heap.push(Reverse(next_sudoku));
            }
        }
        None
    }
}

pub fn solve(sudoku: Vec<Vec<u8>>) -> Option<(u64, Vec<Vec<u8>>)> {
    UniformCostSearch::new(sudoku).solve()
}
