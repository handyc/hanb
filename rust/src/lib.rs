const BOARD_SIZE: usize = 61;

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

    /// Create a new cell with a value and a board.
    pub fn new_with_board(value: char, board: Board) -> Self {
        Self {
            value,
            board: Some(board),
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

/// Hanb board. A board is a collection of 61 cells + 3 optional characters that are the board
/// color.
pub struct Board {
    /// The cells of the board.
    pub cells: Vec<Cell>,
    /// The optional characters of the board.
    pub color: String,
}

impl Board {
    /// Create a new board from a BOARD_SIZE long string. Can optionally include a color.
    pub fn new(board: &str) -> Result<Self, String> {
        if board.len() < BOARD_SIZE {
            return Err(format!(
                "Board must be at least {} characters long. You entered {}",
                BOARD_SIZE,
                board.len()
            ));
        }
        if board.len() > BOARD_SIZE + 3 {
            return Err(format!(
                "Board must be at most {} characters long. You entered {}",
                BOARD_SIZE + 3,
                board.len()
            ));
        }
        let mut cells = Vec::new();
        for c in board.chars().take(BOARD_SIZE) {
            cells.push(Cell::new(c));
        }
        let color = board.chars().skip(BOARD_SIZE).take(3).collect();
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
    pub fn descend(&mut self, cell: usize) -> Option<&Board> {
        self.resolve(cell).ok()?;
        self.cell_stack.push(cell);
        Some(self.current_board())
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
}
