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
    }

    let matches:u32 = matches_history.len().try_into().unwrap();
    for game_id in 0..matches {

        let game = matches_history.get(&game_id).expect(&format!("Failed to get match id {}", game_id));

        println!("------------------------------------ MATCH {} ---------------------------------------------------------", game_id);
        println!("Total kills: {}", game.total_kills);
        println!("Players Table");
        
        for player in game.players.values() {
                println!("\t{}", player);
        }
        println!("Kills scoreboard");
        let mut vec: Vec<(&&str, &i32)> = game.kills.iter().collect();
        vec.sort_by(|a, b| b.1.cmp(a.1));
        for (player, score) in vec {
                println!("\t{}:{}", player, score);
        }
        println!("Kill by means scoreboard");
        for (death_cause, score) in game.kill_by_means.iter() {
                println!("\t{}:{}", death_cause, score);
        }
        println!("-------------------------------------------------------------------------------------------------------");
}
}

