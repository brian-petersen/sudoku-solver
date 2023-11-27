use std::fmt::Write;

#[derive(Debug)]
pub struct SudokuBoard {
    grid: [[u8; 9]; 9],
}

impl SudokuBoard {
    pub fn new() -> Self {
        Self { grid: [[0; 9]; 9] }
    }

    pub fn new_with_grid(grid: [[u8; 9]; 9]) -> Self {
        Self { grid }
    }

    pub fn as_ascii(&self) -> String {
        let mut board = String::new();

        for (i, row) in self.grid.iter().enumerate() {
            if i == 3 || i == 6 {
                writeln!(&mut board, "---------------------").unwrap();
            }

            for (j, &num) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(&mut board, "| ").unwrap();
                }
                if num == 0 {
                    write!(&mut board, ". ").unwrap();
                } else {
                    write!(&mut board, "{} ", num).unwrap();
                }
            }

            writeln!(&mut board).unwrap();
        }

        board
    }

    pub fn get_value(&self, row: u8, col: u8) -> u8 {
        debug_assert!(row <= 8 && col <= 8);

        self.grid[row as usize][col as usize]
    }

    pub fn is_complete(&self) -> bool {
        self.do_is_legal(false) == Ok(())
    }

    pub fn is_legal(&self) -> Result<(), InvalidBoard> {
        self.do_is_legal(true)
    }

    pub fn set_value(&mut self, row: u8, col: u8, value: u8) {
        debug_assert!(row <= 8 && col <= 8 && value <= 9);

        self.grid[row as usize][col as usize] = value;
    }

    fn do_is_legal(&self, skip_empty_check: bool) -> Result<(), InvalidBoard> {
        // Check rows
        for (i, row) in self.grid.iter().enumerate() {
            if let Err(reason) = Self::is_unit_legal(row, skip_empty_check) {
                return Err(InvalidBoard::Row(i as u8, reason));
            }
        }

        // Check columns
        for col in 0..9 {
            let column = (0..9).map(|row| self.grid[row][col]).collect::<Vec<_>>();
            if let Err(reason) = Self::is_unit_legal(&column, skip_empty_check) {
                return Err(InvalidBoard::Column(col as u8, reason));
            }
        }

        // Check 3x3 sections
        for section in 0..9 {
            let (start_row, start_col) = match section {
                0 => (0, 0),
                1 => (0, 3),
                2 => (0, 6),
                3 => (3, 0),
                4 => (3, 3),
                5 => (3, 6),
                6 => (6, 0),
                7 => (6, 3),
                8 => (6, 6),
                _ => panic!("Invalid section"),
            };

            let subgrid = (0..3)
                .flat_map(|i| (0..3).map(move |j| self.grid[start_row + i][start_col + j]))
                .collect::<Vec<_>>();

            if let Err(reason) = Self::is_unit_legal(&subgrid, skip_empty_check) {
                return Err(InvalidBoard::Section(section as u8, reason));
            }
        }

        Ok(())
    }

    fn is_unit_legal(unit: &[u8], skip_empty_check: bool) -> Result<(), InvalidUnit> {
        debug_assert!(unit.len() == 9);

        // 0 is not used, so we use indices 1 through 9.
        let mut seen = [false; 10];

        for &num in unit {
            if !skip_empty_check && num == 0 {
                return Err(InvalidUnit::Empty);
            }

            if seen[num as usize] && num != 0 {
                return Err(InvalidUnit::Duplicate(num));
            }

            seen[num as usize] = true;
        }

        Ok(())
    }
}

impl Default for SudokuBoard {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum InvalidBoard {
    Row(u8, InvalidUnit),
    Column(u8, InvalidUnit),
    Section(u8, InvalidUnit),
}

#[derive(Debug, PartialEq)]
pub enum InvalidUnit {
    /// Contains dupliate value.
    Duplicate(u8),

