use hashbrown::HashMap;
use crate::game::{Board, FieldState};

const TOTAL_BOARD_VARIATIONS: usize = 3usize.pow(9);

pub struct Agent {
    board_memory: HashMap<Board, Vec<PossibleMove>>
}

impl Agent {
    pub fn new_blank() -> Self {
        let mut board_memory = HashMap::new();
        let mut board = Board([[FieldState::Empty; 3]; 3]);
        board_memory.insert(board.clone(), board.get_possible_moves());

        for i in 1..30 {
            let mut i = i;
            let first = &mut board.0[0][0];
            *first = first.shift();
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
            /*for exp in 1..9 {
                let x = exp as usize % 3;
                let y = exp as usize / 3;
                if i % 3usize.pow(exp) == 0 {
                    let field = &mut board.0[y][x];
                    *field = field.shift();
                }
            }*/
            board_memory.insert(board.clone(), board.get_possible_moves());
            println!(" {} | {} | {}", &board.0[0][0], &board.0[0][1], &board.0[0][2]);
            println!("------------");
            println!(" {} | {} | {}", &board.0[1][0], &board.0[1][1], &board.0[1][2]);
            println!("------------");
            println!(" {} | {} | {}\n\n", &board.0[2][0], &board.0[2][1], &board.0[2][2]);
        }
        todo!()
    }
}

pub struct PossibleMove {
    field: BoardField,
    probability: f32
}

impl PossibleMove {
    pub fn new(field: BoardField) -> Self {
        Self {
            field,
            probability: 1.0
        }
    }
}

pub enum BoardField {
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast
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
            _ => unreachable!("Invalid board position! x: {}, y: {}", x, y)
        }
    }
}