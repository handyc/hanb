use std::io;
use std::io::BufRead;

use clap::{self, Parser};

const  CHARACTER_MAP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.";

/// Hanb is a simple language for creating model universes at any scale
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input string of 1 to 64 characters.
    #[clap(default_value_t = String::from(""))]
    input: String,

    /// Width of the board
    #[clap(default_value_t = 12)]
    width: u8,

    /// Read from stdin instead of using the input argument.
    #[clap(short, long)]
    stdin: bool,
}

fn print_board(board: &str, width: u8) {
    if board.is_empty() || board.len() > 64 {
        eprintln!("Input string must be 1 to 64 characters long. You entered with {}", board.len());
        return;
    }
    if width < 10 {
        eprintln!("Width must be greater than {}", 9);
        return;
    }
    let mut board = board.chars();
    for c in 0..9 {
        let mut n_cells = c + 5;
        if c > 4 {
            n_cells = 13 - c;
        }
        for r in 0..width {
            if r < (width - n_cells) / 2 || r >= (width + n_cells) / 2 {
                print!("▗");
            } else {
                if let Some(cell) = board.next() {
                    print!("{}", cell);
                } else {
                    print!("▗");
                }
            }
        }
        println!();
    }
}

fn main() {
    let args = Args::parse();
    if args.stdin {
        println!("Reading from stdin...");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            print_board(&line.unwrap(), args.width);
        }
    }
    print_board(&args.input, args.width);
}
