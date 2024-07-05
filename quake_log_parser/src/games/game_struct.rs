use std::collections::HashMap;

pub struct Game<'a> {
    pub total_kills: u32,
    pub players: HashMap<u32, &'a str>,
    pub kills: HashMap<&'a str, i32>,
    pub kill_by_means: HashMap<String, u32>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        Game {
            total_kills: 0,
            players: HashMap::new(),
            kills: HashMap::new(),
            kill_by_means: HashMap::new(),
        }
    }

    pub fn get_total_kills_count(&self) -> u32 {
        self.total_kills
    }

    pub fn increase_total_kills_count(&mut self) {
        self.total_kills += 1;
    }

    pub fn insert_update_player_in_match(&mut self, player_name: &'a str, player_id: &'a str) {
        if let Ok(parsed_id) = player_id.parse::<u32>() {
            self.players.insert(parsed_id, player_name);
        } else {
            // Opcional: trate o erro de conversão aqui
            eprintln!("Failed to parse player_id: {}", player_id);
        }
    }

    pub fn increase_player_kills(&mut self, killer_id: u32, dead_id: u32) {      
        
        if killer_id == 1022 {
            let deads_name = self
                .players
                .get(&dead_id)
                .expect("Failed to retrieve Dead's name.");
            if let Some(player_kills) = self.kills.get_mut(*deads_name) {
                *player_kills -= 1;
            } else {
                self.kills.insert(deads_name, -1);
            }
        } else {
            let killer_name = self
            .players
            .get(&killer_id)
            .expect("Failed to retrieve Killer Name.");
            if let Some(player_kills) = self.kills.get_mut(*killer_name) {
                *player_kills += 1;
            } else {
                self.kills.insert(killer_name, 1);
            }
        }
    }

    pub fn increase_kill_by_means(&mut self, kill_method: String) {
        if let Some(weapon_kill_count) = self.kill_by_means.get_mut(&kill_method) {
            *weapon_kill_count += 1;
        } else {
            self.kill_by_means.insert(kill_method, 1);
        }
    }
}
