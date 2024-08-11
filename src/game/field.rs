use super::*;
use crate::app::{theme, ui};
use bevy_prototype_lyon::prelude::*;

pub const FIELD_LINE_W: f32 = ui::SPACE_SIZE * 0.5;
pub const FIELD_COLOR: Color = theme::BG_COLOR;
pub const FIELD_TEXT_COLOR: Color = theme::FG_COLOR;
pub const FIELD_SB_TEXT_COLOR: Color = theme::SECONDARY_COLOR;

pub fn refresh_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bg_query: Query<Entity, With<GameBg>>,
    fg_query: Query<Entity, With<GameFg>>,
    window_query: Query<&Window>,
    mut game_status: ResMut<GameStatus>,
) {
    if !game_status.is_refreshed(ui::REFRESH_GAME_BG) {
        let bg_entity = bg_query.get_single().unwrap();
        let mut entity_commands = commands.get_entity(bg_entity).unwrap();
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            draw_grid_bg(parent, &window_query);
        });
        game_status.set_refreshed(ui::REFRESH_GAME_BG)
    }
    if !game_status.is_refreshed(ui::REFRESH_GAME_FG) {
        let fg_entity = fg_query.get_single().unwrap();
        let mut entity_commands = commands.get_entity(fg_entity).unwrap();
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            draw_game_fg(parent, &window_query, &asset_server, &game_status);
        });
        game_status.set_refreshed(ui::REFRESH_GAME_FG);
    }
}

const FIELD_GRID_SIZE: f32 = ui::FONT_SIZE * 0.5;

fn draw_grid_bg(parent: &mut ChildBuilder, window_query: &Query<&Window>) {
    let window = window_query.get_single().unwrap();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    let mut grid_l_builder = PathBuilder::new();
    grid_l_builder.move_to(Vec2::new(-win_w * 0.5, 0.0));
    grid_l_builder.line_to(Vec2::new(win_w * 0.5, 0.0));
    grid_l_builder.move_to(Vec2::new(0.0, -win_h * 0.5));
    grid_l_builder.line_to(Vec2::new(0.0, win_h * 0.5));
    let mut grid_s_builder = PathBuilder::new();
    let y_line_count = (win_h * 0.5 / FIELD_GRID_SIZE).ceil() as u8;
    for i in 1..=y_line_count {
        if i % 5 == 0 {
            grid_l_builder.move_to(Vec2::new(-win_w * 0.5, 0.0 + i as f32 * FIELD_GRID_SIZE));
            grid_l_builder.line_to(Vec2::new(win_w * 0.5, 0.0 + i as f32 * FIELD_GRID_SIZE));
            grid_l_builder.move_to(Vec2::new(-win_w * 0.5, 0.0 - i as f32 * FIELD_GRID_SIZE));
            grid_l_builder.line_to(Vec2::new(win_w * 0.5, 0.0 - i as f32 * FIELD_GRID_SIZE));
        } else {
            grid_s_builder.move_to(Vec2::new(-win_w * 0.5, 0.0 + i as f32 * FIELD_GRID_SIZE));
            grid_s_builder.line_to(Vec2::new(win_w * 0.5, 0.0 + i as f32 * FIELD_GRID_SIZE));
            grid_s_builder.move_to(Vec2::new(-win_w * 0.5, 0.0 - i as f32 * FIELD_GRID_SIZE));
            grid_s_builder.line_to(Vec2::new(win_w * 0.5, 0.0 - i as f32 * FIELD_GRID_SIZE));
        }
    }
    let x_line_count = (win_w * 0.5 / FIELD_GRID_SIZE).ceil() as u8;
    for i in 1..=x_line_count {
        if i % 5 == 0 {
            grid_l_builder.move_to(Vec2::new(0.0 + i as f32 * FIELD_GRID_SIZE, -win_h * 0.5));
            grid_l_builder.line_to(Vec2::new(0.0 + i as f32 * FIELD_GRID_SIZE, win_h * 0.5));
            grid_l_builder.move_to(Vec2::new(0.0 - i as f32 * FIELD_GRID_SIZE, -win_h * 0.5));
            grid_l_builder.line_to(Vec2::new(0.0 - i as f32 * FIELD_GRID_SIZE, win_h * 0.5));
        } else {
            grid_s_builder.move_to(Vec2::new(0.0 + i as f32 * FIELD_GRID_SIZE, -win_h * 0.5));
            grid_s_builder.line_to(Vec2::new(0.0 + i as f32 * FIELD_GRID_SIZE, win_h * 0.5));
            grid_s_builder.move_to(Vec2::new(0.0 - i as f32 * FIELD_GRID_SIZE, -win_h * 0.5));
            grid_s_builder.line_to(Vec2::new(0.0 - i as f32 * FIELD_GRID_SIZE, win_h * 0.5));
        }
    }
    parent.spawn((
        ShapeBundle {
            path: grid_s_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.001),
                ..default()
            },
            ..default()
        },
        Stroke::new(theme::MUTE_COLOR, FIELD_LINE_W),
    ));
    parent.spawn((
        ShapeBundle {
            path: grid_l_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.002),
                ..default()
            },
            ..default()
        },
        Stroke::new(theme::MUTE_COLOR, FIELD_LINE_W * 2.0),
    ));
    let circle = shapes::Circle {
        radius: FIELD_GRID_SIZE * 20.0,
        center: Vec2::new(0.0, 0.0),
    };
    let circle_2 = shapes::Circle {
        radius: FIELD_GRID_SIZE * 30.0,
        center: Vec2::new(0.0, 0.0),
    };
    let circle_3 = shapes::Circle {
        radius: FIELD_GRID_SIZE * 40.0,
        center: Vec2::new(0.0, 0.0),
    };
    let circle_builder = GeometryBuilder::new()
        .add(&circle)
        .add(&circle_2)
        .add(&circle_3);
    parent.spawn((
        ShapeBundle {
            path: circle_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.003),
                ..default()
            },
            ..default()
        },
        Stroke::new(theme::MUTE_COLOR, FIELD_LINE_W),
    ));
    let rect = shapes::Rectangle {
        extents: Vec2::new(win_w, win_h),
        ..default()
    };
    let cover_builder = GeometryBuilder::new().add(&rect);
    parent.spawn((
        ShapeBundle {
            path: cover_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.004),
                ..default()
            },
            ..default()
        },
        Fill::color(theme::BG_COLOR.with_alpha(0.9)),
    ));
}