    /// Contains an empty value.
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum SetError {
    Row,
    Col,
    Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_ascii() {
        let mut board = SudokuBoard::new();
        board.set_value(0, 0, 1);
        board.set_value(0, 8, 8);
        board.set_value(4, 4, 4);
        board.set_value(1, 1, 7);
        board.set_value(2, 2, 3);
        board.set_value(8, 8, 6);

        let ascii = board.as_ascii();

        let expected = "1 . . | . . . | . . 8 
. 7 . | . . . | . . . 
. . 3 | . . . | . . . 
---------------------
. . . | . . . | . . . 
. . . | . 4 . | . . . 
. . . | . . . | . . . 
---------------------
. . . | . . . | . . . 
. . . | . . . | . . . 
. . . | . . . | . . 6 
";

        assert_eq!(ascii, expected);
    }

    #[test]
    #[should_panic]
    fn test_is_unit_legal_throws_if_too_few() {
        let unit = vec![0; 8];
        let _ = SudokuBoard::is_unit_legal(&unit, false);
    }

    #[test]
    #[should_panic]
    fn test_is_unit_legal_throws_if_too_many() {
        let unit = vec![0; 10];
        let _ = SudokuBoard::is_unit_legal(&unit, false);
    }

    #[test]
    fn test_is_unit_legal_all_empty() {
        let unit = vec![0; 9];
        assert_eq!(
            SudokuBoard::is_unit_legal(&unit, false),
            Err(InvalidUnit::Empty)
        );
    }

    #[test]
    fn test_is_unit_legal_all_empty_skip() {
        let unit = vec![0; 9];
        assert_eq!(SudokuBoard::is_unit_legal(&unit, true), Ok(()));
    }

    #[test]
    fn test_is_unit_legal_all_seen_once() {
        let unit = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(SudokuBoard::is_unit_legal(&unit, false), Ok(()));
    }

    #[test]
    fn test_is_unit_legal_all_seen_once_reversed() {
        let unit = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(SudokuBoard::is_unit_legal(&unit, false), Ok(()));
    }

    #[test]
    fn test_is_unit_legal_all_one_duplicate() {
        let unit = vec![2, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            SudokuBoard::is_unit_legal(&unit, false),
            Err(InvalidUnit::Duplicate(2))
        );
    }

    #[test]
    fn test_is_unit_legal_missing_one() {
        let unit = vec![0, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            SudokuBoard::is_unit_legal(&unit, false),
            Err(InvalidUnit::Empty)
        );
    }

    #[test]
    fn test_is_unit_legal_missing_one_skip() {
        let unit = vec![0, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(SudokuBoard::is_unit_legal(&unit, true), Ok(()));
    }

    #[test]
    fn test_is_legal_row_duplicate() {
        let grid = [
            [2, 2, 3, 4, 5, 6, 7, 8, 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
        ];

        let board = SudokuBoard { grid };

        assert_eq!(
            board.is_legal(),
            Err(InvalidBoard::Row(0, InvalidUnit::Duplicate(2)))
        );
    }

    #[test]
    fn test_is_legal_row_duplicate_second() {
        let grid = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 3, 3, 4, 5, 6, 7, 8, 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
        ];

        let board = SudokuBoard { grid };

        assert_eq!(
            board.is_legal(),
            Err(InvalidBoard::Row(1, InvalidUnit::Duplicate(3)))
        );
    }

    #[test]
    fn test_is_legal_row_empty() {
        let grid = [
            [0; 9], [0; 9], [0; 9], [0; 9], [0; 9], [0; 9], [0; 9], [0; 9], [0; 9],
        ];

        let board = SudokuBoard { grid };

        assert_eq!(board.is_legal(), Ok(()));
        assert!(!board.is_complete());
    }

    #[test]
    fn test_is_legal_row_empty_second() {
        let grid = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
            [0; 9],
        ];

        let board = SudokuBoard { grid };

        assert_eq!(board.is_legal(), Ok(()));
        assert!(!board.is_complete());
    }

    #[test]
    fn test_is_legal_valid_board() {
        let grid = [
            [8, 2, 7, 1, 5, 4, 3, 9, 6],
            [9, 6, 5, 3, 2, 7, 1, 4, 8],
            [3, 4, 1, 6, 8, 9, 7, 5, 2],
            [5, 9, 3, 4, 6, 8, 2, 7, 1],
            [4, 7, 2, 5, 1, 3, 6, 8, 9],
            [6, 1, 8, 9, 7, 2, 4, 3, 5],
            [7, 8, 6, 2, 3, 5, 9, 1, 4],
            [1, 5, 4, 7, 9, 6, 8, 2, 3],
            [2, 3, 9, 8, 4, 1, 5, 6, 7],
        ];

        let board = SudokuBoard { grid };

        assert!(board.is_complete());
    }

    #[test]
    fn test_is_legal_valid_board_missing() {
        let grid = [
            [0, 2, 7, 1, 5, 4, 3, 9, 6],
            [9, 6, 5, 3, 2, 7, 1, 4, 8],
            [3, 4, 1, 6, 8, 9, 0, 5, 2],
            [5, 9, 3, 4, 6, 8, 2, 7, 1],
            [4, 7, 2, 5, 1, 3, 6, 8, 9],
            [6, 1, 8, 9, 7, 2, 4, 3, 5],
            [7, 8, 6, 2, 3, 5, 9, 1, 4],
            [1, 5, 4, 7, 0, 6, 8, 2, 3],
            [2, 3, 9, 8, 4, 1, 5, 6, 7],
        ];

        let board = SudokuBoard { grid };

        assert!(!board.is_complete());
        assert_eq!(board.is_legal(), Ok(()));
    }

    #[test]
    fn test_set_move_valid() {
        let mut board = SudokuBoard::new();
        board.set_value(0, 0, 0);
        board.set_value(8, 0, 0);
        board.set_value(0, 8, 0);
        board.set_value(0, 0, 9);
    }

    #[test]
    #[should_panic]
    fn test_set_move_invalid_row() {
        let mut board = SudokuBoard::new();
        board.set_value(9, 0, 0);
    }

    #[test]
    #[should_panic]
    fn test_set_move_invalid_col() {
        let mut board = SudokuBoard::new();
        board.set_value(0, 9, 0);
    }

    #[test]
    #[should_panic]
    fn test_set_move_invalid_value() {
        let mut board = SudokuBoard::new();
        board.set_value(0, 0, 10);
    }
}
