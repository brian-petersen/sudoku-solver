use crate::board::SudokuBoard;

use super::Solver;

pub struct Backtracking {
    // TODO remove pub
    pub board: SudokuBoard,
    history: Vec<(u8, u8)>,
}

impl Backtracking {
    pub fn new(board: SudokuBoard) -> Self {
        Self { board, history: vec![] }
    }

    fn get_next(&mut self, backtrack_needed: bool) -> ((u8, u8), u8) {
        if backtrack_needed {
            let Some(current) = self.history.pop() else {
                // uh... don't know
                todo!()
            };
            
            // TODO need to rethink the backtracking logic here (and come up with a test case for it)

            (current, self.board.get_value(current.0, current.1) + 1)
        } else {
            let previous = match self.history.last() {
                Some(previous) => *previous,
                None => (0, 0),
            };

            let Some(next) = self.find_next_empty(previous.0, previous.1) else {
                // what should we do if no empty is found?
                todo!()
            };

            (next, 1)
        }
    }

    fn find_next_empty(&self, start_row: u8, start_col: u8) -> Option<(u8, u8)> {
        for row in start_row..9 {
            // Start from indicated col in start row, otherwise start beginning of following rows
            let real_start_col = if row == start_row { start_col } else { 0 };

            for col in real_start_col..9 {
                if self.board.get_value(row, col) == 0 {
                    return Some((row, col));
                }
            }
        }

        None
    }
}

impl Solver for Backtracking {
    fn solve(&mut self) {
        if let Err(_) = self.board.is_legal() {
            // what should we do if given board that can't be solved?
            todo!()
        }

        let mut backtrack_needed = false;
        loop {
            let (current, start) = self.get_next(backtrack_needed);

            backtrack_needed = false;
            for i in start..10 {
                self.board.set_value(current.0, current.1, i);

                if self.board.is_legal() == Ok(()) {
                    self.history.push(current);
                    break;
                }
                else if i == 9 {
                    backtrack_needed = true;
                    self.board.set_value(current.0, current.1, 0);
                }
            }

            if self.board.is_complete() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_row(board: &mut SudokuBoard, row: u8, value: u8) {
        for col in 0..9 {
            board.set_value(row, col, value);
        }
    }

    #[test]
    fn test_find_next_empty() {
        let board = SudokuBoard::new();
        let mut solver = Backtracking::new(board);

        assert_eq!(solver.find_next_empty(0, 0), Some((0, 0)));

        set_row(&mut solver.board, 1, 1);
        assert_eq!(solver.find_next_empty(0, 0), Some((0, 0)));
        assert_eq!(solver.find_next_empty(1, 0), Some((2, 0)));
        assert_eq!(solver.find_next_empty(1, 1), Some((2, 0)));
        assert_eq!(solver.find_next_empty(1, 5), Some((2, 0)));

        assert_eq!(solver.find_next_empty(2, 3), Some((2, 3)));

        set_row(&mut solver.board, 0, 1);
        assert_eq!(solver.find_next_empty(0, 0), Some((2, 0)));

        for row in 0..9 {
            set_row(&mut solver.board, row, 1);
        }
        assert_eq!(solver.find_next_empty(0, 0), None);
    }

    #[test]
    fn test_1() {
        let grid = [
            [0, 2, 7, 1, 5, 4, 3, 9, 6],
            [9, 6, 5, 3, 2, 7, 1, 4, 8],
            [3, 4, 1, 6, 8, 9, 7, 5, 2],
            [5, 9, 3, 4, 6, 8, 2, 7, 1],
            [4, 7, 2, 5, 1, 3, 6, 8, 9],
            [6, 1, 8, 9, 7, 2, 4, 3, 5],
            [7, 8, 6, 2, 3, 5, 9, 1, 4],
            [1, 5, 4, 7, 9, 6, 8, 2, 3],
            [2, 3, 9, 8, 4, 1, 5, 6, 7],
        ];
        let board = SudokuBoard::new_with_grid(grid);
        let mut solver = Backtracking::new(board);
        solver.solve();

        println!("{}", solver.board.as_ascii());

        assert!(false);
    }
}
