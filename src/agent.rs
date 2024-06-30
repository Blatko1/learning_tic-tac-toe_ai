use crate::board::{Board, BoardFieldPosition, FieldState};
use hashbrown::HashMap;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

const TOTAL_BOARD_VARIATIONS: usize = 3usize.pow(9);

pub struct Agent {
    board_memory: HashMap<Board, Vec<AgentAction>>,
    epsilon: f64,
    rng: ThreadRng,
}

impl Agent {
    pub fn new_blank(epsilon: f64) -> Self {
        let mut board_memory = HashMap::new();
        let mut board = Board([[FieldState::Empty; 3]; 3]);
        let possible_moves: Vec<AgentAction> = board
            .get_empty_fields()
            .iter()
            .map(|field| AgentAction::new(*field))
            .collect();
        board_memory.insert(board.clone(), possible_moves);

        'boards: for i in 1..TOTAL_BOARD_VARIATIONS {
            let mut i = i;
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

            // There must be equal number of crosses and circles or
            // the number of crosses is by one higher than number of circles.
            // Total amount of crosses and circles (total moves) must be less than 8
            // meaning the board will be filtered if all spaces are filled or just one
            // is empty (since one is empty there is only one move available so no thinking needed)
            let cross_count = board.cross_count();
            let circle_count = board.circle_count();
            let total_moves = cross_count + circle_count;
            if (cross_count != circle_count && cross_count != circle_count + 1) || total_moves >= 8
            {
                continue 'boards;
            }

            // Filters for if the board is invalid or unusable
            let is_all_same = |row: &[FieldState; 3]| -> bool {
                let first = row[0];
                first != FieldState::Empty && first == row[1] && first == row[2]
            };
            // Check each row for three of a kind
            for y in 0..3 {
                if is_all_same(&board.0[y]) {
                    continue 'boards;
                }
            }
            // Check each column for three of a kind
            for x in 0..3 {
                if is_all_same(&[board.0[0][x], board.0[1][x], board.0[2][x]]) {
                    continue 'boards;
                }
            }
            // Check diagonals for three of a kind
            if is_all_same(&[board.0[0][0], board.0[1][1], board.0[2][2]]) {
                continue 'boards;
            }
            if is_all_same(&[board.0[2][0], board.0[1][1], board.0[0][2]]) {
                continue 'boards;
            }

            if Self::_find_identical_transformed_stored_board(&board_memory, &board).is_some() {
                continue 'boards;
            }

            let possible_moves: Vec<AgentAction> = board
                .get_empty_fields()
                .iter()
                .map(|field| AgentAction::new(*field))
                .collect();
            board_memory.insert(board.clone(), possible_moves);
        }

        let mut out = String::new();
        for (i, k) in board_memory.keys().enumerate() {
            out.push_str(&format!("id: {}\n{}", i, k));
        }
        std::fs::write("./test.txt", out).unwrap();

        Self {
            board_memory,
            epsilon,
            rng: rand::thread_rng(),
        }
    }

    fn _find_identical_transformed_stored_board(
        board_memory: &HashMap<Board, Vec<AgentAction>>,
        board: &Board,
    ) -> Option<(Board, BoardTransformation)> {
        // Rotated 90 degrees clockwise
        let rotated = board.get_rotated_90_clockwise();
        if board_memory.contains_key(&rotated) {
            return Some((rotated, BoardTransformation::Rotated90CW));
        }
        // Rotated 180 degrees
        let rotated = rotated.get_rotated_90_clockwise();
        if board_memory.contains_key(&rotated) {
            return Some((rotated, BoardTransformation::Rotated180));
        }
        // Rotated 270 clockwise
        let rotated = rotated.get_rotated_90_clockwise();
        if board_memory.contains_key(&rotated) {
            return Some((rotated, BoardTransformation::Rotated90CCW));
        }

        let flipped = board.get_flipped_horizontally();
        if board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedHorizontally));
        }
        let flipped = board.get_flipped_vertically();
        if board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedVertically));
        }
        let flipped = board.get_flipped_diagonally_southwest_northeast();
        if board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedDiagonallySWNE));
        }
        let flipped = board.get_flipped_diagonally_northwest_southeast();
        if board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedDiagonallyNWSE));
        }
        None
    }

    fn find_identical_transformed_stored_board(
        &self,
        board: &Board,
    ) -> Option<(Board, BoardTransformation)> {
        Self::_find_identical_transformed_stored_board(&self.board_memory, board)
    }

    pub fn play_greedy(&self, board: &Board) -> BoardFieldPosition {
        // Find the transformed version of the board in memory
        let (board, transformation) = self
            .find_identical_transformed_stored_board(board)
            .unwrap_or((board.clone(), BoardTransformation::None));
        println!("out transformed: {}", board);
        // Get the best learnt move
        let field_pos = self
            .board_memory
            .get(&board)
            .unwrap()
            .iter()
            .max()
            .unwrap()
            .field;

        // Transform to real board position
        transformation.transform_back_field_position(field_pos)
    }

    pub fn play_exploration(&mut self, board: &Board) -> BoardFieldPosition {
        // Find the transformed version of the board in memory
        let (board, transformation) = self
            .find_identical_transformed_stored_board(board)
            .unwrap_or((board.clone(), BoardTransformation::None));

        // Get the best learnt move
        let field_pos = self
            .board_memory
            .get(&board)
            .unwrap()
            .as_slice()
            .choose(&mut self.rng)
            .unwrap()
            .field;

        // Transform to real board position
        transformation.transform_back_field_position(field_pos)
    }

    pub fn play_greedy_exploration(&mut self, board: &Board) -> BoardFieldPosition {
        let action = if self.rng.gen_bool(self.epsilon) {
            self.play_exploration(board)
        } else {
            self.play_greedy(board)
        };

        action
    }

    pub fn memorized_boards_count(&self) -> usize {
        self.board_memory.len()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct AgentAction {
    field: BoardFieldPosition,
    bias: i32,
}

impl AgentAction {
    pub fn new(field: BoardFieldPosition) -> Self {
        Self { field, bias: 0 }
    }

    pub fn give_feedback(&mut self, reward: i32) {
        self.bias += reward
    }
}

impl Ord for AgentAction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bias.cmp(&other.bias)
    }
}

