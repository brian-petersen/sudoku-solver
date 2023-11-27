use sudoku_solver::{
    board,
    solver::{self, Solver},
};

fn main() {
    // let mut board = board::SudokuBoard::new();
    //
    // println!("I made an empty board for you!");
    // println!();
    // println!("{}", board.as_ascii());
    //
    // println!("Let me fill in some squares for you...");
    // for i in 0..9 {
    //     board.set_value(i, i, i + 1);
    // }
    // println!();
    // println!("{}", board.as_ascii());
    //
    // println!("All done :)")

    // let grid = [
    //     [0, 2, 7, 1, 5, 4, 3, 9, 6],
    //     [9, 6, 5, 3, 2, 7, 1, 4, 8],
    //     [3, 4, 1, 6, 8, 9, 7, 5, 2],
    //     [5, 9, 3, 4, 6, 8, 2, 7, 1],
    //     [4, 7, 2, 5, 1, 3, 6, 8, 9],
    //     [6, 1, 8, 9, 7, 2, 4, 3, 5],
    //     [7, 8, 6, 2, 3, 5, 9, 1, 4],
    //     [1, 5, 4, 7, 9, 6, 8, 2, 3],
    //     [2, 3, 9, 8, 4, 1, 5, 6, 7],
    // ];
    let grid = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let board = board::SudokuBoard::new_with_grid(grid);
    let mut solver = solver::backtracking::Backtracking::new(board);
    solver.solve();

    println!("{}", solver.board.as_ascii());

    assert!(false);
}
