const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(PartialEq, Debug)]
pub enum BoardPlaceError {
    InvalidColumn,
    InvalidPiece
}

#[derive(Debug)]
pub struct Board {
    slots: [[u8; HEIGHT]; WIDTH]
}

impl Board {
    pub fn new() -> Board {
        Board {
            slots: [[0; HEIGHT]; WIDTH]
        }
    }

    pub fn place(&mut self, col: usize, piece: u8) -> Result<(), BoardPlaceError> {
        if piece < 1 || piece > 2 {
            return Err(BoardPlaceError::InvalidPiece)
        }

        match self.first_available_row_for_column(col) {
            Some(row) => {
                self.slots[col][row] = piece;
                Ok(())
            },
            None => Err(BoardPlaceError::InvalidColumn)
        }
    }

    pub fn winner(&self) -> Option<u8> {
        // column search
        for col in 0..WIDTH {
            let mut pos = 0;
            let mut piece = 0;

            for row in 0..HEIGHT {
                if self.slots[col][row] == 0 {
                    continue
                }

                if piece != self.slots[col][row] {
                    pos = row;
                    piece = self.slots[col][row];
                }

                if row - pos >= 3 {
                    return Some(piece)
                }
            }
        }

        // row search
        for row in 0..HEIGHT {
            let mut pos = 0;
            let mut piece = 0;
            
            for col in 0..WIDTH {
                if self.slots[col][row] == 0 {
                    continue
                }

                if piece != self.slots[col][row] {
                    pos = col;
                    piece = self.slots[col][row];
                }

                if col - pos >= 3 {
                    return Some(piece)
                }
            }
        }

        // positive diagonal search
        for col in 0..(WIDTH - 3) {
            for row in 0..(HEIGHT - 3) {
                let piece = self.slots[col][row];
                if piece == 0 {
                    continue;
                }

                if (0..=3).all(|i| self.slots[col + i][row + i] == piece) {
                    return Some(piece)
                }
            }
        }

        // negative diagonal search
        for col in (WIDTH - 4)..WIDTH {
            for row in 0..(HEIGHT - 3) {
                let piece = self.slots[col][row];
                if piece == 0 {
                    continue;
                }

                if (0..=3).all(|i| self.slots[col - i][row + i] == piece) {
                    return Some(piece)
                }
            }
        }

        None
    }

    pub fn full(&self) -> bool {
        for col in 0..WIDTH {
            for row in 0..HEIGHT {
                if self.slots[col][row] == 0 {
                    return false
                }
            }
        }
        true
    }

    pub fn print(&self) {
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                print!("{}", Board::rune_for_piece(self.slots[col][row]));
            }
            println!("")
        }
    }

    pub fn rune_for_piece(piece: u8) -> char {
        if piece == 1 {
            'X'
        } else if piece == 2 {
            'O'
        } else {
            '.'
        }
    }

    fn first_available_row_for_column(&self, col: usize) -> Option<usize> {
        if col >= WIDTH {
            return None
        }

        for row in 0..HEIGHT {
            if self.slots[col][row] == 0 {
                return Some(row)
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_board() -> Board {
        Board::new()
    }

    #[test]
    fn test_board_creation() {
        let board = setup_board();
        assert_eq!(board.slots, [[0; HEIGHT]; WIDTH]);
    }

    #[test]
    fn test_returns_none_for_out_of_bound_column() {
        let board = setup_board();
        assert!(board.first_available_row_for_column(WIDTH + 1).is_none());
    }

    #[test]
    fn test_returns_none_for_full_column() {
        let mut board = setup_board();
        for _ in 0..HEIGHT {
            assert!(board.place(0, 1).is_ok());
        }
        assert!(board.first_available_row_for_column(0).is_none());
    }

    #[test]
    fn test_returns_some_for_good_column() {
        let mut board = setup_board();
        assert_eq!(board.first_available_row_for_column(0).unwrap(), 0);
        assert!(board.place(0, 1).is_ok());
        assert_eq!(board.first_available_row_for_column(0).unwrap(), 1);
    }

    #[test]
    fn test_returns_err_for_invalid_piece_placement() {
        let mut board = setup_board();
        assert_eq!(board.place(0, 0), Err(BoardPlaceError::InvalidPiece));
        assert_eq!(board.place(0, 3), Err(BoardPlaceError::InvalidPiece));
    }

    #[test]
    fn test_returns_winner_with_simple_column() {
        let mut board = setup_board();
        for _ in 0..=3 {
            assert!(board.winner().is_none());
            let _ = board.place(0, 1);
        }
        assert_eq!(board.winner().unwrap(), 1);
    }

    #[test]
    fn test_returns_winner_with_interspersed_column() {
        let mut board = setup_board();
        let _ = board.place(0, 1);
        let _ = board.place(0, 2);
        let _ = board.place(1, 2);
        for _ in 0..=3 {
            assert!(board.winner().is_none());
            let _ = board.place(1, 1);
        }
        assert_eq!(board.winner().unwrap(), 1);
    }
    
    #[test]
    fn test_returns_winner_with_simple_row() {
        let mut board = setup_board();
        for i in 0..=3 {
            assert!(board.winner().is_none());
            let _ = board.place(i, 1);
        }
        assert_eq!(board.winner().unwrap(), 1);
    }

    #[test]
    fn test_returns_winner_with_interspersed_row() {
        let mut board = setup_board();
        let _ = board.place(0, 1);
        let _ = board.place(1, 2);
        let _ = board.place(2, 2);
        let _ = board.place(3, 2);
        let _ = board.place(4, 1);
        let _ = board.place(0, 2);
        for i in 1..=4 {
            assert!(board.winner().is_none());
            let _ = board.place(i, 1);
        }
        assert_eq!(board.winner().unwrap(), 1);
    }

    #[test]
    fn returns_winner_with_positive_diagonal() {
        let mut board = setup_board();
        for i in 1..4 {
            for _ in 0..i {
                let _ = board.place(i, 2);
            }
        }

        for i in 0..4 {
            assert!(board.winner().is_none());
            let _ = board.place(i, 1);
        }
        assert_eq!(board.winner().unwrap(), 1);
    }

    #[test]
    fn returns_winner_with_negative_diagonal() {
        let mut board = setup_board();
        for i in 1..4 {
            for _ in 0..i {
                let _ = board.place(WIDTH - i - 1, 2);
            }
        }

        for i in 1..=4 {
            assert!(board.winner().is_none());
            let _ = board.place(WIDTH - i, 1);
        }
        assert_eq!(board.winner().unwrap(), 1);
    }

    #[test]
    fn test_full_returns_true_when_full() {
        let mut board = setup_board();
        for col in 0..WIDTH {
            for _ in 0..HEIGHT {
                assert!(!board.full());
                let _ = board.place(col, 1);
            }
        }
        assert!(board.full());
    }
}