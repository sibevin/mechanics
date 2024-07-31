use crate::{app::anime_effect, book::page::*, game::*};
use bevy_alt_ui_navigation_lite::{prelude::*, NavRequestSystem};
use bevy_prototype_lyon::path::PathBuilder;
use bevy_prototype_lyon::prelude::*;

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
                (
                    page_update,
                    handle_ui_navigation
                        .after(NavRequestSystem)
                        .run_if(in_state(self.state())),
                ),
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

const LV_CIRCLE_R: f32 = ui::FONT_SIZE * 3.0;
const LV_LINE_W: f32 = ui::FONT_SIZE * 0.2;
const LV_CONTROL_SIZE: f32 = ui::FONT_SIZE / 9.0 * 20.0;
const LV_CONTROL_GAP: f32 = LV_CONTROL_SIZE * 0.2;
const LV_CONTROL_FS: f32 = LV_CONTROL_SIZE * 0.5;
const LV_TITLE_FS: f32 = ui::FONT_SIZE * 2.0;
const LV_NUM_FS: f32 = ui::FONT_SIZE * 1.5;
const LV_SUB_NUM_FS: f32 = LV_NUM_FS * 0.6;
const LV_TITLE_W: f32 = ui::FONT_SIZE * 12.0;
const LV_SHADOW_OFFSET: f32 = LV_LINE_W;
const LV_SHADOW_W: f32 = LV_LINE_W * 0.5;

const GAME_FG_REFRESH_TARGET: &str = "GAME_FG_REFRESH_TARGET";

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<PhaseState>>,
    mut game_status: ResMut<GameStatus>,
) {
    game_state.set(PhaseState::Preparing);
    game_status.mode = StatusMode::Setup;
    game_status.control_displays = vec![
        BallControlDisplay {
            ball_type: Some(BallType::Stone),
            control_type: BallControlType::Angle,
            text: String::from("00.00"),
        },
        BallControlDisplay {
            ball_type: Some(BallType::Stone),
            control_type: BallControlType::Force,
            text: String::from("00.00"),
        },
        BallControlDisplay {
            ball_type: Some(BallType::Goal),
            control_type: BallControlType::Move2D,
            text: String::from("00.00\n00.00"),
        },
        BallControlDisplay {
            ball_type: Some(BallType::Bomb),
            control_type: BallControlType::Move1D,
            text: String::from("00.00"),
        },
    ];
    game_status.require_refresh(Some(ui::REFRESH_GAME_FG));
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
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        position_type: PositionType::Absolute,
                        bottom: ui::px_p(ui::PAGE_PADDING),
                        left: ui::px_p(ui::PAGE_PADDING),
                        column_gap: ui::px_p(ui::PAGE_PADDING),
                        ..default()
                    },
                    ..default()
                },))
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

fn page_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    mut game_status: ResMut<GameStatus>,
    fg_query: Query<Entity, With<GameFg>>,
) {
}

fn page_exit(mut game_state: ResMut<NextState<PhaseState>>, mut game_status: ResMut<GameStatus>) {
    game_state.set(PhaseState::Ready);
    game_status.mode = StatusMode::Demo;
    game_status.require_refresh(Some(ui::REFRESH_GAME_FG));
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
