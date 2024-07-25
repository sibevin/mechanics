use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::{
    events::Direction,
    prelude::{NavRequest, NavRequestSystem},
    systems::InputMapping,
};

#[derive(PartialEq, Default)]
pub enum KeyBindingMode {
    #[default]
    Navgation,
    Gaming,
    Keyboard,
}

#[derive(Resource, Default)]
pub struct KeyBindingConfig {
    pub mode: KeyBindingMode,
}

pub struct KeyBindingPlugin;

impl Plugin for KeyBindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindingConfig {
            mode: KeyBindingMode::Navgation,
        })
        .add_systems(Startup, setup_input_mapping)
        .add_systems(Update, handle_key_binding.before(NavRequestSystem));
    }
}

fn setup_input_mapping(mut input_mapping: ResMut<InputMapping>) {
    input_mapping.keyboard_navigation = false;
    input_mapping.key_action = KeyCode::Enter;
    input_mapping.focus_follows_mouse = true;
}

fn handle_key_binding(
    mut requests: EventWriter<NavRequest>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<KeyBindingConfig>,
) {
    move_by_arrow(&mut requests, &input);
    match config.mode {
        KeyBindingMode::Navgation => {
            if input.any_just_pressed([KeyCode::Space]) {
                requests.send(NavRequest::Action);
            }
            if input.any_just_pressed([KeyCode::Delete]) {
                requests.send(NavRequest::Cancel);
            }
            move_by_wsad(&mut requests, &input);
            move_by_kjhl(&mut requests, &input);
        }
        KeyBindingMode::Gaming => {
            if input.any_just_pressed([
                KeyCode::Space,
                KeyCode::Backspace,
                KeyCode::Delete,
                KeyCode::Escape,
            ]) {
                requests.send(NavRequest::Action);
            }
        }
        // NOTE: use default key binding only
        KeyBindingMode::Keyboard => (),
    }
}

fn move_by_arrow(requests: &mut EventWriter<NavRequest>, input: &Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::ArrowUp) {
        requests.send(NavRequest::Move(Direction::North));
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        requests.send(NavRequest::Move(Direction::South));
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        requests.send(NavRequest::Move(Direction::West));
    }
    if input.just_pressed(KeyCode::ArrowRight) {
        requests.send(NavRequest::Move(Direction::East));
    }
}

fn move_by_wsad(requests: &mut EventWriter<NavRequest>, input: &Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyW) {
        requests.send(NavRequest::Move(Direction::North));
    }
    if input.just_pressed(KeyCode::KeyS) {
        requests.send(NavRequest::Move(Direction::South));
    }
    if input.just_pressed(KeyCode::KeyA) {
        requests.send(NavRequest::Move(Direction::West));
    }
    if input.just_pressed(KeyCode::KeyD) {
        requests.send(NavRequest::Move(Direction::East));
    }
}

fn move_by_kjhl(requests: &mut EventWriter<NavRequest>, input: &Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyK) {
        requests.send(NavRequest::Move(Direction::North));
    }
    if input.just_pressed(KeyCode::KeyJ) {
        requests.send(NavRequest::Move(Direction::South));
    }
    if input.just_pressed(KeyCode::KeyH) {
        requests.send(NavRequest::Move(Direction::West));
    }
    if input.just_pressed(KeyCode::KeyL) {
        requests.send(NavRequest::Move(Direction::East));
    }
}
