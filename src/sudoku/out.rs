pub fn print(solution: (u128, Option<(u64, Vec<Vec<u8>>)>)) {
    let (elapse_time, sudoku_option) = solution;
    println!("elapse_time: {}", elapse_time);

    let sudoku = match sudoku_option {
        Some(sudoku) => sudoku,
        None => panic!("Can not find a solution"),
    };

    println!("expanded_nodes: {}", sudoku.0);
    for line in sudoku.1 {
        println!("{:?}", line)
    }
    println!("");
}
