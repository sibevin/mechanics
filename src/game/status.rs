use bevy::{prelude::*, utils::HashSet};
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
    pub refreshed_targets: HashSet<String>,
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

    pub fn require_refresh(&mut self, target: Option<&str>) {
        if let Some(target) = target {
            self.refreshed_targets.remove(target);
        } else {
            self.refreshed_targets.clear();
        }
    }

    pub fn is_refreshed(&self, target: &str) -> bool {
        self.refreshed_targets.contains(target)
    }

    pub fn set_refreshed(&mut self, target: &str) {
        self.refreshed_targets.insert(String::from(target));
    }
}
