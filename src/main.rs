use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    X,
    O,
}

pub enum Player {
    X,
    O,
}

impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

pub struct GameState {
    current_player: Player,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    /// Creates a brand-new empty board (same as `Board::default()`)
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the cell at a given row/col (0-based)
    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    /// Set a cell (you'll probably want bounds checking in real code)
    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    /// Check if the board is completely full (no Empty cells)
    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|&c| c != Cell::Empty)
    }

    /// Pretty-print the board (very handy for debugging)
    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Empty => print!(" . "),
                    Cell::X => print!(" X "),
                    Cell::O => print!(" O "),
                }
            }
            println!();
        }
    }
}

fn main() {
    let board = Board::new();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    println!("Hello {}", input)
}
