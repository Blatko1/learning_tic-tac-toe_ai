use crate::board::{Board, FieldState};

///
/// [[.,.,.],
///  [.,.,.],
///  [.,.,.]]
///
/// [[1,2,3],
///  [4,5,6],
///  [7,8,9]]
///
pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_new<F>(&self, mut event_handler: F)
    where
        F: FnMut(GameEvent, &mut Board),
    {
        let mut board = Board::EMPTY;
        loop {
            let event = match board.find_winner() {
                FieldState::X => GameEvent::CrossWon,
                FieldState::O => GameEvent::CircleWon,
                _ => {
                    let cross_count = board.field_state_count(FieldState::X);
                    let circle_count = board.field_state_count(FieldState::O);
                    let empty_field_count = board.field_state_count(FieldState::Empty);
                    if empty_field_count == 0 {
                        GameEvent::Draw
                    } else if cross_count == circle_count {
                        GameEvent::CrossTurn
                    } else if cross_count == (circle_count + 1) {
                        GameEvent::CircleTurn
                    } else {
                        GameEvent::InvalidBoard
                    }
                }
            };
            event_handler(event, &mut board);

            match event {
                GameEvent::CrossWon | GameEvent::CircleWon | GameEvent::Draw => break,
                _ => (),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    CrossWon,
    CircleWon,
    Draw,
    CrossTurn,
    CircleTurn,
    InvalidBoard,
}
