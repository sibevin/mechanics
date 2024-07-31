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
const LV_LINE_W: f32 = ui::FONT_SIZE / 9.0 * 2.0;
const LV_CONTROL_SIZE: f32 = ui::FONT_SIZE / 9.0 * 20.0;
const LV_CONTROL_GAP: f32 = LV_CONTROL_SIZE * 0.2;
const LV_CONTROL_FS: f32 = LV_CONTROL_SIZE * 0.5;
const LV_TITLE_FS: f32 = ui::FONT_SIZE * 2.0;
const LV_NUM_FS: f32 = ui::FONT_SIZE * 1.5;
const LV_SUB_NUM_FS: f32 = LV_NUM_FS * 0.6;
const LV_TITLE_W: f32 = ui::FONT_SIZE * 12.0;
const LV_SHADOW_OFFSET: f32 = LV_LINE_W;
const LV_SHADOW_W: f32 = LV_LINE_W * 0.3;

const GAME_FG_REFRESH_TARGET: &str = "GAME_FG_REFRESH_TARGET";

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<PhaseState>>,
    mut game_status: ResMut<GameStatus>,
) {
    game_state.set(PhaseState::Preparing);
    game_status.require_refresh(Some(GAME_FG_REFRESH_TARGET));
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
    if !game_status.is_refreshed(GAME_FG_REFRESH_TARGET) {
        let fg_entity = fg_query.get_single().unwrap();
        let mut entity_commands = commands.get_entity(fg_entity).unwrap();
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            draw_game_fg(parent, &window_query, &asset_server);
        });
        game_status.set_refreshed(GAME_FG_REFRESH_TARGET);
    }
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

