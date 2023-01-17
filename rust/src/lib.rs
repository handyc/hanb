use std::cmp::min;

const BOARD_SIZE: usize = 61;
const DIAGONAL: u8 = 9;
const SIZES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-.";

pub const DEFAULT_WIDTH: u8 = 12;

/// Hanb cell. A cell has a fixed value and can resolve to a dynamic board.
pub struct Cell {
    /// The value of the cell.
    pub value: char,
    /// The board that this cell resolves to.
    pub board: Option<Board>,
}

impl Cell {
    /// Create a new cell with a value.
    pub fn new(value: char) -> Self {
        Self { value, board: None }
    }

    /// Define the board that this cell resolves to.
    pub fn set_board(&mut self, board: Board) {
        self.board = Some(board);
    }

    /// Define the board that this cell resolves to from a string.
    pub fn set_board_from_str(&mut self, board: &str) -> Result<(), String> {
        let board = Board::new(board)?;
        self.set_board(board);
        Ok(())
    }

    /// Get the board that this cell resolves to.
    pub fn get_board(&self) -> Option<&Board> {
        self.board.as_ref()
    }
}

/// Hanb board. A board is a collection of 61 cells + 3 optional characters that are the board
/// color.
pub struct Board {
    /// The cells of the board.
    pub cells: Vec<Cell>,
    /// The optional characters of the board.
    pub color: String,
}

impl Board {
    /// Preprocess input characters in a board
    fn preprocess(input: &str) -> String {
        let mut output = String::new();
        for c in input.chars() {
            if SIZES.contains(c) {
                output.push(c);
            } else {
                output.push(match c {
                    ' ' => '-',
                    // TODO what to do with other invalid characters?
                    _ => '.',
                });
            }
        }
        output
    }

    /// Create a new board from a BOARD_SIZE long string. Can optionally include a color.
    pub fn new(board: &str) -> Result<Self, String> {
        if board.len() < 1 {
            return Err(format!(
                "Board must be at least {} characters long. You entered {}",
                1,
                board.len()
            ));
        }
        let mut truncated_board = Board::preprocess(&board[0..min(board.len(), BOARD_SIZE + 3)]);
        // Complete the remaining with '.'
        truncated_board += &".".repeat(BOARD_SIZE + 3 - board.len());
        let mut cells = Vec::new();
        for c in truncated_board.chars().take(BOARD_SIZE) {
            cells.push(Cell::new(c));
        }
        let color = truncated_board.chars().skip(BOARD_SIZE).take(3).collect();
        Ok(Board { cells, color })
    }

    /// Converts the board to a string.
    pub fn to_string(&self) -> String {
        let mut board = String::new();
        for cell in &self.cells {
            board.push(cell.value);
        }
        board.push_str(&self.color);
        board
    }
}

/// Implement the board navigator that keeps track of the current level and board.
pub struct Navigator {
    pub root_board: Board,
    pub cell_stack: Vec<usize>,
}

impl Navigator {
    /// Create a new navigator with a board.
    pub fn new(board: Board) -> Self {
        Self {
            root_board: board,
            cell_stack: Vec::new(),
        }
    }

    /// Get the current board.
    pub fn current_board(&self) -> &Board {
        let mut board = &self.root_board;
        for cell in &self.cell_stack {
            board = board.cells[*cell].get_board().unwrap();
        }
        board
    }

    /// Return to previous/upper board returning it
    pub fn ascend(&mut self) -> Option<&Board> {
        self.cell_stack.pop()?;
        Some(self.current_board())
    }

    /// Go to a cell in the current board returning it
    pub fn descend(&mut self, cell: usize) -> Result<&Board, String> {
        self.resolve(cell)?;
        self.cell_stack.push(cell);
        Ok(self.current_board())
    }

    /// Resolve the cell at position `pos` in the current board.
    pub fn resolve(&mut self, pos: usize) -> Result<&Board, String> {
        if pos >= BOARD_SIZE {
            return Err(format!(
                "Position must be less than {}. You entered {}",
                BOARD_SIZE, pos
            ));
        }
        let cells = &self.current_board().cells;
        if cells[pos].board.is_none() {
            return Err(format!(
                "Cell at position {} does not resolve to a board",
                pos
            ));
        }
        let cell = &cells[pos];
        if let Some(board) = cell.get_board() {
            Ok(board)
        } else {
            Err(format!(
                "Cell at position {} does not resolve to a board",
                pos
            ))
        }
    }

    /// Define the board for a cell in the current board.
    pub fn define(&mut self, pos: usize, value: &str) -> Result<(), String> {
        if pos >= self.current_board().cells.len() {
            return Err(format!(
                "Position must be less than {}. You entered {}",
                self.current_board().cells.len(),
                pos
            ));
        }

        let mut board = &mut self.root_board;
        for cell in &self.cell_stack {
            board = board.cells[*cell].board.as_mut().unwrap();
        }
        board.cells[pos].set_board_from_str(value)?;
        Ok(())
    }
}

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
