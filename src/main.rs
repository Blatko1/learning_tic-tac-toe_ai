use std::hash::{DefaultHasher, Hash, Hasher};

use game::{Board, FieldState};

mod agent;
mod game;

fn main() {
    let board = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::Empty],
        [FieldState::Empty, FieldState::Empty, FieldState::Empty],
        [FieldState::Empty, FieldState::Empty, FieldState::Empty],
    ]);
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    println!("hash is {}", hasher.finish());
}
