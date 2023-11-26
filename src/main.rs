use sudoku_solver::board;

fn main() {
    let mut board = board::SudokuBoard::new();

    println!("I made an empty board for you!");
    println!();
    println!("{}", board.as_ascii());

    println!("Let me fill in some squares for you...");
    for i in 0..9 {
        board.set_value(i, i, (i + 1) as u8).unwrap();
    }
    println!();
    println!("{}", board.as_ascii());

    println!("All done :)")
}
