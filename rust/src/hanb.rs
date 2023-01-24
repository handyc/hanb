use std::{cmp::min, fmt};

use crate::constants::{BOARD_SIZE, SIZES};

static DEFAULT_CELL_SIZE: &str = ".";

/// Compares two characters in the SIZE sequence. If a character index is greater than the other it
/// is considered to be bigger.
pub fn size_greater(a: char, b: char) -> bool {
    let a_index = SIZES.chars().position(|r| r == a).unwrap();
    let b_index = SIZES.chars().position(|r| r == b).unwrap();
    a_index > b_index
}

pub fn size_smaller(a: char, b: char) -> bool {
    !size_greater(a, b)
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

/// Hanb board. A board is a collection of 61 cells + 3 optional characters that are the board
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
            if SIZES.contains(c) {
                output.push(c);
            } else {
                output.push(match c {
                    ' ' => SIZES.chars().rev().nth(1).unwrap(),
                    _ => SIZES.chars().rev().next().unwrap(),
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
        // Complete the remaining with '.'
        if truncated_board.len() < BOARD_SIZE + 3 {
            truncated_board.push_str(&DEFAULT_CELL_SIZE.repeat(BOARD_SIZE + 3 - truncated_board.len()));
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
        write!(f, "{}", board)
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
            return Err(format!("Invalid level: {}", level));
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
    pub fn ascend(&mut self) -> Result<&Board, String> {
        let index = SIZES.find(self.level).unwrap();
        if index == SIZES.len() - 1 {
            return Err(format!(
                "You've reached the top limit fo the universe: {}",
                self.current_board().color
            ));
        }
        match self.cell_stack.pop() {
            Some(it) => it,
            None => return Err("You can't go upper".to_string()),
        };
        self.level = SIZES.chars().nth(index + 1).unwrap();
        Ok(self.current_board())
    }

    /// Go to a cell in the current board returning it
    pub fn descend(&mut self, cell: usize) -> Result<&Board, String> {
        let index = SIZES.find(self.level).unwrap();
        if index == 0 {
            return Err(format!(
                "You've reached the bottom limit fo the universe: {}",
                self.current_board().color
            ));
        }
        self.level = SIZES.chars().nth(index - 1).unwrap();
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
            self.add_cell(pos)?;
        }
        let cells = &self.current_board().cells;
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
    /// current level.
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
        while self.ascend().is_ok() {}
    }

    fn serialize(&mut self) -> String {
        let mut situation = String::new();
        situation.push_str(self.get_path().as_str());
        situation.push(' ');
        let clone = self.clone();
        let cells = &clone.current_board().cells;
        for cell in cells.iter() {
            situation.push(cell.size);
        }
        situation.push('\n');
        for (i, _) in cells.iter().enumerate() {
            if self.current_board().cells[i].board.is_some() {
                self.descend(i).unwrap();
                situation.push_str(self.serialize().as_str());
                self.ascend().unwrap();
            }
        }
        situation
    }

    /// Serialize into a situaion string representation
    pub fn get_situation(&mut self) -> String {
        // Store the path to restore it later
        let current_path = self.cell_stack.clone();
        self.go_to_root();
        let situation = self.serialize();
        // Return to the current path
        for cell in current_path {
            self.descend(cell).unwrap();
        }
        situation
    }

    /// Creates a navigator from a situation string representation
    pub fn from_situation(_situation: &str) -> Result<Self, String> {
        todo!()
    }
}
