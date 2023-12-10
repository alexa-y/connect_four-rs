use std::io::{self, Read, BufRead, BufReader, Write};
use std::net::TcpStream;
use crate::board::Board;

pub struct Client {
    stream: TcpStream,
    board: Board,
    piece: u8,
    turn: u8
}

impl Client {
    pub fn new(t: TcpStream, piece: u8) -> Client {
        Self {
            stream: t,
            board: Board::new(),
            piece: piece,
            turn: 1
        }
    }

    pub fn process(&mut self) -> io::Result<()> {
        loop {
            self.stream.take_error().expect("Error communicating with remote");

            let column: usize;

            if self.piece == self.turn {
                column = self.read_from_stdin();
            } else {
                column = self.read_from_remote()?;
            }

            match self.board.place(column - 1, self.turn) {
                Ok(_) => {
                    if self.piece == self.turn {
                        self.send_move_to_remote(column)?;
                    }
                    self.board.print();
                    self.turn ^= 3;
                    if let Some(winner) = self.board.winner() {
                        println!("{} {}", Board::rune_for_piece(winner), "wins!");
                        return Ok(())
                    } else if self.board.full() {
                        println!("{}", "No more available slots remain. Result is a draw.");
                        return Ok(())
                    }
                },
                Err(_) => {
                    Self::print_column_error();
                }
            }


            if let Some(winner) = self.board.winner() {
                println!("{} {}", Board::rune_for_piece(winner), "wins!");
                return Ok(())
            } else if self.board.full() {
                println!("{}", "No more available slots remain. Result is a draw.");
                return Ok(())
            }
        }
    }

    fn read_from_stdin(&self) -> usize {
        println!("Your turn! Input a column 1-7");
        let mut buffer = String::new();
        let mut reader = BufReader::new(io::stdin().lock());

        while let Ok(_) = reader.read_line(&mut buffer) {
            let col = buffer.trim().parse::<usize>();

            buffer.clear();

            match col {
                Ok(column) => return column,
                Err(_) => Self::print_column_error()
            }
        }

        0
    }

    fn read_from_remote(&mut self) -> io::Result<usize> {
        println!("It is your opponent's turn, waiting for them to make a move.");
        let mut buf = [0; 1];

        self.stream.read_exact(&mut buf)?;

        Ok(buf[0usize] as usize)
    }

    fn send_move_to_remote(&mut self, column: usize) -> io::Result<usize> {
        self.stream.write(&[column as u8])
    }

    fn print_column_error() {
        println!("{}", "Not a valid column number. Select 1-7.");
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.stream.shutdown(std::net::Shutdown::Both);
    }
}
