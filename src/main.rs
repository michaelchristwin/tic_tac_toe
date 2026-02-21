mod board;
mod game;
mod input;

use game::{GameState, GameStatus};
use input::read_position;

fn main() {
    let mut game = GameState::new();

    while game.status.is_in_progress() {
        game.board.print();

        println!("Player {:?}'s turn", game.current_player);

        let pos = read_position();

        match game.apply_move(pos) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                continue; // retry same player
            }
        }
    }

    game.board.print();

    match game.status {
        GameStatus::Won(player) => {
            println!("Player {:?} wins!", player);
        }
        GameStatus::Draw => {
            println!("It's a draw.");
        }
        _ => {}
    }
}
