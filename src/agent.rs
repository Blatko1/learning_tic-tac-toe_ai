use std::io::Write;

use crate::game::{Board, FieldState};
use hashbrown::HashMap;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

const TOTAL_BOARD_VARIATIONS: usize = 3usize.pow(9);

pub struct Agent {
    board_memory: HashMap<Board, Vec<PossibleMove>>,
    epsilon: f32,
    rng: ThreadRng
}

impl Agent {
    pub fn new_blank() -> Self {
        let mut board_memory = HashMap::new();
        let mut board = Board([[FieldState::Empty; 3]; 3]);
        board_memory.insert(board.clone(), board.get_possible_moves());

        'boards: for i in 1..TOTAL_BOARD_VARIATIONS {
            let mut i = i;
            let first = &mut board.0[0][0];
            *first = first.shift();
            // Change each field relative to the iteration number
            (0..3).for_each(|y| {
                (0..3).for_each(|x| {
                    board.0[y][x] = match i % 3 {
                        0 => FieldState::Empty,
                        1 => FieldState::X,
                        2 => FieldState::O,
                        _ => unreachable!(),
                    };
                    i /= 3;
                })
            });

            let is_all_same = |row: &[FieldState; 3]| -> bool {
                let first = row[0];
                first != FieldState::Empty && first == row[1] && first == row[2]
            };

            // Check each row for three same
            for y in 0..3 {
                if is_all_same(&board.0[y]) {
                    continue 'boards;
                }
            }

            // Check each column for three same
            for x in 0..3 {
                if is_all_same(&[board.0[0][x], board.0[1][x], board.0[2][x]]) {
                    continue 'boards;
                }
            }

            // Check diagonals for three same
            if is_all_same(&[board.0[0][0], board.0[1][1], board.0[2][2]]) {
                continue 'boards;
            }
            if is_all_same(&[board.0[2][0], board.0[1][1], board.0[0][2]]) {
                continue 'boards;
            }

            // There must be equal crosses and circles or number
            // of crosses is by one higher than number of circles.
            // Amount of crosses and circles (total moves) must be less than 9
            let cross_count = board.cross_count();
            let circle_count = board.circle_count();
            let total_moves = cross_count + circle_count;
            if (cross_count == circle_count || cross_count == circle_count + 1) && total_moves != 9 {
                board_memory.insert(board.clone(), board.get_possible_moves());
            }
        }

        let mut out = String::new();
        for (i, k) in board_memory.keys().enumerate() {
            out.push_str(&format!("id: {}\n{}", i, k));
        }
        std::fs::write("./test.txt", out).unwrap();

        Self { board_memory, epsilon: 0.0, rng: rand::thread_rng()  }
    }

    pub fn play(&self, board: &Board) -> BoardField {
        self.board_memory.get(board).unwrap().iter().max().unwrap().field
    }

    pub fn play_and_learn(&mut self, board: &Board) -> &mut PossibleMove {
        let rng: f32 = self.rng.gen();
        if rng < self.epsilon {
            self.board_memory.get_mut(board).unwrap().as_mut_slice().choose_mut(&mut self.rng).unwrap()
        } else {
            self.board_memory.get_mut(board).unwrap().iter_mut().max().unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct PossibleMove {
    field: BoardField,
    bias: i32,
}

impl PossibleMove {
    pub fn new(field: BoardField) -> Self {
        Self {
            field,
            bias: 0,
        }
    }
}

impl Ord for PossibleMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bias.cmp(&other.bias)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BoardField {
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl BoardField {
    pub fn from_pos(x: usize, y: usize) -> Self {
        match (x, y) {
            (0, 0) => Self::NorthWest,
            (1, 0) => Self::North,
            (2, 0) => Self::NorthEast,
            (0, 1) => Self::West,
            (1, 1) => Self::Center,
            (2, 1) => Self::East,
            (0, 2) => Self::SouthWest,
            (1, 2) => Self::South,
            (2, 2) => Self::SouthEast,
            _ => unreachable!("Invalid board position! x: {}, y: {}", x, y),
        }
    }
}
