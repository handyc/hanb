use std::io::{self, BufRead};

use crate::constants::DIAGONAL;
use crate::hanb::Navigator;

pub mod commands;
pub mod constants;
pub mod hanb;

pub fn print_level_board(navigator: &Navigator, width: u8) -> Result<String, String> {
    let level = navigator.level;
    let mut board_string = String::new();
    board_string += &format!("Level: {} = {}\n\n", level, navigator.get_path());
    match print_board(&navigator.current_board().to_string(), width) {
        Ok(board) => board_string += &board,
        Err(e) => return Err(e),
    }
    Ok(board_string)
}

pub fn print_board(board: &str, width: u8) -> Result<String, String> {
    let mut output: String = "".to_string();
    if board.is_empty() || board.len() > 64 {
        return Err(format!(
            "Input string must be 1 to 64 characters long. You entered with {}",
            board.len()
        ));
    }
    if width < 10 {
        return Err(format!("Width must be greater than {}", DIAGONAL));
    }
    let mut board = board.chars();
    let left_padding = (width - DIAGONAL) / 2;
    let right_padding = width - DIAGONAL - left_padding;
    for r in 0..DIAGONAL {
        let mut n_cells = r + 5_u8;
        if r > DIAGONAL / 2 {
            n_cells = DIAGONAL / 2 + DIAGONAL - r;
        }
        let offset: i8 = (r as i8) - (DIAGONAL as i8) / 2;
        for _ in 0..(left_padding + offset.unsigned_abs()) {
            output += "  ";
        }
        let start = (width - n_cells) / 2;
        let end = (width + n_cells) / 2;
        for c in start..end {
            let char = if let Some(cell) = board.next() {
                cell
            } else {
                // Unset cell
                '.'
            };
            if c == end - 1 {
                output += format!(" {}", char).as_str();
            } else {
                output += format!(" {}  ", char).as_str();
            }
        }
        for _ in 0..(right_padding + offset.unsigned_abs()) {
            output += "  ";
        }
        output += "\n";
    }
    Ok(output)
}

pub struct EvalContext {
    stdout: bool,
    repl: bool,
    pub history: String,
}

impl EvalContext {
    pub fn new(stdout: bool, repl: bool) -> Self {
        Self {
            stdout,
            repl,
            history: String::new(),
        }
    }
}

pub fn parse_level(line: &str) -> Result<char, String> {
    if line.is_empty() {
        return Err("Empty line".to_string());
    }
    let level = line.split('#').next().unwrap().trim();
    if level.len() > 1 {
        return Err(format!("Invalid level: {}", level));
    }
    let value = level.chars().next();
    if value.is_none() {
        return Err("Empty line".to_string());
    }
    Ok(value.unwrap())
}

/// Reads from a line string iterator and evals each line
pub fn eval_lines(lines: &mut dyn Iterator<Item = String>, context: &mut EvalContext) {
    let mut level: Result<char, String>;
    loop {
        let first = lines.next().unwrap();
        level = parse_level(&first);
        if level.is_err() {
            match level.as_ref().err().unwrap().as_str() {
                "Empty line" => continue,
                _ => {
                    if context.repl {
                        eprintln!("{}", level.err().unwrap());
                        return;
                    } else {
                        println!("Defaulting to level '.'");
                        level = Ok('.');
                    }
                }
            }
        }
        break;
    }
    let level = level.unwrap();
    context.history.push_str(format!("{}\n", level).as_str());
    let navigator = &mut Navigator::new(level).unwrap();
    for stdinline in lines {
        let line = stdinline.trim().to_owned();
        if let Err(e) = eval(navigator, line.as_str(), context) {
            eprintln!("{}", e);
        }
    }
}

pub fn eval(
    navigator: &mut Navigator,
    line: &str,
    context: &mut EvalContext,
) -> Result<(), String> {
    let stdout: bool = context.stdout;
    let repl: bool = context.repl;

    // Everything after a '#' is a comment so we can ignore it.
    let line = line.split('#').next().unwrap().trim();
    if line.is_empty() {
        return Ok(());
    }
    // Process commands
    let mut args = line.split_whitespace();
    let command = args.next().unwrap().to_lowercase();
    let args = args.collect::<Vec<&str>>().join(" ");
    for cmd in commands::COMMANDS.iter() {
        if cmd.repl_only && !repl {
            continue;
        }
        if cmd.command == command || cmd.short == command {
            let result = match cmd.command {
                "export" => {
                    if args.trim().is_empty() {
                        return Err("Missing filename".to_string());
                    }
                    (cmd.action)(
                        cmd,
                        navigator,
                        (args + " " + context.history.as_str()).as_str(),
                    )
                }
                "import" => {
                    let file = std::fs::File::open(args).expect("Unable to open file");
                    let reader = io::BufReader::new(file);
                    let mut lines = reader.lines().map(|l| l.unwrap());
                    eval_lines(&mut lines, context);
                    Err("Import sucessful!".to_string())
                }
                _ => (cmd.action)(cmd, navigator, &args),
            };
            match result {
                Ok(res) => {
                    if stdout || cmd.stdout {
                        println!("{}", res);
                    }
                    context.history.push_str(format!("{}\n", line).as_str());
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        }
    }
    Err(format!(
        "Command {} not found. Use 'help' to see available commands.",
        command
    ))
}
