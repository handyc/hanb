mod lib;

use std::io;
use std::io::BufRead;

use clap::{self, Parser};

use rustyline::{Editor, error::ReadlineError};
use lib::{Board, Cell, Navigator};

/// Hanb is a simple language for creating model universes at any scale
/// Run without arguments to launch the repl
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

fn eval(navigator: &mut Navigator, line: &str) -> Result<(), String> {
    // Process commands
    let mut args = line.split_whitespace();
    let command = args.next().unwrap();
    let args = args.collect::<Vec<&str>>().join(" ");
    Ok(())
}

fn repl(){
    let rl = Editor::<()>::new();
    if rl.is_err() {
        eprintln!("Error creating editor");
        return;
    }
    let mut rl = rl.unwrap();
    // TODO: Add repl history
    // TODO: Add repl completion
    // TODO: Use board from cli or stdin or file
    let mut navigator: Navigator;
    loop {
        println!();
        println!("Please provide initial board:");
        let readline = rl.readline("board> ");
        if readline.is_err() {
            println!("Exiting");
            return;
        }
        let board = Board::new(&readline.unwrap());
        if let Err(e) = board {
            eprintln!("{}", e);
        } else {
            navigator = Navigator::new(board.unwrap());
            break;
        }
    }
    loop {
        let readline = rl.readline("hanb> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line);
                if let Err(e) = eval(&mut navigator, line) {
                    eprintln!("{}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
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
    if args.input.is_empty() {
        println!("Welcome to Hanb! Version {}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
        repl();
    }
    print_board(&args.input, args.width);
}
