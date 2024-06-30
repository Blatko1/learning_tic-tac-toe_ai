use crate::board::{Board, FieldPosition, FieldState};
use hashbrown::HashMap;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct Agent {
    board_memory: HashMap<Board, Vec<AgentAction>>,
    epsilon: f64,
    rng: ThreadRng,
    recorded_actions: Vec<RecordedAction>,
}

impl Agent {
    pub fn new_blank(epsilon: f64) -> Self {
        Self {
            board_memory: HashMap::new(),
            epsilon,
            rng: rand::thread_rng(),
            recorded_actions: Vec::with_capacity(9),
        }
    }

    pub fn get_actions_from_board(&self, board: &Board) -> Option<Vec<AgentAction>> {
        let (saved_board, transformation) = match self.get_saved_board(board) {
            Some(s) => s,
            None => return None,
        };
        match self.board_memory.get(&saved_board) {
            Some(actions) => Some(
                actions
                    .iter()
                    .map(|action| AgentAction {
                        field_pos: transformation.pos_to_original(action.field_pos),
                        bias: action.bias,
                    })
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn play_greedy_exploration(&mut self, board: &Board) -> Option<FieldPosition> {
        let empty_field_count = board.field_state_count(FieldState::Empty);
        if empty_field_count == 0 {
            return None;
        } else if empty_field_count == 1 {
            return Some(board.get_empty_fields_pos()[0]);
        }
        let (saved_board, transformation) = match self.get_saved_board(board) {
            Some(saved) => saved,
            None => {
                // Save the board if not already saved
                self.save_board(board.clone());
                (board.clone(), BoardTransformation::None)
            }
        };
        let action = if self.rng.gen_bool(self.epsilon) {
            // Choose a random action
            self.board_memory
                .get(&saved_board)
                .unwrap()
                .choose(&mut self.rng)
        } else {
            // Choose the best action
            self.board_memory.get(&saved_board).unwrap().iter().max()
        };

        let action = match action {
            Some(&a) => a,
            None => return None,
        };

        self.recorded_actions.push(RecordedAction {
            board: saved_board.clone(),
            action,
        });

        // Transform back to real board position
        Some(transformation.pos_to_original(action.field_pos))
    }

    fn save_board(&mut self, board: Board) {
        let actions: Vec<AgentAction> = board
            .get_empty_fields_pos()
            .iter()
            .map(|&pos| AgentAction::new(pos))
            .collect();
        self.board_memory.insert(board, actions);
    }

    fn get_saved_board(&self, board: &Board) -> Option<(Board, BoardTransformation)> {
        if self.board_memory.contains_key(board) {
            return Some((board.clone(), BoardTransformation::None));
        }
        // Also check if identical transformed board is saved
        // Rotated 90 degrees clockwise
        let rotated = board.get_rotated_90_clockwise();
        if self.board_memory.contains_key(&rotated) {
            return Some((rotated, BoardTransformation::Rotated90CW));
        }
        // Rotated 180 degrees
        let rotated = rotated.get_rotated_90_clockwise();
        if self.board_memory.contains_key(&rotated) {
            return Some((rotated, BoardTransformation::Rotated180));
        }
        // Rotated 270 clockwise
        let rotated = rotated.get_rotated_90_clockwise();
        if self.board_memory.contains_key(&rotated) {
            return Some((rotated, BoardTransformation::Rotated90CCW));
        }

        let flipped = board.get_flipped_horizontally();
        if self.board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedHorizontally));
        }
        let flipped = board.get_flipped_vertically();
        if self.board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedVertically));
        }
        let flipped = board.get_flipped_diagonally_southwest_northeast();
        if self.board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedDiagonallySWNE));
        }
        let flipped = board.get_flipped_diagonally_northwest_southeast();
        if self.board_memory.contains_key(&flipped) {
            return Some((flipped, BoardTransformation::FlippedDiagonallyNWSE));
        }
        None
    }

    pub fn give_feedback(&mut self, reward: i32) {
        if reward > 0 {
            self.epsilon = (self.epsilon * 0.995).max(0.15);
        }
        for recorded_action in self.recorded_actions.drain(..) {
            let board = &recorded_action.board;
            let action = &recorded_action.action;
            let saved_action = self
                .board_memory
                .get_mut(board)
                .unwrap()
                .iter_mut()
                .find(|saved_action| saved_action.field_pos == action.field_pos)
                .unwrap();
            saved_action.bias += reward;
        }
    }

    pub fn memorized_boards_count(&self) -> usize {
        self.board_memory.len()
    }

    pub fn epsilon(&self) -> f64 {
        self.epsilon
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct AgentAction {
    pub field_pos: FieldPosition,
    pub bias: i32,
}

impl AgentAction {
    pub fn new(field_pos: FieldPosition) -> Self {
        Self { field_pos, bias: 0 }
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

struct RecordedAction {
    board: Board,
    action: AgentAction,
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
    fn pos_to_original(&self, pos: FieldPosition) -> FieldPosition {
        let mut board = Board::EMPTY;
        board.0[pos.y][pos.x] = FieldState::X;
        let transformed_board = match self {
            BoardTransformation::None => return pos,
            BoardTransformation::Rotated90CW => board
                .get_rotated_90_clockwise()
                .get_rotated_90_clockwise()
                .get_rotated_90_clockwise(),
            BoardTransformation::Rotated180 => {
                board.get_rotated_90_clockwise().get_rotated_90_clockwise()
            }
            BoardTransformation::Rotated90CCW => board.get_rotated_90_clockwise(),
            BoardTransformation::FlippedHorizontally => board.get_flipped_horizontally(),
            BoardTransformation::FlippedVertically => board.get_flipped_vertically(),
            BoardTransformation::FlippedDiagonallySWNE => {
                board.get_flipped_diagonally_southwest_northeast()
            }
            BoardTransformation::FlippedDiagonallyNWSE => {
                board.get_flipped_diagonally_northwest_southeast()
            }
        };
        for y in 0..3 {
            for x in 0..3 {
                if transformed_board.0[y][x] == FieldState::X {
                    return FieldPosition::new(x, y);
                }
            }
        }
        unreachable!()
    }

    fn transform_pos(&self, pos: FieldPosition) -> FieldPosition {
        let mut board = Board::EMPTY;
        board.0[pos.y][pos.x] = FieldState::X;
        let transformed_board = match self {
            BoardTransformation::None => return pos,
            BoardTransformation::Rotated90CW => board.get_rotated_90_clockwise(),
            BoardTransformation::Rotated180 => {
                board.get_rotated_90_clockwise().get_rotated_90_clockwise()
            }
            BoardTransformation::Rotated90CCW => board
                .get_rotated_90_clockwise()
                .get_rotated_90_clockwise()
                .get_rotated_90_clockwise(),
            BoardTransformation::FlippedHorizontally => board.get_flipped_horizontally(),
            BoardTransformation::FlippedVertically => board.get_flipped_vertically(),
            BoardTransformation::FlippedDiagonallySWNE => {
                board.get_flipped_diagonally_southwest_northeast()
            }
            BoardTransformation::FlippedDiagonallyNWSE => {
                board.get_flipped_diagonally_northwest_southeast()
            }
        };
        for y in 0..3 {
            for x in 0..3 {
                if transformed_board.0[y][x] == FieldState::X {
                    return FieldPosition::new(x, y);
                }
            }
        }
        unreachable!()
    }
}
