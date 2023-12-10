use crate::board::Board;
use rand::seq::IteratorRandom;

pub struct Game {
    pub board: Board,
    pub moves: Vec<(u8, usize)>
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            moves: Vec::new()
        }
    }

    pub fn generate() -> Game {
        let mut game = Self::new();
        let mut rng = rand::thread_rng();
        let mut turn = 1;

        while !game.board.finished() {
            let cols = game.board.available_columns();
            let move_col = cols.iter().choose(&mut rng).unwrap();

            match game.board.place(*move_col, turn) {
                Ok(()) => {
                    game.moves.push((turn, *move_col));
                    turn ^= 3;
                },
                Err(err) => panic!("{:?}", err)
            }
        }

        game
    }

    pub fn print(&self) {
        self.board.print();
        println!("Winner: {:?}", Board::rune_for_piece(self.board.winner().unwrap_or(0)));
        println!("Moves made: {:?}", self.moves.len());
        println!("Moves (piece, column): {:?}", self.moves);
    }
}
