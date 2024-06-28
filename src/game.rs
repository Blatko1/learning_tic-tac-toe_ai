use std::fmt::Display;

use crate::agent::{BoardField, PossibleMove};

///
/// [[.,.,.],
///  [.,.,.],
///  [.,.,.]]
///
/// [[1,2,3],
///  [4,5,6],
///  [7,8,9]]
///
pub struct GameState {
    board: [[FieldState; 3]; 3],
}

impl GameState {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Board(pub [[FieldState; 3]; 3]);

impl Board {
    pub fn cross_count(&self) -> u64 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&field| match field {
                        FieldState::X => 1,
                        _ => 0,
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    pub fn circle_count(&self) -> u64 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&field| match field {
                        FieldState::O => 1,
                        _ => 0,
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    pub fn get_possible_moves(&self) -> Vec<PossibleMove> {
        let mut possible = Vec::new();
        for y in 0..3 {
            for x in 0..3 {
                match &self.0[y][x] {
                    FieldState::Empty => {
                        possible.push(PossibleMove::new(BoardField::from_pos(x, y)))
                    }
                    _ => (),
                }
            }
        }
        possible
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum FieldState {
    Empty = 0,
    X = 1,
    O = 2,
}

impl FieldState {
    pub fn shift(self) -> Self {
        match self {
            FieldState::Empty => FieldState::X,
            FieldState::X => FieldState::O,
            FieldState::O => FieldState::Empty,
        }
    }
}

impl Display for FieldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldState::Empty => write!(f, " "),
            FieldState::X => write!(f, "X"),
            FieldState::O => write!(f, "O"),
        }
    }
}