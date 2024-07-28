use super::*;
use crate::{app, book::PageState};
use bevy_persistent::prelude::*;

pub struct Phase;

impl PhaseBase for Phase {
    fn state(&self) -> PhaseState {
        PhaseState::Setup
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), state_enter)
            .add_systems(Update, state_update.run_if(in_state(self.state())));
    }
}

fn state_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
    mut window_query: Query<&mut Window>,
    mut game_status: ResMut<GameStatus>,
) {
    // app startup
    app::startup(&mut commands, &asset_server, &settings, &mut window_query);

    // game startup
    startup(&mut commands, &mut game_status);
}

fn state_update(
    mut game_phase: ResMut<NextState<PhaseState>>,
    mut book_page: ResMut<NextState<PageState>>,
) {
    game_phase.set(PhaseState::Ready);
    // TODO: Test
    // book_page.set(PageState::Menu);
    book_page.set(PageState::Game);
}
