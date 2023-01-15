mod lib;

use std::io;
use std::io::BufRead;

use clap::{self, Parser};

use rustyline::{Editor, error::ReadlineError};
use lib::{Board, Cell, Navigator};

const DEFAULT_WIDTH: u8 = 12;

/// Hanb is a simple language for creating model universes at any scale
/// Run without arguments to launch the repl
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input string of 1 to 64 characters.
    #[clap(default_value_t = String::from(""))]
    input: String,

    /// Width of the board
    #[clap(default_value_t = DEFAULT_WIDTH)]
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
    match command {
        "help" | "h" => {
            println!("Commands:");
            println!("  help, h: Print this help message");
            println!("  print, p: Print the current board");
            println!("  save, sv [filename]: Save the current board to a file");
            println!("  load, ld [filename]: Load a board from a file");
            println!("  down, d [cell_number]: Resolves into cell in the current board");
            println!("  up, u: Ascends to containing board");
            println!("  define, df [cell_number] [value]: Defines the inner board of a cell");
        }
        "print" | "p" => {
            print_board(&navigator.root_board.to_string(), DEFAULT_WIDTH);
        }
        "save" | "sv" => {
            return Err("Not implemented".to_string())
        }
        "load" | "ld" => {
            return Err("Not implemented".to_string())
        }
        "down" | "d" => {
            let cell = args.parse::<usize>().map_err(|_| "Cell must be a number".to_string())?;
            if cell >= navigator.root_board.cells.len() {
                return Err(format!("Cell must be between 0 and {}", navigator.root_board.cells.len() - 1));
            }
            match navigator.descend(cell) {
                Ok(board) => {
                    println!("Resolved cell '{}'. Board color '{}' You see:", cell, board.color);
                    print_board(&board.to_string(), DEFAULT_WIDTH);
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
        "up" | "u" => {
            match navigator.ascend() {
                Some(board) => {
                    println!("You ascend to the upper board. You see:");
                    print_board(&board.to_string(), DEFAULT_WIDTH);
                }
                None => {
                    println!("{}", navigator.current_board().color);
                }
            }
        }
        "define" | "df" => {
            let mut args = args.split_whitespace();
            let cell = args.next().unwrap().parse::<usize>().map_err(|_| "Cell must be a number".to_string())?;
            if cell >= navigator.root_board.cells.len() {
                return Err(format!("Cell must be between 0 and {}", navigator.root_board.cells.len() - 1));
            }
            let value = args.collect::<Vec<&str>>().join(" ");
            if value.len() > 64 {
                return Err("Value must be 1 to 64 characters long".to_string());
            }
            match navigator.define(cell, value.as_str()) {
                Ok(_) => {
                    print_board(&navigator.current_board().to_string(), DEFAULT_WIDTH);
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
        _ => {
            return Err(format!("Unknown command: {}. Type help to see available commands.", command));
        }
    }
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

        let line = readline.unwrap();
        let board = Board::new(&line);
        rl.add_history_entry(line);
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
    } else {
        print_board(&args.input, args.width);
    }
}
