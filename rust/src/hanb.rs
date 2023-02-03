use std::{cmp::min, fmt};

use crate::constants::{ALPHABET, BOARD_SIZE, DEFAULT_CELL_SIZE, SIZES};

/// Compares two characters in the SIZE sequence. If a character index is greater than the other it
/// is considered to be bigger.
pub fn size_greater(a: char, b: char) -> bool {
    let a_index = SIZES.chars().position(|r| r == a).unwrap();
    let b_index = SIZES.chars().position(|r| r == b).unwrap();
    a_index > b_index
}

pub fn size_smaller(a: char, b: char) -> bool {
    !size_greater(a, b) && a != b
}

/// Hanb cell. A cell has a fixed value and can resolve to a dynamic board.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The Size of the cell.
    pub size: char,
    /// The board that this cell resolves to.
    pub board: Option<Board>,
}

impl Cell {
    /// Create a new cell with a value.
    pub fn new(value: char) -> Self {
        Self {
            size: value,
            board: None,
        }
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.size)
    }
}

/// Hanb board. A board is a collection of 61 cells + 3 optional characters that are the board's
/// color.
#[derive(Debug, Clone)]
pub struct Board {
    /// The cells of the board.
    pub cells: Vec<Cell>,
    /// The optional characters of the board.
    pub color: String,
}

impl Board {
    /// Preprocess input characters in a board limiting to the valid SIZES
    fn preprocess(input: &str) -> String {
        let mut output = String::new();
        for c in input.chars() {
            if ALPHABET.contains(c) {
                output.push(c);
            } else {
                output.push(match c {
                    ' ' => ALPHABET.chars().rev().nth(1).unwrap(),
                    _ => ALPHABET.chars().rev().next().unwrap(),
                });
            }
        }
        output
    }

    /// Create a new board from a BOARD_SIZE long string. Can optionally include a color.
    pub fn new(board: &str) -> Result<Self, String> {
        if board.is_empty() {
            return Err(format!(
                "Board must be at least {} characters long. You entered {}",
                1,
                board.len()
            ));
        }
        let mut truncated_board = Board::preprocess(&board[0..min(board.len(), BOARD_SIZE + 3)]);
        // Complete the remaining with 'a'
        if truncated_board.len() < BOARD_SIZE + 3 {
            truncated_board
                .push_str(&DEFAULT_CELL_SIZE.repeat(BOARD_SIZE + 3 - truncated_board.len()));
        }
        let mut cells = Vec::new();
        for c in truncated_board.chars().take(BOARD_SIZE) {
            cells.push(Cell::new(c));
        }
        let color = truncated_board.chars().skip(BOARD_SIZE).take(3).collect();
        Ok(Board { cells, color })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for cell in &self.cells {
            board.push(cell.size);
        }
        board.push_str(&self.color);
        write!(f, "{board}")
    }
}

/// Implement the board navigator that keeps track of the current level and board.
#[derive(Debug, Clone)]
pub struct Navigator {
    pub root_board: Board,
    pub cell_stack: Vec<usize>,
    pub level: char,
}

