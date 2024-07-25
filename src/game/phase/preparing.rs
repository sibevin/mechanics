use self::ball::{Ball, BallProperty};
use super::*;
use crate::app::{
    anime_effect::{self, ANIME_EFFECT_DONE_EVENT},
    key_binding, ui,
};
use bevy_tweening::{component_animator_system, TweenCompleted};
use std::collections::HashSet;

pub struct Phase;

impl PhaseBase for Phase {
    fn state(&self) -> PhaseState {
        PhaseState::Preparing
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), state_enter)
            .add_systems(
                Update,
                (
                    state_update,
                    component_animator_system::<Ball>,
                    component_animator_system::<anime_effect::AnimeEffect>,
                    field::field_systems(),
                )
                    .run_if(in_state(self.state())),
            )
            .add_systems(
                OnExit(self.state()),
                (
                    anime_effect::clear_anime_effect,
                    state_exit,
                    ui::despawn_ui::<OnPage>,
                ),
            );
    }
}

#[derive(Component)]
struct OnPage;

fn state_enter(
    mut commands: Commands,
    dyn_query: Query<Entity, With<GameDyn>>,
    mut game_status: ResMut<GameStatus>,
    mut key_binding: ResMut<key_binding::KeyBindingConfig>,
) {
    game_status.mode = StatusMode::Playing;
    key_binding.mode = key_binding::KeyBindingMode::Gaming;
    let dyn_entity = dyn_query.get_single().unwrap();
    let mut entity_commands = commands.get_entity(dyn_entity).unwrap();
    entity_commands.despawn_descendants();
    entity_commands.with_children(|parent| {
        Ball::create_sprite(
            BallType::Bullet,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(0.0, 0.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::Movable,
            },
        );
        Ball::create_sprite(
            BallType::Bomb,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(100.0, 0.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::Movable,
            },
        );
        Ball::create_sprite(
            BallType::Goal,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(200.0, 0.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::Movable,
            },
        );
        Ball::create_sprite(
            BallType::Bullet,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(0.0, 100.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::Fixed,
            },
        );
        Ball::create_sprite(
            BallType::Bomb,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(100.0, 100.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::Fixed,
            },
        );
        Ball::create_sprite(
            BallType::Goal,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(200.0, 100.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::Fixed,
            },
        );
        Ball::create_sprite(
            BallType::Bullet,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(0.0, 200.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::FixedReversed,
            },
        );
        Ball::create_sprite(
            BallType::Bomb,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(100.0, 200.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::FixedReversed,
            },
        );
        Ball::create_sprite(
            BallType::Goal,
            parent,
            {},
            BallProperty {
                pos: Vec2::new(200.0, 200.0),
                v: Vec2::new(0.0, 0.0),
                radius: 24.0,
                movement_type: BallMovementType::FixedReversed,
            },
        );
    });
}

fn state_update(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Ball, &mut Transform), With<Ball>>,
    mut ae_query: Query<(Entity, &mut anime_effect::AnimeEffect), With<anime_effect::AnimeEffect>>,
    mut tween_completed_events: EventReader<TweenCompleted>,

    mut refresh_timer: ResMut<timer::GameRefreshTimer>,
    time: Res<Time>,
) {
    if refresh_timer.0.tick(time.delta()).just_finished() {
        for (_, mut ball, _) in ball_query.iter_mut() {
            if ball.state == BallState::Created {
                ball.trigger_starting(&mut commands);
                continue;
            }
            if ball.state == BallState::Running {
                ball.travel();
                // TODO
                // 1. Detect hit
                // 2. Update position
            }
            ball.update_anime(&mut commands);
        }
        for (_, mut ae) in ae_query.iter_mut() {
            anime_effect::update_anime_effect(&mut commands, &mut ae);
        }

        let mut entities_to_despawn: HashSet<Entity> = HashSet::new();
        for tween_event in tween_completed_events.read() {
            if tween_event.user_data == STARTING_DONE_EVENT {
                for (e, mut ball, _) in ball_query.iter_mut() {
                    if e == tween_event.entity {
                        ball.trigger_anime(BallState::Running);
                    }
                }
            }
            if tween_event.user_data == ENDING_DONE_EVENT
                || tween_event.user_data == ANIME_EFFECT_DONE_EVENT
            {
                entities_to_despawn.insert(tween_event.entity);
            }
        }
        for entity in entities_to_despawn.iter() {
            if let Some(entity_commands) = commands.get_entity(*entity) {
                entity_commands.despawn_recursive()
            }
        }
    }
}

fn state_exit() {}
