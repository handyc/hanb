use std::cmp::min;

use crate::constants::{SIZES, BOARD_SIZE};

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
pub struct Cell {
    /// The Size of the cell.
    pub size: char,
    /// The board that this cell resolves to.
    pub board: Option<Board>,
}

impl Cell {
    /// Create a new cell with a value.
    pub fn new(value: char) -> Self {
        Self { size: value, board: None }
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

    /// Converts the cell size to a string.
    pub(crate) fn to_string(&self) -> String {
        format!("{}", self.size)
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
        if truncated_board.len() < BOARD_SIZE + 3 {
            truncated_board.push_str(&".".repeat(BOARD_SIZE + 3 - truncated_board.len()));
        }
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
            board.push(cell.size);
        }
        board.push_str(&self.color);
        board
    }
}

/// Implement the board navigator that keeps track of the current level and board.
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
        let root_board = Board::new(&".".repeat(BOARD_SIZE))?;
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
            return Err(format!("You've reached the top limit fo the universe: {}", self.current_board().color));
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
            return Err(format!("You've reached the bottom limit fo the universe: {}", self.current_board().color));
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
    /// If the cell is already defined, it will be overwritten.
    /// If any of the input board cells are bigger than the current one whey will be truncated to
    /// the level smaller than the current one.
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
        let mut truncated_value = String::new();
        for char in value.chars() {
            if size_smaller(char, self.level) {
                truncated_value.push(self.level);
            } else {
                let index = SIZES.find(self.level).unwrap();
                if index == 0 {
                    return Err(format!(
                        "Nothing can be smaller than this level: {}",
                        self.level
                    ));
                }
                let level = SIZES.chars().nth(index - 1).unwrap();
                truncated_value.push(level);
            }
        }

        board.cells[pos].set_board_from_str(truncated_value.as_str())?;
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
        current_board.cells = board.cells;
        current_board.color = board.color;
        Ok(())
    }
}