enum BoardTransformation {
    None,
    Rotated90CW,
    Rotated180,
    Rotated90CCW,
    FlippedHorizontally,
    FlippedVertically,
    FlippedDiagonallySWNE,
    FlippedDiagonallyNWSE,
}

impl BoardTransformation {
    fn transform_back_field_position(&self, field_pos: BoardFieldPosition) -> BoardFieldPosition {
        match self {
            BoardTransformation::None => field_pos,
            BoardTransformation::Rotated90CW => match field_pos {
                BoardFieldPosition::NorthWest => BoardFieldPosition::SouthWest,
                BoardFieldPosition::North => BoardFieldPosition::West,
                BoardFieldPosition::NorthEast => BoardFieldPosition::NorthWest,
                BoardFieldPosition::West => BoardFieldPosition::South,
                BoardFieldPosition::East => BoardFieldPosition::North,
                BoardFieldPosition::SouthWest => BoardFieldPosition::SouthEast,
                BoardFieldPosition::South => BoardFieldPosition::East,
                BoardFieldPosition::SouthEast => BoardFieldPosition::NorthEast,
                p => p,
            },
            BoardTransformation::Rotated180 => match field_pos {
                BoardFieldPosition::NorthWest => BoardFieldPosition::SouthEast,
                BoardFieldPosition::North => BoardFieldPosition::South,
                BoardFieldPosition::NorthEast => BoardFieldPosition::SouthWest,
                BoardFieldPosition::West => BoardFieldPosition::East,
                BoardFieldPosition::East => BoardFieldPosition::West,
                BoardFieldPosition::SouthWest => BoardFieldPosition::NorthEast,
                BoardFieldPosition::South => BoardFieldPosition::North,
                BoardFieldPosition::SouthEast => BoardFieldPosition::NorthWest,
                p => p,
            },
            BoardTransformation::Rotated90CCW => match field_pos {
                BoardFieldPosition::NorthWest => BoardFieldPosition::NorthEast,
                BoardFieldPosition::North => BoardFieldPosition::East,
                BoardFieldPosition::NorthEast => BoardFieldPosition::SouthEast,
                BoardFieldPosition::West => BoardFieldPosition::North,
                BoardFieldPosition::East => BoardFieldPosition::South,
                BoardFieldPosition::SouthWest => BoardFieldPosition::NorthWest,
                BoardFieldPosition::South => BoardFieldPosition::West,
                BoardFieldPosition::SouthEast => BoardFieldPosition::SouthWest,
                p => p,
            },
            BoardTransformation::FlippedVertically => match field_pos {
                BoardFieldPosition::NorthWest => BoardFieldPosition::SouthWest,
                BoardFieldPosition::North => BoardFieldPosition::South,
                BoardFieldPosition::NorthEast => BoardFieldPosition::SouthEast,
                BoardFieldPosition::SouthWest => BoardFieldPosition::NorthWest,
                BoardFieldPosition::South => BoardFieldPosition::North,
                BoardFieldPosition::SouthEast => BoardFieldPosition::NorthEast,
                p => p,
            },
            BoardTransformation::FlippedHorizontally => match field_pos {
                BoardFieldPosition::NorthWest => BoardFieldPosition::NorthEast,
                BoardFieldPosition::NorthEast => BoardFieldPosition::NorthWest,
                BoardFieldPosition::West => BoardFieldPosition::East,
                BoardFieldPosition::East => BoardFieldPosition::West,
                BoardFieldPosition::SouthWest => BoardFieldPosition::SouthEast,
                BoardFieldPosition::SouthEast => BoardFieldPosition::SouthWest,
                p => p,
            },
            BoardTransformation::FlippedDiagonallySWNE => match field_pos {
                BoardFieldPosition::North => BoardFieldPosition::West,
                BoardFieldPosition::NorthEast => BoardFieldPosition::SouthWest,
                BoardFieldPosition::West => BoardFieldPosition::North,
                BoardFieldPosition::East => BoardFieldPosition::South,
                BoardFieldPosition::SouthWest => BoardFieldPosition::NorthEast,
                BoardFieldPosition::South => BoardFieldPosition::East,
                p => p,
            },
            BoardTransformation::FlippedDiagonallyNWSE => match field_pos {
                BoardFieldPosition::NorthWest => BoardFieldPosition::SouthEast,
                BoardFieldPosition::North => BoardFieldPosition::East,
                BoardFieldPosition::West => BoardFieldPosition::South,
                BoardFieldPosition::East => BoardFieldPosition::North,
                BoardFieldPosition::South => BoardFieldPosition::West,
                BoardFieldPosition::SouthEast => BoardFieldPosition::NorthWest,
                p => p,
            },
        }
    }
}
