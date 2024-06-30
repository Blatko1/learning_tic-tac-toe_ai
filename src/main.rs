use agent::{Agent};
use board::Board;
use game::{GameEvent, GameState};
use hashbrown::HashMap;

mod agent;
mod board;
mod game;

fn main() {
    println!(
        "boards memorized: {}",
        Agent::new_blank(0.0).memorized_boards_count()
    );

    let game = GameState::new();

    let mut agent_cross = Agent::new_blank(0.6);
    let mut agent_circle = Agent::new_blank(0.6);
    let mut cross_wins = 0;
    let mut circle_wins = 0;
    let mut draws = 0;
    for _ in 0..100000 {
        game.run_new(|event, board_state| {
            match event {
                GameEvent::CrossWon => {
                    agent_cross.give_feedback(6);
                    agent_circle.give_feedback(-3);
                    cross_wins += 1;
                }
                GameEvent::CircleWon => {
                    agent_cross.give_feedback(-3);
                    agent_circle.give_feedback(6);
                    circle_wins += 1;
                }
                GameEvent::Draw => {
                    agent_cross.give_feedback(1);
                    agent_circle.give_feedback(1);
                    draws += 1;
                    
                }
                GameEvent::CrossTurn => {
                    let position = agent_cross.play_greedy_exploration(board_state);
                    board_state.play_move_at(position.unwrap());
                }
                GameEvent::CircleTurn => {
                    let position = agent_circle.play_greedy_exploration(board_state);
                    board_state.play_move_at(position.unwrap());
                }
                GameEvent::InvalidBoard => unreachable!(),
            }
        })
    }

    loop {
        game.run_new(|event, board_state| {
            match event {
                GameEvent::CrossWon => {
                    println!("X WON!!!\n");
                    agent_cross.give_feedback(6);
                    agent_circle.give_feedback(-3);
                    cross_wins += 1;
                    
                    println!("X wins: {}\nO wins: {}\nDraws: {}\n", cross_wins, circle_wins, draws);
                }
                GameEvent::CircleWon => {
                    println!("O WON!!!\n");
                    agent_cross.give_feedback(-3);
                    agent_circle.give_feedback(6);
                    circle_wins += 1;
                    
                    println!("X wins: {}\nO wins: {}\nDraws: {}\n", cross_wins, circle_wins, draws);
                }
                GameEvent::Draw => {
                    println!("It's a draw!\n");
                    agent_cross.give_feedback(1);
                    agent_circle.give_feedback(1);
                    draws += 1;
                    
                    println!("X wins: {}\nO wins: {}\nDraws: {}\n", cross_wins, circle_wins, draws);
                }
                GameEvent::CrossTurn => {
                    let position = agent_cross.play_greedy_exploration(board_state);
                    println!("Cross plays: {:?}", position);
                    if let Some(actions) = agent_cross.get_actions_from_board(board_state) {
                        for action in actions {
                            print!("POS: ({}, {}), BIAS: {}\n", action.field_pos.x, action.field_pos.y, action.bias)
                        }
                        println!("Memorized Boards: {}, epsilon: {}", agent_cross.memorized_boards_count(), agent_cross.epsilon());
                    }
                    board_state.play_move_at(position.unwrap());
                }
                GameEvent::CircleTurn => {
                    let position = agent_circle.play_greedy_exploration(board_state);
                    println!("Circle plays: {:?}", position);
                    if let Some(actions) = agent_circle.get_actions_from_board(board_state) {
                        for action in actions {
                            print!("POS: ({}, {}), BIAS: {}\n", action.field_pos.x, action.field_pos.y, action.bias)
                        }
                        println!("Memorized Boards: {}, epsilon: {}", agent_circle.memorized_boards_count(), agent_cross.epsilon());
                    }
                    board_state.play_move_at(position.unwrap());
                }
                GameEvent::InvalidBoard => unreachable!(),
            }
            println!("Current: \n{}\n\n", board_state);
            std::io::stdin().read_line(&mut String::new()).unwrap();
        })
    }
}
