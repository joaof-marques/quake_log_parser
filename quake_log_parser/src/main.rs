use std::fs;
use std::collections::HashMap;
use std::thread::current;

mod actions;
mod games;

use actions::action_handler;
use games::Game;


fn main() {

    let mut matches_control: HashMap<&str, Game> = HashMap::new();

    let file_path = "qgames.log";
    
    let mut current_match = Game::new();

    let mut result: Vec<Vec<&str>> = Vec::new();

    let content = fs::read_to_string(file_path).expect("Failed to read the file");
    
    for line in content.lines(){
        let mut words: Vec<&str> = line.trim().split(' ').collect();
        words[1] = words[1].trim_end_matches(":");
        result.push(words);
    }
    

    for log in result{
        continue;
    }
}

