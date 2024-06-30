use std::fmt::Display;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Board(pub [[FieldState; 3]; 3]);

impl Board {
    pub const EMPTY: Self = Self([[FieldState::Empty; 3]; 3]);

    pub fn play_move_at(&mut self, board_pos: FieldPosition) {
        let x = board_pos.x;
        let y = board_pos.y;
        let cross_count = self.field_state_count(FieldState::X);
        let circle_count = self.field_state_count(FieldState::O);

        let field = &mut self.0[y][x];
        if *field != FieldState::Empty {
            panic!("Something is wrong with AI: to play at: {:?}", board_pos)
        }
        if cross_count == circle_count {
            *field = FieldState::X
        } else {
            *field = FieldState::O
        }
    }

    pub fn field_state_count(&self, field_state: FieldState) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&field| field == &field_state).count()
            })
            .sum()
    }

    pub fn get_empty_fields_pos(&self) -> Vec<FieldPosition> {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, field)| field == &&FieldState::Empty)
                    .map(move |(x, _)| FieldPosition::new(x, y))
            })
            .flatten()
            .collect()
    }

    pub fn get_rotated_90_clockwise(&self) -> Self {
        let mut rotate = self.clone();
        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, &field)| rotate.0[x][2 - y] = field)
        });
        rotate
    }

    pub fn get_flipped_vertically(&self) -> Self {
        let mut flip = self.clone();
        flip.0.swap(0, 2);
        flip
    }

    pub fn get_flipped_horizontally(&self) -> Self {
        let mut flip = self.clone();
        flip.0.iter_mut().for_each(|row| row.swap(0, 2));
        flip
    }

    pub fn get_flipped_diagonally_southwest_northeast(&self) -> Self {
        let mut flip = self.clone();
        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, &field)| flip.0[x][y] = field)
        });
        flip
    }

    pub fn get_flipped_diagonally_northwest_southeast(&self) -> Self {
        let mut flip = self.clone();
        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, &field)| flip.0[2 - x][2 - y] = field)
        });
        flip
    }

    pub fn find_winner(&self) -> FieldState {
        let is_all_same = |row: &[FieldState; 3]| -> bool {
            let first = row[0];
            first != FieldState::Empty && first == row[1] && first == row[2]
        };
        // Check each row for three of a kind
        for y in 0..3 {
            if is_all_same(&self.0[y]) {
                return self.0[y][0]
            }
        }
        // Check each column for three of a kind
        for x in 0..3 {
            if is_all_same(&[self.0[0][x], self.0[1][x], self.0[2][x]]) {
                return self.0[0][x]
            }
        }
        // Check diagonals for three of a kind
        if is_all_same(&[self.0[0][0], self.0[1][1], self.0[2][2]]) {
            return self.0[1][1]
        }
        if is_all_same(&[self.0[2][0], self.0[1][1], self.0[0][2]]) {
            return self.0[1][1]
        }

        FieldState::Empty
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum FieldState {
    Empty = 0,
    X = 1,
    O = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct FieldPosition {
    pub x: usize,
    pub y: usize
}

impl FieldPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BoardFieldPosition {
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

impl BoardFieldPosition {
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

    pub fn to_pos(&self) -> (usize, usize) {
        match self {
            BoardFieldPosition::NorthWest => (0, 0),
            BoardFieldPosition::North => (1, 0),
            BoardFieldPosition::NorthEast => (2, 0),
            BoardFieldPosition::West => (0, 1),
            BoardFieldPosition::Center => (1, 1),
            BoardFieldPosition::East => (2, 1),
            BoardFieldPosition::SouthWest => (0, 2),
            BoardFieldPosition::South => (1, 2),
            BoardFieldPosition::SouthEast => (2, 2),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {} | {} | {}\n",
            &self.0[0][0], &self.0[0][1], &self.0[0][2]
        )?;
        write!(f, "------------\n")?;
        write!(
            f,
            " {} | {} | {}\n",
            &self.0[1][0], &self.0[1][1], &self.0[1][2]
        )?;
        write!(f, "------------\n")?;
        write!(
            f,
            " {} | {} | {}\n\n",
            &self.0[2][0], &self.0[2][1], &self.0[2][2]
        )
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

#[test]
fn board_rotation_test() {
    let board = Board([
        [FieldState::O, FieldState::X, FieldState::Empty],
        [FieldState::Empty, FieldState::Empty, FieldState::Empty],
        [FieldState::Empty, FieldState::X, FieldState::Empty],
    ]);
    let left = board.get_rotated_90_clockwise();
    let right = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::O],
        [FieldState::X, FieldState::Empty, FieldState::X],
        [FieldState::Empty, FieldState::Empty, FieldState::Empty],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );

    let board = Board([
        [FieldState::Empty, FieldState::X, FieldState::X],
        [FieldState::Empty, FieldState::O, FieldState::Empty],
        [FieldState::Empty, FieldState::Empty, FieldState::O],
    ]);
    let left = board.get_rotated_90_clockwise();
    let right = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::Empty],
        [FieldState::Empty, FieldState::O, FieldState::X],
        [FieldState::O, FieldState::Empty, FieldState::X],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );

    let board = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::O],
        [FieldState::X, FieldState::Empty, FieldState::Empty],
        [FieldState::O, FieldState::X, FieldState::Empty],
    ]);
    let left = board.get_rotated_90_clockwise();
    let right = Board([
        [FieldState::O, FieldState::X, FieldState::Empty],
        [FieldState::X, FieldState::Empty, FieldState::Empty],
        [FieldState::Empty, FieldState::Empty, FieldState::O],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );

    let board = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::O],
        [FieldState::X, FieldState::Empty, FieldState::X],
        [FieldState::O, FieldState::X, FieldState::Empty],
    ]);
    let left = board.get_flipped_horizontally();
    let right = Board([
        [FieldState::O, FieldState::X, FieldState::Empty],
        [FieldState::X, FieldState::Empty, FieldState::X],
        [FieldState::Empty, FieldState::Empty, FieldState::O],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );

    let board = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::O],
        [FieldState::O, FieldState::Empty, FieldState::X],
        [FieldState::O, FieldState::X, FieldState::Empty],
    ]);
    let left = board.get_flipped_vertically();
    let right = Board([
        [FieldState::O, FieldState::Empty, FieldState::Empty],
        [FieldState::X, FieldState::Empty, FieldState::O],
        [FieldState::Empty, FieldState::X, FieldState::O],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );

    let board = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::O],
        [FieldState::O, FieldState::Empty, FieldState::X],
        [FieldState::O, FieldState::X, FieldState::Empty],
    ]);
    let left = board.get_flipped_diagonally_southwest_northeast();
    let right = Board([
        [FieldState::Empty, FieldState::O, FieldState::O],
        [FieldState::Empty, FieldState::Empty, FieldState::X],
        [FieldState::O, FieldState::X, FieldState::Empty],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );

    let board = Board([
        [FieldState::Empty, FieldState::Empty, FieldState::O],
        [FieldState::O, FieldState::Empty, FieldState::X],
        [FieldState::O, FieldState::X, FieldState::Empty],
    ]);
    let left = board.get_flipped_diagonally_northwest_southeast();
    let right = Board([
        [FieldState::Empty, FieldState::X, FieldState::O],
        [FieldState::X, FieldState::Empty, FieldState::Empty],
        [FieldState::O, FieldState::O, FieldState::Empty],
    ]);
    assert_eq!(
        left, right,
        "Not matching: left: \n{}\n, right: \n{}\n",
        left, right
    );
}
