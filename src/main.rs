mod board;

use crate::board::Board;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut reader = io::stdin().lock();
    play(&mut reader)
}

fn play(reader: &mut dyn BufRead) -> io::Result<()> {
    let mut board = Board::new();
    let mut piece: u8 = 1;
    let mut buffer = String::new();

    println!("Input a column 1-7");

    while let Ok(_) = reader.read_line(&mut buffer) {
        let col = buffer.trim().parse::<usize>();

        buffer.clear();

        if col.is_err() {
            print_column_error();
            continue;
        }

        match board.place(col.unwrap() - 1, piece) {
            Ok(_) => {
                board.print();
                piece ^= 3;
                if let Some(winner) = board.winner() {
                    println!("{} {}", Board::rune_for_piece(winner), "wins!");
                    return Ok(())
                } else if board.full() {
                    println!("{}", "No more available slots remain. Result is a draw.");
                    return Ok(())
                }
            },
            Err(_) => {
                print_column_error();
            }
        }
    }
    Ok(())
}

fn print_column_error() {
    println!("{}", "Not a valid column number. Select 1-7.");
}
