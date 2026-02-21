use crate::board::{Board, Cell};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn other(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub enum GameStatus {
    InProgress,
    Won(Player),
    Draw,
}

impl GameStatus {
    pub fn is_in_progress(&self) -> bool {
        matches!(self, GameStatus::InProgress)
    }
}
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub status: GameStatus,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::O,
            status: GameStatus::InProgress,
        }
    }
    pub fn apply_move(&mut self, position: usize) -> Result<(), String> {
        // 1. Ensure game is still active
        if !matches!(self.status, GameStatus::InProgress) {
            return Err("Game is already over.".into());
        }

        if position > 8 {
            return Err("Invalid board position.".into());
        }

        // 2. Convert 0–8 index into row/col
        let row = position / 3;
        let col = position % 3;

        // 3. Ensure cell is empty
        if self.board.cells[row][col] != Cell::Empty {
            return Err("Cell already occupied.".into());
        }

        // 4. Place mark
        self.board.cells[row][col] = self.current_player.into();

        // 5. Check for win
        if self.check_win(self.current_player) {
            self.status = GameStatus::Won(self.current_player);
            return Ok(());
        }

        // 6. Check for draw
        if self.board.is_full() {
            self.status = GameStatus::Draw;
            return Ok(());
        }

        // 7. Otherwise switch player
        self.current_player = self.current_player.other();

        Ok(())
    }

    pub fn check_win(&self, player: Player) -> bool {
        let target: Cell = player.into();
        let b = &self.board.cells;

        // Rows
        for i in 0..3 {
            if b[i][0] == target && b[i][1] == target && b[i][2] == target {
                return true;
            }
        }

        // Columns
        for i in 0..3 {
            if b[0][i] == target && b[1][i] == target && b[2][i] == target {
                return true;
            }
        }

        // Diagonals
        if (b[0][0] == target && b[1][1] == target && b[2][2] == target)
            || (b[0][2] == target && b[1][1] == target && b[2][0] == target)
        {
            return true;
        }

        false
    }
}
