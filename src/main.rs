use agent::{Agent, AgentAction};
use board::Board;
use game::{GameEvent, GameState};
use hashbrown::HashMap;

mod agent;
mod game;
mod board;

fn main() {
    println!("boards memorized: {}", Agent::new_blank(0.0).memorized_boards_count());

    let game = GameState::new();

    let mut agent_cross = Agent::new_blank(0.2);
    let mut agent_circle = Agent::new_blank(0.2 );
    let mut agent_cross_moves = Vec::new();
    let mut agent_circle_moves = Vec::new();
    game.run_new(|event, board_state| {
        match event {
            GameEvent::CrossWon => println!("X WON!!!"),
            GameEvent::CircleWon => println!("O WON!!!"),
            GameEvent::CrossTurn => {
                let position = agent_cross.play_greedy_exploration(board_state);
                agent_cross_moves.push((board_state.clone(), position));
                println!("Cross plays: {:?}", position);
                board_state.play_move_at(position);
            },
            GameEvent::CircleTurn => {
                let position = agent_circle.play_greedy_exploration(board_state);
                agent_circle_moves.push((board_state.clone(), position));
                println!("Circle plays: {:?}", position);
                board_state.play_move_at(position);
            },
            GameEvent::InvalidBoard => unreachable!(),
        }
        println!("Current: \n{}\n\n", board_state)
    })
}
