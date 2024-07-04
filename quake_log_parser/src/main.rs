use std::fs;
use std::collections::HashMap;
use std::thread::current;

mod actions;
mod games;

use actions::action_handler;
use games::Game;


fn main() {

    let mut matches_control: HashMap<&str, Game> = HashMap::new();
    let mut current_match = Game::new();

    let file_path = "qgames.log"; 
    let mut result: Vec<Vec<&str>> = Vec::new();
    let content = fs::read_to_string(file_path).expect("Failed to read the file");
    
    for line in content.lines(){
        let mut words: Vec<&str> = line.trim().split(' ').collect();
        words[1] = words[1].trim_end_matches(":");
        result.push(words);
    }
    

    for log in result{
        action_handler(log[1]);
    }
}

fn action_handler(action: &str) {
    match action {
        "ClientConnect" => actions::client_actions::print_client(action),
        "ClientUserinfoChanged" => actions::client_actions::print_client(action),
        "ClientBegin" => actions::client_actions::print_client(action),
        "Kill" => actions::client_actions::print_client(action),
        "InitGame" => actions::match_actions::print_match(action),
        "ShutdownGame" => actions::match_actions::print_match(action),
        _ => (),
    }
}