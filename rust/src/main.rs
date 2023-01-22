use std::io;
use std::io::BufRead;

use clap::{self, Parser};

use hanb::{constants::DEFAULT_WIDTH, hanb::Navigator, eval, print_board, commands::COMMANDS};
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

    /// Max verbosity
    #[clap(short, long)]
    verbose: bool,

    /// List query language commands and exit
    #[clap(short, long)]
    commands: bool,

    /// Execute hanb script from file
    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.commands {
        for cmd in COMMANDS {
            if cmd.repl_only {
                continue;
            }
            println!("{}", cmd);
        }
        return;
    }
    if args.file.is_some() {
        let filename = args.file.unwrap();
        let file = std::fs::File::open(filename).expect("Unable to open file");
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines().map(|l| l.unwrap());
        eval_lines(&mut lines, args.verbose);
        return;
    }
    if args.stdin {
        println!("Waiting for board...");
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());
        eval_lines(&mut lines, args.verbose);
        return;
    }
    if args.input.is_empty() {
        println!(
            "Welcome to Hanb! Version {}",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );
        repl();
    } else {
        let pb = print_board(&args.input, args.width);
        match pb {
            Ok(board) => println!("{}", board),
            Err(e) => eprintln!("{}", e),
        }
    }
}


fn parse_level(line: &str) -> Result<char, String> {
    if line.is_empty() {
        return Err("Empty line".to_string());
    }
    let level = line.split("#").next().unwrap().trim();
    if level.len() > 1 {
        return Err(format!("Invalid level: {}", level));
    }
    let value = level.chars().next();
    if value.is_none() {
        return Err("Empty line".to_string());
    }
    Ok(value.unwrap())
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
        println!("Please provide start level");
        let readline = rl.readline("level> ");
        if readline.is_err() {
            println!("Exiting");
            return;
        }

        let line = readline.unwrap();
        if line.is_empty() {
            continue
        }
        let level = parse_level(&line);
        if level.is_err() {
            println!("{}", level.err().unwrap());
            continue;
        }
        rl.add_history_entry(line);
        match Navigator::new(level.unwrap()) {
            Ok(nav) => {
                navigator = nav;
                break
            },
            Err(e) => println!("{}", e),
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
fn eval_lines(lines: &mut dyn Iterator<Item = String>, stdout: bool) {
    let mut level: Result<char, String>;
    loop {
        let first = lines.next().unwrap();
        level = parse_level(&first);
        if level.is_err() {
            match level.as_ref().err().unwrap().as_str() {
                "Empty line" => continue,
                _ => {
                    eprintln!("{}", level.err().unwrap());
                    return;
                }
            }
        }
        break;
    }
    let navigator = &mut Navigator::new(level.unwrap()).unwrap();
    for stdinline in lines {
        let line = stdinline.trim().to_owned();
        if let Err(e) = eval(navigator, line.as_str(), stdout) {
            eprintln!("{}", e);
        }
    }
}

