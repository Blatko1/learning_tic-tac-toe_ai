use std::fmt::Display;

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

    pub fn get_empty_fields(&self) -> Vec<BoardField> {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, field)| match field {
                        FieldState::Empty => true,
                        _ => false,
                    })
                    .map(move |(x, _)| BoardField::from_pos(x, y))
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

    pub fn get_flipped_horizontally(&self) -> Self {
        let mut flip = self.clone();
        flip.0.swap(0, 2);
        flip
    }

    pub fn get_flipped_vertically(&self) -> Self {
        let mut flip = self.clone();
        flip.0.iter_mut().for_each(|row| {
            row.swap(0, 2)
        });
        flip
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum FieldState {
    Empty = 0,
    X = 1,
    O = 2,
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
}