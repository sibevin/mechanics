use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const MAX_HISTORIES_PER_LEVEL: usize = 9;

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct LevelHistory {
    pub is_clear: bool,
    pub time: u32,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub force: Vec<f32>,
    pub angle: Vec<f32>,
    pub created_at: String,
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct LevelRecord {
    pub is_clear: bool,
    pub best_history: Option<LevelHistory>,
    pub histories: Vec<LevelHistory>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct LeaderboardModel {
    pub level_map: HashMap<String, LevelRecord>,
}

impl LeaderboardModel {
    pub fn open_level(&mut self, level_code: String) {
        if self.level_map.get(&level_code).is_none() {
            self.level_map.insert(
                level_code,
                LevelRecord {
                    is_clear: false,
                    best_history: None,
                    histories: vec![],
                },
            );
        }
    }
    pub fn store_level_history(&mut self, level_code: String, history: LevelHistory) {
        if let Some(record) = self.level_map.get_mut(&level_code) {
            record.histories.push(history.clone());
            if record.histories.len() > MAX_HISTORIES_PER_LEVEL {
                record.histories.pop();
            }
            if history.is_clear {
                if let Some(best_history) = &record.best_history {
                    if best_history.time > history.time {
                        record.best_history = Some(history);
                    }
                } else {
                    record.best_history = Some(history);
                }
                if !record.is_clear {
                    record.is_clear = true;
                }
            }
        }
    }
    pub fn level_info(&self, level_code: String) -> LevelRecord {
        if let Some(record) = self.level_map.get(&level_code) {
            record.clone()
        } else {
            LevelRecord {
                is_clear: false,
                best_history: None,
                histories: vec![],
            }
        }
    }
}
