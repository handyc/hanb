use std::io;
use std::io::BufRead;

use clap::{self, Parser};

use hanb::{
    commands::COMMANDS, constants::DEFAULT_WIDTH, eval, eval_lines, hanb::Navigator, parse_level,
    print_board, EvalContext,
};
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
        let mut context = EvalContext::new(false, false);
        eval_lines(&mut lines, &mut context);
        return;
    }
    if args.stdin {
        println!("Waiting for board...");
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());
        let mut context = EvalContext::new(args.verbose, false);
        eval_lines(&mut lines, &mut context);
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

fn repl() {
    let rl = Editor::<()>::new();
    if rl.is_err() {
        eprintln!("Error creating editor");
        return;
    }
    let mut rl = rl.unwrap();
    // TODO: Add repl history
    // TODO: Add repl completion
    let mut navigator: Navigator;
    let mut context = EvalContext::new(true, true);
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
            continue;
        }
        let level = parse_level(&line);
        if level.is_err() {
            println!("{}", level.err().unwrap());
            continue;
        }
        let level = level.unwrap();
        rl.add_history_entry(line);
        match Navigator::new(level) {
            Ok(nav) => {
                navigator = nav;
                context.history.push_str(format!("{}\n", level).as_str());
                break;
            }
            Err(e) => println!("{}", e),
        }
    }
    loop {
        let readline = rl.readline("hanb> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line);
                if let Err(e) = eval(&mut navigator, line, &mut context) {
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
