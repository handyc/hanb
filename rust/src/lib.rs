use constants::{DIAGONAL, DEFAULT_WIDTH};
use hanb::Navigator;

pub mod commands;
pub mod hanb;
pub mod constants;


pub fn print_board(board: &str, width: u8) {
    if board.is_empty() || board.len() > 64 {
        eprintln!(
            "Input string must be 1 to 64 characters long. You entered with {}",
            board.len()
        );
        return;
    }
    if width < 10 {
        eprintln!("Width must be greater than {}", 9);
        return;
    }
    let mut board = board.chars();
    let left_padding = (width - DIAGONAL) / 2;
    let right_padding = width - DIAGONAL - left_padding;
    for r in 0..DIAGONAL {
        let mut n_cells = r + 5 as u8;
        if r > DIAGONAL / 2 {
            n_cells = DIAGONAL / 2 + DIAGONAL - r;
        }
        let offset: i8 = (r as i8) - (DIAGONAL as i8) / 2;
        for _ in 0..(left_padding + offset.abs() as u8) {
            print!("  ");
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
                print!(" {}", char);
            } else {
                print!(" {}  ", char);
            }
        }
        for _ in 0..(right_padding + offset.abs() as u8) {
            print!("  ");
        }
        println!();
    }
}

pub fn eval(navigator: &mut Navigator, line: &str, stdout: bool) -> Result<(), String> {
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
            print_board(&navigator.current_board().to_string(), DEFAULT_WIDTH);
        }
        "save" | "sv" => return Err("Not implemented".to_string()),
        "load" | "ld" => return Err("Not implemented".to_string()),
        "down" | "d" => {
            let cell = args
                .parse::<usize>()
                .map_err(|_| "Cell must be a number".to_string())?;
            if cell >= navigator.root_board.cells.len() {
                return Err(format!(
                    "Cell must be between 0 and {}",
                    navigator.root_board.cells.len() - 1
                ));
            }
            match navigator.descend(cell) {
                Ok(board) => {
                    if stdout {
                        println!(
                            "Resolved cell '{}'. Board color '{}' You see:",
                            cell, board.color
                        );
                        print_board(&board.to_string(), DEFAULT_WIDTH);
                    }
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
        "up" | "u" => match navigator.ascend() {
            Some(board) => {
                if stdout {
                    println!("You ascend to the upper board. You see:");
                    print_board(&board.to_string(), DEFAULT_WIDTH);
                }
            }
            None => {
                println!("{}", navigator.current_board().color);
            }
        },
        "define" | "df" => {
            let mut args = args.split_whitespace();
            let cell = args
                .next()
                .unwrap()
                .parse::<usize>()
                .map_err(|_| "Cell must be a number".to_string())?;
            if cell >= navigator.root_board.cells.len() {
                return Err(format!(
                    "Cell must be between 0 and {}",
                    navigator.root_board.cells.len() - 1
                ));
            }
            let value = args.collect::<Vec<&str>>().join(" ");
            if value.len() > 64 {
                return Err("Value must be 1 to 64 characters long".to_string());
            }
            match navigator.define(cell, value.as_str()) {
                Ok(_) => {
                    if stdout {
                        print_board(&navigator.current_board().to_string(), DEFAULT_WIDTH);
                    }
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
        _ => {
            return Err(format!(
                "Unknown command: {}. Type help to see available commands.",
                command
            ));
        }
    }
    Ok(())
}
