use crate::game::GameStatus;
use bevy::{
    ecs::{event::EventReader, system::ResMut},
    window::WindowResized,
};

pub const REFRESH_GAME_BG: u8 = 0;
pub const REFRESH_GAME_FG: u8 = 1;

pub fn refresh_on_resize(
    mut resize_events: EventReader<WindowResized>,
    mut game_status: ResMut<GameStatus>,
) {
    for _event in resize_events.read() {
        game_status.require_refresh(None);
    }
}
