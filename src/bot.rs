pub mod bot {
    use crate::board::{WIDTH, Board};

    pub fn best_move(board: &Board) -> Option<usize> {
        if let Some(col) = find_available_column(board) {
            return Some(col)
        }
        None
    }

    fn find_available_column(board: &Board) -> Option<usize> {
        for i in 0..WIDTH {
            if let Some(col) = board.first_available_row_for_column(i) {
                return Some(col)
            }
        }
        None
    }
}
