use hashbrown::HashMap;
use crate::game::Board;

const TOTAL_BOARD_VARIATIONS: u64 = 3u64.pow(9);


pub struct Agent {
    board_memory: HashMap<Board, Vec<PossibleMove>>
}

impl Agent {
    pub fn new_blank() -> Self {
        for iteration in 0u32..9u32 {
            let variations = 3u64.pow(iteration + 1);
            for field in 0..
        }
        todo!()
    }
}

struct PossibleMove {
    field: BoardField,
    probability: f32
}

enum BoardField {
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