const LV_CIRCLE_R: f32 = ui::FONT_SIZE * 3.0;
const LV_LINE_W: f32 = ui::FONT_SIZE * 0.2;
const LV_CONTROL_SIZE: f32 = ui::FONT_SIZE / 9.0 * 20.0;
const LV_CONTROL_GAP: f32 = LV_CONTROL_SIZE * 0.2;
const LV_CONTROL_FS: f32 = LV_CONTROL_SIZE * 0.3;
const LV_TITLE_FS: f32 = ui::FONT_SIZE * 2.0;
const LV_NUM_FS: f32 = ui::FONT_SIZE * 1.5;
const LV_SUB_NUM_FS: f32 = LV_NUM_FS * 0.6;
const LV_TITLE_W: f32 = ui::FONT_SIZE * 12.0;
const LV_SHADOW_OFFSET: f32 = LV_LINE_W;
const LV_SHADOW_W: f32 = LV_LINE_W * 0.5;

fn draw_game_fg(
    parent: &mut ChildBuilder,
    window_query: &Query<&Window>,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
) {
    let window = window_query.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    let lv_x_base = -win_w / 2.0 + LV_CONTROL_GAP + LV_CONTROL_SIZE / 2.0;
    let lv_y_base = win_h / 2.0 - LV_CONTROL_GAP - LV_CONTROL_SIZE / 2.0;
    let lv_z = 0.01;
    let shadow_color = theme::MUTE_COLOR.with_alpha(0.3);
    let main_line_color: Color;
    let main_text_color: Color;
    if game_status.mode == StatusMode::Demo {
        main_line_color = shadow_color;
        main_text_color = shadow_color
    } else {
        main_line_color = theme::MUTE_COLOR;
        main_text_color = theme::FG_COLOR;
    };
    parent
        .spawn((SpatialBundle {
            transform: Transform::from_xyz(lv_x_base, lv_y_base, lv_z),
            ..default()
        },))
        .with_children(|parent| {
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
                Stroke::new(main_line_color, LV_LINE_W * 1.5),
            ));
            let lv_num_offset = Vec2::new(LV_NUM_FS * 0.2, LV_NUM_FS * 0.3);
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    "000",
                    TextStyle {
                        font: asset_server.load(theme::FONT_DIGIT),
                        font_size: LV_NUM_FS,
                        color: main_text_color,
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
                        color: main_text_color,
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(lv_num_offset.x, lv_num_offset.y, lv_z + 0.003),
                ..default()
            },));
            lv_y = lv_y - LV_CONTROL_GAP * 6.0;
            let controls: &Vec<BallControlDisplay> = if game_status.mode == StatusMode::Demo {
                &vec![
                    BallControlDisplay {
                        ball_type: None,
                        control_type: BallControlType::Angle,
                        text: String::from("00.00"),
                    },
                    BallControlDisplay {
                        ball_type: None,
                        control_type: BallControlType::Force,
                        text: String::from("00.00"),
                    },
                    BallControlDisplay {
                        ball_type: None,
                        control_type: BallControlType::Move2D,
                        text: String::from("00.00\n00.00"),
                    },
                    BallControlDisplay {
                        ball_type: None,
                        control_type: BallControlType::Move1D,
                        text: String::from("00.00"),
                    },
                ]
            } else {
                &game_status.control_displays
            };
            let lv_x_font_base = LV_CONTROL_GAP * 2.0 + LV_CONTROL_SIZE * 0.8;
            for control in controls.iter() {
                let ball_type_str: String;
                let control_num_color: Color;
                if let Some(ball_type) = &control.ball_type {
                    ball_type_str = ball_type.to_string();
                    control_num_color = ball_type.color();
                } else {
                    ball_type_str = String::from("demo");
                    control_num_color = shadow_color;
                };
                lv_y = lv_y - LV_CONTROL_SIZE - LV_CONTROL_GAP;
                let icon_path = format!(
                    "images/game/controls/{}_{}.png",
                    ball_type_str,
                    control.control_type.to_string()
                );
                let icon = asset_server.load(icon_path);
                parent.spawn((SpriteBundle {
                    texture: icon,
                    transform: Transform::from_xyz(0.0, lv_y, lv_z + 0.001),
                    ..default()
                },));
                parent.spawn((Text2dBundle {
                    text: Text::from_section(
                        control.text.clone(),
                        TextStyle {
                            font: asset_server.load(theme::FONT_DIGIT),
                            font_size: LV_CONTROL_FS,
                            color: control_num_color,
                        },
                    )
                    .with_justify(JustifyText::Left),
                    transform: Transform::from_xyz(lv_x_font_base, lv_y, lv_z + 0.001),
                    ..default()
                },));
                let text_shadow_x = if control.control_type == BallControlType::Move2D {
                    ui::FONT_SIZE * 1.4
                } else {
                    ui::FONT_SIZE * 4.0
                };
                let mut line_builder = PathBuilder::new();
                let text_shadow_y = lv_y - LV_CONTROL_FS / 2.0 - LV_SHADOW_OFFSET;
                line_builder.move_to(Vec2::new(0.0, text_shadow_y));
                line_builder.line_to(Vec2::new(text_shadow_x, text_shadow_y));
                parent.spawn((
                    ShapeBundle {
                        path: line_builder.build(),
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.000),
                            ..default()
                        },
                        ..default()
                    },
                    Stroke::new(main_line_color, LV_LINE_W * 0.3),
                ));
                if control.control_type == BallControlType::Move2D {
                    let mut line_builder = PathBuilder::new();
                    line_builder.move_to(Vec2::new(text_shadow_x, lv_y - LV_CONTROL_SIZE * 0.5));
                    line_builder.line_to(Vec2::new(text_shadow_x, lv_y + LV_CONTROL_SIZE * 0.5));
                    parent.spawn((
                        ShapeBundle {
                            path: line_builder.build(),
                            spatial: SpatialBundle {
                                transform: Transform::from_xyz(0.0, 0.0, lv_z + 0.000),
                                ..default()
                            },
                            ..default()
                        },
                        Stroke::new(main_line_color, LV_LINE_W * 0.3),
                    ));
                }
            }
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
                Stroke::new(main_line_color, LV_LINE_W),
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
    if game_status.mode != StatusMode::Demo {
        parent
            .spawn((SpatialBundle {
                transform: Transform::from_xyz(0.0, lv_y_base, lv_z),
                ..default()
            },))
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
                            color: main_text_color,
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
                    Stroke::new(main_line_color, LV_LINE_W * 0.8),
                ));
            });
    }
}
