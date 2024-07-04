use std::collections::HashMap;

pub struct Game<'a> {
    total_kills: u32,
    pub players: HashMap<u32, &'a str>,
    pub kills: HashMap<&'a str, u32>,
    kill_by_means: HashMap<&'a str, u32>,
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

    pub fn insert_update_player_in_match(&mut self, player_name: &'a str, player_id: u32) {
        self.players.insert(player_id, player_name);
    }

    pub fn increase_player_kills(&mut self, player_name: &'a str) {
        if let Some(player_kills) = self.kills.get_mut(&player_name) {
            *player_kills += 1;
        } else {
            self.kills.insert(player_name, 1);
        }
    }

    pub fn increase_kill_by_means(&mut self, player_name: &'a str) {
        if let Some(kill_method) = self.kill_by_means.get_mut(&player_name) {
            *kill_method += 1;
        } else {
            self.kills.insert(player_name, 1);
        }
    }
}
