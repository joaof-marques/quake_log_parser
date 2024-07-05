use crate::games::game_struct::Game;
use std::collections::HashMap;

pub fn action_handler<'a>(
    current_match: &mut Game<'a>,
    log_line: Vec<&'a str>,
    matches_history: &mut HashMap<u32, Game<'a>>,
    configs: &'a str,
) {
    match log_line[1] {
        "ClientUserinfoChanged" => {
            let player_name = get_players_name(configs);
            current_match.insert_update_player_in_match(player_name, log_line[2]);
        }
        "Kill" => {
            current_match.increase_total_kills_count();
            let (killer_id, dead_id, kill_method) = get_kill_informations(&configs);
            current_match.increase_player_kills(killer_id, dead_id);
            current_match.increase_kill_by_means(kill_method);
        }
        "InitGame" => {
            if current_match.total_kills != 0 {
                let id: u32 = matches_history.len().try_into().unwrap();
                matches_history.insert(id, std::mem::replace(current_match, Game::new()));
            }
        }
        "ShutdownGame" => {
            let id: u32 = matches_history.len().try_into().unwrap();
            matches_history.insert(id, std::mem::replace(current_match, Game::new()));
        }
        _ => (),
    }
}

pub fn get_players_name(model_config: &str) -> &str {
    let configs = model_config
        .split("n\\")
        .nth(1)
        .expect("Error retrieving players name. Exit 1");
    let player_name = configs
        .split("\\")
        .nth(0)
        .expect("Error retrieving players name. Exit 2");
    return player_name;
}

fn get_kill_informations(kill_string: &str) -> (u32, u32, String) {
    let separate: Vec<&str> = kill_string.trim().split(" ").collect();
    let killer_id: u32 = separate[2].parse().expect("Failed to parse Killer_id");
    let dead_id: u32 = separate[3].parse().expect("Failed to parse dead_id");
    let kill_method = separate.last().expect("Failed to retrieve Kill method");
    return (killer_id, dead_id, kill_method.to_string());
}
