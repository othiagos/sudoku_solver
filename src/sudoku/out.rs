use super::solver::Solution;

pub fn print(solution: Solution) {
    println!("elapse_time: {}", solution.get_expand_nodes());

    let sudoku = match solution.get_solution() {
        Some(sudoku) => sudoku,
        None => panic!("Can not find a solution"),
    };

    println!("expanded_nodes: {}", sudoku.0);
    for line in &sudoku.1 {
        println!("{:?}", line)
    }
}
