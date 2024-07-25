use super::*;

mod game_over;
mod paused;
mod preparing;
mod ready;
mod running;
mod setup;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PhaseState {
    #[default]
    Setup,
    Ready,
    Preparing,
    Running,
    Paused,
    GameOver,
}

pub trait PhaseBase {
    fn state(&self) -> PhaseState;
    fn build(&self, app: &mut App);
}

pub const PHASES: [&dyn PhaseBase; 6] = [
    &setup::Phase,
    &ready::Phase,
    &preparing::Phase,
    &running::Phase,
    &paused::Phase,
    &game_over::Phase,
];
