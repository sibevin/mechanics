use crate::game::GameStatus;
use bevy::{
    ecs::{event::EventReader, system::ResMut},
    window::WindowResized,
};

pub fn refresh_on_resize(
    mut resize_events: EventReader<WindowResized>,
    mut game_status: ResMut<GameStatus>,
) {
    for _event in resize_events.read() {
        game_status.require_refresh(None);
    }
}
