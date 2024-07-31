use bevy::{prelude::*, utils::HashSet};
use chrono::Local;

use super::{level_builder::LevelConfig, BallControlDisplay, LevelHistory};

#[derive(PartialEq, Default, Debug)]
pub enum StatusMode {
    #[default]
    Demo,
    Setup,
    Deploying,
    Running,
    Paused,
    Done,
}

#[derive(Resource, Default, Debug)]
pub struct GameStatus {
    pub refreshed_targets: HashSet<u8>,
    pub in_modified_sensitivity: bool,
    pub mode: StatusMode,
    pub control_displays: Vec<BallControlDisplay>,
    pub current_history: LevelHistory,
    pub current_level: LevelConfig,
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

    pub fn require_refresh(&mut self, target: Option<u8>) {
        if let Some(target) = target {
            self.refreshed_targets.remove(&target);
        } else {
            self.refreshed_targets.clear();
        }
    }

    pub fn is_refreshed(&self, target: u8) -> bool {
        self.refreshed_targets.contains(&target)
    }

    pub fn set_refreshed(&mut self, target: u8) {
        self.refreshed_targets.insert(target);
    }
}