fn draw_game_fg(
    parent: &mut ChildBuilder,
    window_query: &Query<&Window>,
    asset_server: &Res<AssetServer>,
) {
    let window = window_query.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    let lv_x_base = -win_w / 2.0 + LV_CONTROL_GAP + LV_CONTROL_SIZE / 2.0;
    let lv_y_base = win_h / 2.0 - LV_CONTROL_GAP - LV_CONTROL_SIZE / 2.0;
    let lv_z = layer::GAME_COVER_Z_INDEX;
    let shadow_color = theme::MUTE_COLOR.with_alpha(0.3);

    parent
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(lv_x_base, lv_y_base, layer::GAME_COVER_Z_INDEX),
                ..default()
            },
            OnPage,
        ))
        .with_children(|parent| {
            let lv_x_font_base = LV_CONTROL_GAP * 2.0 + LV_CONTROL_SIZE;
            let mut lv_y = 0.0;
            let shape = shapes::Circle {
                radius: LV_CIRCLE_R,
                ..default()
            };
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(theme::BG_COLOR),
                Stroke::new(theme::MUTE_COLOR, LV_LINE_W * 1.5),
            ));
            let lv_num_offset = Vec2::new(LV_NUM_FS * 0.2, LV_NUM_FS * 0.3);
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    "000",
                    TextStyle {
                        font: asset_server.load(theme::FONT_DIGIT),
                        font_size: LV_NUM_FS,
                        color: theme::FG_COLOR,
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(lv_num_offset.x, lv_num_offset.y, lv_z + 0.003),
                ..default()
            },));
            let mut line_builder = PathBuilder::new();
            let lv_num_line_y = lv_num_offset.y - LV_NUM_FS * 0.7;
            line_builder.move_to(Vec2::new(-40.0, lv_num_line_y));
            line_builder.line_to(Vec2::new(60.0, lv_num_line_y));
            parent.spawn((
                ShapeBundle {
                    path: line_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.002),
                        ..default()
                    },
                    ..default()
                },
                Stroke {
                    color: shadow_color,
                    options: StrokeOptions::DEFAULT
                        .with_line_width(LV_LINE_W * 0.8)
                        .with_line_cap(LineCap::Round),
                },
            ));
            let lv_num_offset = Vec2::new(0.0, -LV_NUM_FS);
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    "ABCD",
                    TextStyle {
                        font: asset_server.load(theme::FONT_DIGIT),
                        font_size: LV_SUB_NUM_FS,
                        color: theme::FG_COLOR,
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(lv_num_offset.x, lv_num_offset.y, lv_z + 0.003),
                ..default()
            },));
            lv_y = lv_y - LV_CONTROL_GAP * 6.0;
            lv_y = lv_y - LV_CONTROL_SIZE - LV_CONTROL_GAP;
            let icon_path = format!("images/game/controls/stone_angle.png");
            let icon = asset_server.load(icon_path);
            parent.spawn((SpriteBundle {
                texture: icon,
                transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
                ..default()
            },));
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    "00.00",
                    TextStyle {
                        font: asset_server.load(theme::FONT_DIGIT),
                        font_size: LV_CONTROL_FS,
                        color: theme::SECONDARY_COLOR,
                    },
                )
                .with_justify(JustifyText::Left),
                transform: Transform::from_xyz(lv_x_font_base, lv_y, lv_z + 0.001),
                ..default()
            },));
            let mut line_builder = PathBuilder::new();
            let text_shadow_y = lv_y - LV_CONTROL_FS / 2.0 - LV_SHADOW_OFFSET;
            line_builder.move_to(Vec2::new(0.0, text_shadow_y));
            line_builder.line_to(Vec2::new(200.0, text_shadow_y));
            parent.spawn((
                ShapeBundle {
                    path: line_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.000),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::MUTE_COLOR, LV_LINE_W * 0.3),
            ));
            lv_y = lv_y - LV_CONTROL_SIZE - LV_CONTROL_GAP;
            let icon_path = format!("images/game/controls/stone_move_2d.png");
            let icon = asset_server.load(icon_path);
            parent.spawn(SpriteBundle {
                texture: icon,
                transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
                ..default()
            });
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    "00.00",
                    TextStyle {
                        font: asset_server.load(theme::FONT_DIGIT),
                        font_size: LV_CONTROL_FS,
                        color: theme::SECONDARY_COLOR,
                    },
                )
                .with_justify(JustifyText::Left),
                transform: Transform::from_xyz(lv_x_font_base, lv_y, lv_z + 0.001),
                ..default()
            },));
            let mut line_builder = PathBuilder::new();
            let text_shadow_y = lv_y - LV_CONTROL_FS / 2.0 - LV_SHADOW_OFFSET;
            line_builder.move_to(Vec2::new(0.0, text_shadow_y));
            line_builder.line_to(Vec2::new(200.0, text_shadow_y));
            parent.spawn((
                ShapeBundle {
                    path: line_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.000),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::MUTE_COLOR, LV_LINE_W * 0.3),
            ));
            // lv_y = lv_y - LV_CONTROL_SIZE - LV_CONTROL_GAP;
            // let icon_path = format!("images/game/controls/empty.png");
            // let icon = asset_server.load(icon_path);
            // parent.spawn(SpriteBundle {
            //     texture: icon,
            //     transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
            //     ..default()
            // });
            // lv_y = lv_y - LV_CONTROL_SIZE - LV_CONTROL_GAP;
            // let icon_path = format!("images/game/controls/empty.png");
            // let icon = asset_server.load(icon_path);
            // parent.spawn(SpriteBundle {
            //     texture: icon,
            //     transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
            //     ..default()
            // });
            // lv_y = lv_y - LV_CONTROL_SIZE - LV_CONTROL_GAP;
            // let icon_path = format!("images/game/controls/empty.png");
            // let icon = asset_server.load(icon_path);
            // parent.spawn(SpriteBundle {
            //     texture: icon,
            //     transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
            //     ..default()
            // });
            let mut line_builder = PathBuilder::new();
            line_builder.move_to(Vec2::new(0.0, win_h));
            line_builder.line_to(Vec2::new(0.0, lv_y));
            line_builder.move_to(Vec2::new(-win_w, 0.0));
            line_builder.line_to(Vec2::new(win_w, 0.0));
            parent.spawn((
                ShapeBundle {
                    path: line_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.000),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::MUTE_COLOR, LV_LINE_W),
            ));
            let shape = shapes::Circle {
                radius: LV_CIRCLE_R + LV_SHADOW_OFFSET * 1.2,
                ..default()
            };
            let mut line_builder = PathBuilder::new();
            line_builder.move_to(Vec2::new(0.0, win_h));
            line_builder.line_to(Vec2::new(0.0, lv_y));
            line_builder.move_to(Vec2::new(-win_w, 0.0));
            line_builder.line_to(Vec2::new(win_w, 0.0));
            parent.spawn((
                ShapeBundle {
                    path: line_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(
                            LV_SHADOW_OFFSET,
                            -LV_SHADOW_OFFSET,
                            lv_z + 0.000,
                        ),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(shadow_color, LV_SHADOW_W),
            ));
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z - 0.001),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(shadow_color, LV_SHADOW_W),
            ));
        });
    parent
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, lv_y_base, layer::GAME_COVER_Z_INDEX),
                ..default()
            },
            OnPage,
        ))
        .with_children(|parent| {
            let title_h = LV_CONTROL_SIZE * 0.8;
            let shape = shapes::Rectangle {
                extents: Vec2::new(LV_TITLE_W, title_h),
                ..default()
            };
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.001),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(theme::BG_COLOR),
            ));
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    "BASIC",
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: LV_TITLE_FS,
                        color: theme::FG_COLOR,
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.002),
                ..default()
            },));
            let mut line_builder = PathBuilder::new();
            line_builder.move_to(Vec2::new(-LV_TITLE_W / 2.0, title_h / 2.0));
            line_builder.line_to(Vec2::new(-LV_TITLE_W / 2.0, -title_h / 2.0));
            line_builder.move_to(Vec2::new(LV_TITLE_W / 2.0, title_h / 2.0));
            line_builder.line_to(Vec2::new(LV_TITLE_W / 2.0, -title_h / 2.0));
            parent.spawn((
                ShapeBundle {
                    path: line_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.003),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::MUTE_COLOR, LV_LINE_W * 0.8),
            ));
        });
}
