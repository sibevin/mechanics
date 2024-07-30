use bevy::prelude::*;
use chrono::Local;

use super::LevelHistory;

#[derive(PartialEq, Default, Debug)]
pub enum StatusMode {
    #[default]
    Setup,
    Deploying,
    Running,
    Paused,
    Done,
}

#[derive(Resource, Default, Debug)]
pub struct GameStatus {
    pub is_refresh_required: bool,
    pub in_modified_sensitivity: bool,
    pub mode: StatusMode,
    pub current_history: LevelHistory,
}

impl GameStatus {
    pub fn reset_history(&mut self) {
        self.current_history = LevelHistory::default()
    }

    pub fn tick(&mut self) {
        self.current_history.time += 1;
    }

    pub fn sumbit_history(&mut self, is_clear: bool) {
        self.current_history.is_clear = is_clear;
        self.current_history.created_at = Local::now().format("%Y-%m-%d_%H:%M:%S%.9f").to_string();
    }
}