impl Navigator {
    /// Create a new navigator with a board.
    pub fn new(level: char) -> Result<Self, String> {
        if !SIZES.contains(level) {
            return Err(format!("Invalid level: {level}"));
        }
        let root_board = Board::new(&DEFAULT_CELL_SIZE.repeat(BOARD_SIZE))?;
        Ok(Self {
            root_board,
            cell_stack: Vec::new(),
            level,
        })
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
    pub fn up(&mut self) -> Result<&Board, String> {
        let index = SIZES.find(self.level).unwrap();
        if index == SIZES.len() - 1 {
            return Err(format!(
                "You've reached the top limit fo the universe\nColor {}",
                self.current_board().color
            ));
        }
        match self.cell_stack.pop() {
            Some(it) => it,
            None => {
                return Err(format!(
                    "You can't go upper\nColor: {}",
                    self.current_board().color
                ))
            }
        };
        // Compute current level and board
        let mut board = &self.root_board;
        for cell in &self.cell_stack {
            self.level = board.cells[*cell].size;
            board = board.cells[*cell].get_board().unwrap();
        }
        if self.cell_stack.is_empty() {
            self.level = SIZES.chars().rev().next().unwrap();
        }
        Ok(board)
    }

    /// Go to a cell in the current board returning it
    pub fn down(&mut self, cell: usize) -> Result<&Board, String> {
        if cell >= BOARD_SIZE {
            return Err(format!(
                "Position must be less than {BOARD_SIZE}. You entered {cell}"
            ));
        }
        let cell_size = self.current_board().cells[cell].size;
        let index = SIZES.find(cell_size).unwrap();
        if index == 0 {
            return Err(format!(
                "You can't go smaller than: {}\nColor: {}",
                cell_size,
                self.current_board().color
            ));
        }
        self.level = SIZES.chars().nth(index).unwrap();
        self.resolve(cell)?;
        self.cell_stack.push(cell);
        Ok(self.current_board())
    }

    /// Resolve the cell at position `pos` in the current board.
    pub fn resolve(&mut self, pos: usize) -> Result<&Board, String> {
        if pos >= BOARD_SIZE {
            return Err(format!(
                "Position must be less than {BOARD_SIZE}. You entered {pos}"
            ));
        }
        let cells = &self.current_board().cells;
        if cells[pos].board.is_none() {
            self.add_cell(pos)?;
        }
        let cells = &self.current_board().cells;
        let cell = &cells[pos];
        if let Some(board) = cell.get_board() {
            Ok(board)
        } else {
            Err(format!(
                "Cell at position {pos} does not resolve to a board"
            ))
        }
    }

    /// Adds a undefine cell to a board
    pub fn add_cell(&mut self, pos: usize) -> Result<(), String> {
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
        board.cells[pos].set_board_from_str(DEFAULT_CELL_SIZE)?;
        Ok(())
    }

    /// Sets the current board's cells to new values.
    /// If any of the values are bigger than the current level they will be truncated to the
    /// current level. Will also recursively truncate inner boards if they exist.
    pub fn set_board(&mut self, board: &str) -> Result<(), String> {
        let board = Board::new(board)?;
        let mut current_board = &mut self.root_board;
        for cell in &self.cell_stack {
            current_board = current_board.cells[*cell].board.as_mut().unwrap();
        }
        for (i, cell) in board.cells.iter().enumerate() {
            if size_smaller(cell.size, self.level) {
                current_board.cells[i].size = cell.size;
            } else {
                let index = SIZES.find(self.level).unwrap();
                if index == 0 {
                    return Err(format!(
                        "Nothing can be smaller than this level: {}",
                        self.level
                    ));
                }
                current_board.cells[i].size = SIZES.chars().nth(index - 1).unwrap();
            }
        }
        current_board.color = board.color;
        Ok(())
    }

    /// Gets the board path as a string.
    pub fn get_path(&self) -> String {
        self.cell_stack
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<String>>()
            .join(" -> ")
    }

    pub fn go_to_root(&mut self) {
        while self.up().is_ok() {}
    }

    fn serialize(&mut self) -> String {
        let mut map = String::new();
        map.push_str(self.get_path().as_str());
        map.push(' ');
        let clone = self.clone();
        let cells = &clone.current_board().cells;
        for cell in cells.iter() {
            map.push(cell.size);
        }
        map.push('\n');
        for (i, _) in cells.iter().enumerate() {
            if self.current_board().cells[i].board.is_some() {
                self.down(i).unwrap();
                map.push_str(self.serialize().as_str());
                self.up().unwrap();
            }
        }
        map
    }

    /// Serialize into a declarative map string representation
    pub fn get_explored_map(&mut self) -> String {
        // Store the path to restore it later
        let current_path = self.cell_stack.clone();
        self.go_to_root();
        let map = self.serialize();
        // Return to the current path
        for cell in current_path {
            self.down(cell).unwrap();
        }
        map
    }

    /// Creates a navigator from a explored map string representation
    pub fn from_map(level: char, map: &str) -> Result<Self, String> {
        // Loop through each line of the map, trim it
        // The path is in the first part in the format number -> number -> ...
        // The board is in the second part
        let navigator = Navigator::new(level);
        if navigator.is_err() {
            return Err(navigator.err().unwrap());
        }
        let mut navigator = navigator.unwrap();
        for line in map.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let args = line.split_whitespace();
            let mut board = String::new();
            let mut is_board = false;
            for value in args {
                if is_board {
                    board.push_str(value);
                    continue;
                }
                if !is_board && value == "->" {
                    continue;
                }
                match value.parse::<usize>() {
                    Ok(numeric) => {
                        match navigator.down(numeric) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(format!(
                                    "Invalid path in map: {}\n{}",
                                    navigator.get_path(),
                                    e
                                ))
                            }
                        };
                    }
                    Err(_) => {
                        is_board = true;
                        board.push_str(value);
                    }
                };
            }
            match navigator.set_board(&board) {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
            navigator.go_to_root();
        }
        Ok(navigator)
    }
}
