mod lib;

use std::io;
use std::io::BufRead;

use clap::{self, Parser};

use lib::{eval, print_board, Board, Navigator, DEFAULT_WIDTH};
use rustyline::{error::ReadlineError, Editor};

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

fn repl() {
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
                if let Err(e) = eval(&mut navigator, line, true) {
                    eprintln!("{}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("exiting...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

/// Reads from a line string iterator and evals each line
fn eval_lines(lines: &mut dyn Iterator<Item = String>) {
    let navigator = &mut Navigator::new(Board::new(lines.next().unwrap().as_str()).unwrap());
    for stdinline in lines {
        let line = stdinline.trim().to_owned();
        if let Err(e) = eval(navigator, line.as_str(), false) {
            eprintln!("{}", e);
        }
    }
}

fn main() {
    let args = Args::parse();
    if args.stdin {
        println!("Waiting for board...");
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());
        eval_lines(&mut lines);
        return;
    }
    if args.input.is_empty() {
        println!(
            "Welcome to Hanb! Version {}",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );
        repl();
    } else {
        print_board(&args.input, args.width);
    }
}
