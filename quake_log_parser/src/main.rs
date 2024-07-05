use std::collections::HashMap;
use std::fs;

mod utils;
mod games;

use utils::utils::action_handler;
use games::game_struct::Game;

fn main() {
    let mut matches_history: HashMap<u32, Game> = HashMap::new();
    let mut current_match = Game::new();

    let file_path = "qgames.log";
    let content = fs::read_to_string(file_path).expect("Failed to read the file");

    for line in content.lines() {
        let mut log_line: Vec<&str> = line.trim().split(" ").collect();
        log_line[1] = log_line[1].trim_end_matches(":");

        action_handler(&mut current_match, log_line, &mut matches_history, line);
        // result.push(actions);
    }

}

