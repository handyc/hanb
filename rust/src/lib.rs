use constants::DIAGONAL;
use hanb::Navigator;

pub mod commands;
pub mod hanb;
pub mod constants;


pub fn print_level_board(navigator: &Navigator, width: u8) -> Result<String, String> {
    let level = navigator.level;
    let mut board_string = String::new();
    board_string += &format!("Level: {} - {}\n\n", level, navigator.get_path());
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
        let mut n_cells = r + 5 as u8;
        if r > DIAGONAL / 2 {
            n_cells = DIAGONAL / 2 + DIAGONAL - r;
        }
        let offset: i8 = (r as i8) - (DIAGONAL as i8) / 2;
        for _ in 0..(left_padding + offset.abs() as u8) {
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
        for _ in 0..(right_padding + offset.abs() as u8) {
            output += "  ";
        }
        output += "\n";
    }
    Ok(output)
}

pub fn eval(navigator: &mut Navigator, line: &str, stdout: bool) -> Result<(), String> {
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
        if cmd.command == command || cmd.short == command {
            match (cmd.action)(cmd, navigator, &args) {
                Ok(res) => {
                    if stdout || cmd.stdout {
                        println!("{}", res);
                    }
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        }
    }
    Err(format!("Command {} not found. Use 'help' to see available commands.", command))
}
