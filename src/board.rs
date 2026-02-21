use crate::game::Player;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
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

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Board {
    pub cells: [[Cell; 3]; 3],
}

impl Board {
    /// Creates a brand-new empty board (same as `Board::default()`)
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|&cell| cell != Cell::Empty))
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
