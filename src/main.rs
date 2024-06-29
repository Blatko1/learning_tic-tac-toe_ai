use agent::Agent;

mod agent;
mod game;
mod board;

fn main() {
    let agent = Agent::new_blank();
    println!("boards: {}", agent.memorized_boards_count())
}
