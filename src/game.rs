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
    board: [[FieldState; 3]; 3]
}

impl GameState {

}

#[derive(Debug, Hash)]
pub struct Board (pub [[FieldState; 3]; 3]);

#[derive(Debug, Hash, Clone, Copy)]
pub enum FieldState {
    Empty = 0,
    Cross = 1,
    Circle = 2,
}