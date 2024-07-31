use super::*;
use crate::app::{theme, ui};
use bevy_prototype_lyon::prelude::*;

pub const FIELD_LINE_W: f32 = ui::SPACE_SIZE * 0.5;
pub const FIELD_COLOR: Color = theme::BG_COLOR;
pub const FIELD_TEXT_COLOR: Color = theme::FG_COLOR;
pub const FIELD_SB_TEXT_COLOR: Color = theme::SECONDARY_COLOR;

const FIELD_REFRESH_TARGET: &str = "FIELD_REFRESH_TARGET";

pub fn refresh_field(
    mut commands: Commands,
    bg_query: Query<Entity, With<GameBg>>,
    window_query: Query<&Window>,
    mut game_status: ResMut<GameStatus>,
) {
    if !game_status.is_refreshed(FIELD_REFRESH_TARGET) {
        let bg_entity = bg_query.get_single().unwrap();
        let mut entity_commands = commands.get_entity(bg_entity).unwrap();
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            draw_grid_bg(parent, &window_query);
        });
        game_status.set_refreshed(FIELD_REFRESH_TARGET)
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
