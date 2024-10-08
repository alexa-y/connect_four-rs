mod board;
mod bot;
mod game;
mod server;
mod client;

use crate::board::Board;
use crate::client::Client;
use crate::game::Game;
use std::env;
use std::io::{self, BufRead};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut reader = io::stdin().lock();
        play(&mut reader)?
    } else if args[1] == "server" {
        server::listen()?;
    } else if args[1] == "address" {
        if args.len() == 2 {
            println!("Expected server address but none provided");
            return Ok(())
        }
        let address = &args[2];

        let stream = TcpStream::connect(address)?;
        Client::new(stream, 2).process()?
    } else if args[1] == "generate" {
        let game = Game::generate();
        game.print();
    } else if args[1] == "train" {
        let _ = bot::bot::train();
    } else if args[1] == "bot" {
        let mut reader = io::stdin().lock();
        play_bot(&mut reader)?
    } else {
        println!("Unknown command: {}", args[1]);
    }

    Ok(())
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

fn play_bot(reader: &mut dyn BufRead) -> io::Result<()> {
    let mut board = Board::new();
    let mut bot = bot::bot::Bot::new();
    bot.load("model.ot");

    let mut buffer = String::new();

    println!("Input a column 1-7");

    // TODO: clean this up
    while let Ok(_) = reader.read_line(&mut buffer) {
        let col = buffer.trim().parse::<usize>();

        buffer.clear();

        if col.is_err() {
            print_column_error();
            continue;
        }

        match board.place(col.unwrap() - 1, 1) {
            Ok(_) => {
                board.print();
                if let Some(winner) = board.winner() {
                    println!("{} {}", Board::rune_for_piece(winner), "wins!");
                    return Ok(())
                } else if board.full() {
                    println!("{}", "No more available slots remain. Result is a draw.");
                    return Ok(())
                }

                println!("Bot is thinking...");

                let bot_move = bot.predict(&board);
                match board.place(bot_move as usize, 2) {
                    Ok(_) => {
                        board.print();
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
