use crate::{app::anime_effect, book::page::*, game::*};
use bevy_alt_ui_navigation_lite::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "game";
const PAGE_NAME: &str = "START";
const PAGE_ICON: &str = "play-light";

pub struct Page;
impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::Game
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(self.state())),
            )
            .add_systems(
                OnExit(self.state()),
                (
                    anime_effect::clear_anime_effect,
                    ui::despawn_ui::<OnPage>,
                    page_exit,
                ),
            );
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    MoveToPage(PageState),
}

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<PhaseState>>,
) {
    game_state.set(PhaseState::Preparing);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnPage,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            position_type: PositionType::Absolute,
                            bottom: ui::px_p(ui::PAGE_PADDING),
                            left: ui::px_p(ui::PAGE_PADDING),
                            column_gap: ui::px_p(ui::PAGE_PADDING),
                            ..default()
                        },
                        ..default()
                    },
                    OnPage,
                ))
                .with_children(|parent| {
                    ui::build_icon_btn(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::MoveToPage(PageState::Menu),
                            app::interaction::IaButton,
                            Focusable::new().prioritized(),
                        ),
                        Style::default(),
                        "arrow-left-light_1.5x",
                    );
                    ui::build_icon_btn(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::MoveToPage(PageState::Level),
                            app::interaction::IaButton,
                            Focusable::new().prioritized(),
                        ),
                        Style::default(),
                        "circles-four-light",
                    );
                });
        });
}

fn page_exit(mut game_state: ResMut<NextState<PhaseState>>) {
    game_state.set(PhaseState::Ready);
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::MoveToPage(state) => page_state.set(*state),
        },
    );
}
