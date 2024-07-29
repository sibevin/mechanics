use super::*;
use crate::app::theme;
use bevy_tweening::*;
use std::time::Duration;

pub struct Ability;

impl BallAbility for Ability {
    fn ball_type(&self) -> BallType {
        BallType::Bomb
    }
    fn color(&self) -> Color {
        theme::FAILURE_COLOR
    }
    fn setup_starting_anime(&self, commands: &mut Commands, ball: &Ball) {
        setup_starting_anime(commands, ball);
    }
    fn setup_ending_anime(&self, commands: &mut Commands, ball: &Ball) {
        setup_ending_anime(commands, ball);
    }
    fn update_starting_anime(&self, commands: &mut Commands, ball: &Ball) {
        update_starting_anime(commands, ball);
    }
    fn update_ending_anime(&self, commands: &mut Commands, ball: &Ball) {
        update_ending_anime(commands, ball);
    }
}

fn setup_starting_anime(commands: &mut Commands, ball: &Ball) {
    if let Some(mut entity_commands) = commands.get_entity(ball.root_entity()) {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(BALL_START_ANIME_L),
            BallAnimeLens {
                start_radius: ball.property.radius * 2.5,
                start_color_alpha: 0.0,
                end_radius: ball.property.radius,
                end_color_alpha: 0.5,
            },
        )
        .with_completed_event(STARTING_DONE_EVENT);
        entity_commands.insert(Animator::new(tween));
    }
}

fn setup_ending_anime(commands: &mut Commands, ball: &Ball) {
    if let Some(mut entity_commands) = commands.get_entity(ball.root_entity()) {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(BALL_END_ANIME_L),
            BallAnimeLens {
                start_radius: ball.property.radius,
                start_color_alpha: 0.3,
                end_radius: ball.property.radius * 5.0,
                end_color_alpha: 0.0,
            },
        )
        .with_completed_event(ENDING_DONE_EVENT);
        entity_commands.insert(Animator::new(tween));
    }
}

fn update_starting_anime(commands: &mut Commands, ball: &Ball) {
    if ball.state != BallState::Starting {
        return;
    }
    if let Some(mut entity_commands) = commands.get_entity(ball.bg_entity()) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            let z_layer = if ball.property.movement_type == BallMovementType::FixedReversed {
                0.0
            } else {
                1.0
            };
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, z_layer),
                    ..default()
                })
                .with_children(|parent| {
                    match ball.property.movement_type {
                        BallMovementType::Movable => {
                            let shape = shapes::Circle {
                                radius: ball.anime_params.radius - BALL_OUTER_W,
                                center: Vec2::new(0.0, 0.0),
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, z_layer + 0.001),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Fill::color(
                                    ball.ability.color().with_alpha(ball.anime_params.alpha),
                                ),
                            ));
                        }
                        BallMovementType::Fixed => {
                            let bg_builder =
                                build_fixed_bg_path(ball.anime_params.radius - BALL_OUTER_W);
                            parent.spawn((
                                ShapeBundle {
                                    path: bg_builder.build(),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, z_layer + 0.001),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Stroke::new(
                                    ball.ability.color().with_alpha(ball.anime_params.alpha),
                                    BALL_LINE_W,
                                ),
                            ));
                            let shape = shapes::Circle {
                                radius: ball.anime_params.radius - BALL_OUTER_W,
                                center: Vec2::new(0.0, 0.0),
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, z_layer + 0.0015),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Stroke::new(
                                    ball.ability.color().with_alpha(ball.anime_params.alpha),
                                    BALL_LINE_W,
                                ),
                            ));
                        }
                        BallMovementType::FixedReversed => {
                            let bg_builder =
                                build_fixed_bg_path(ball.anime_params.radius + BALL_OUTER_W * 3.0);
                            parent.spawn((
                                ShapeBundle {
                                    path: bg_builder.build(),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, z_layer + 0.001),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Stroke {
                                    color: ball.ability.color().with_alpha(ball.anime_params.alpha),
                                    options: StrokeOptions::DEFAULT
                                        .with_line_width(BALL_LINE_W)
                                        .with_line_cap(LineCap::Round),
                                },
                            ));
                            let shape = shapes::Circle {
                                radius: ball.anime_params.radius + BALL_OUTER_W,
                                center: Vec2::new(0.0, 0.0),
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, z_layer + 0.0015),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Stroke::new(
                                    ball.ability.color().with_alpha(ball.anime_params.alpha),
                                    BALL_LINE_W,
                                ),
                                Fill::color(theme::BG_COLOR),
                            ));
                        }
                    }
                    let shape = shapes::Circle {
                        radius: ball.anime_params.radius,
                        center: Vec2::new(0.0, 0.0),
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            spatial: SpatialBundle {
                                transform: Transform::from_xyz(0.0, 0.0, z_layer + 0.002),
                                ..default()
                            },
                            ..default()
                        },
                        Stroke::new(
                            ball.ability.color().with_alpha(ball.anime_params.alpha),
                            BALL_LINE_W,
                        ),
                    ));
                });
        });
    }
}

fn update_ending_anime(commands: &mut Commands, ball: &Ball) {
    if ball.state != BallState::Ending {
        return;
    }
    if let Some(mut entity_commands) = commands.get_entity(ball.bg_entity()) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::Circle {
                        radius: ball.anime_params.radius,
                        center: Vec2::new(0.0, 0.0),
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(ball.ability.color().with_alpha(ball.anime_params.alpha)),
                    ));
                });
        });
    }
}
