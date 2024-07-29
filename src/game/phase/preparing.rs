use self::{
    ball::Ball,
    ball_interaction::{build_hit_map, calcuate_v_after_hit, HitAction},
};
use super::*;
use crate::app::{
    anime_effect::{self, ANIME_EFFECT_DONE_EVENT},
    audio, key_binding, settings, ui,
};
use bevy_persistent::prelude::*;
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
    game_status.mode = StatusMode::Preparing;
    key_binding.mode = key_binding::KeyBindingMode::Gaming;
    let dyn_entity = dyn_query.get_single().unwrap();
    let mut entity_commands = commands.get_entity(dyn_entity).unwrap();
    entity_commands.despawn_descendants();
    entity_commands.with_children(|parent| {
        let ball_configs = level_builder::LEVEL_MAP.get("hit_test").unwrap();
        // let ball_configs = level_builder::LEVEL_MAP.get("simple").unwrap();
        for ball_config in ball_configs.balls.iter() {
            Ball::create_sprite(
                ball_config.ball_type.clone(),
                parent,
                {},
                ball_config.property.clone(),
                ball_config.control_params.clone(),
            );
        }
    });
}

fn state_update(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Ball, &mut Transform), With<Ball>>,
    mut ae_query: Query<(Entity, &mut anime_effect::AnimeEffect), With<anime_effect::AnimeEffect>>,
    mut tween_completed_events: EventReader<TweenCompleted>,

    mut refresh_timer: ResMut<timer::GameRefreshTimer>,
    time: Res<Time>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
    mut ball_tick: Local<u8>,
) {
    if refresh_timer.0.tick(time.delta()).just_finished() {
        if *ball_tick == 0 {
            let hit_map = build_hit_map(&ball_query);
            // NOTE: Detect success and failure first
            for (e, _, _) in ball_query.iter_mut() {
                if let Some(actions) = hit_map.get(&e) {
                    for action in actions.iter() {
                        match action {
                            HitAction::Success => {
                                // success
                            }
                            HitAction::Failure => {
                                // failure
                            }
                            _ => (),
                        }
                    }
                }
            }
            // NOTE: Handle ball hit
            for (e, mut b, _) in ball_query.iter_mut() {
                let mut is_hit: bool = false;
                let mut is_no_hit_detected: bool = true;
                let mut bp = b.property.clone();
                if let Some(actions) = hit_map.get(&e) {
                    for action in actions.iter() {
                        match action {
                            HitAction::Move(info) => {
                                if b.check_hit_window(info.opponent_entity) {
                                    let new_v = calcuate_v_after_hit(
                                        &info.hit_type,
                                        &bp,
                                        &info.opponent_property,
                                    );
                                    bp.pos = bp.pos + new_v;
                                    bp.v = new_v;
                                    b.store_hit_entity(info.opponent_entity);
                                    is_hit = true
                                }
                                is_no_hit_detected = false;
                            }
                            _ => (),
                        }
                    }
                }
                if is_hit && b.property.movement_type == BallMovementType::Movable {
                    b.update_pos(bp.pos);
                    b.update_v(bp.v);
                    audio::play_se("hit", &mut commands, &asset_server, settings.as_ref());
                }
                if is_no_hit_detected {
                    b.clear_hit_counter();
                }
            }
            for (_, mut ball, mut trans) in ball_query.iter_mut() {
                if ball.state == BallState::Running
                    && ball.property.movement_type == BallMovementType::Movable
                {
                    ball.travel();
                    trans.translation = ball.property.pos.extend(0.0);
                }
            }
            *ball_tick = 1;
        } else {
            *ball_tick -= 1;
        }
        for (_, mut ball, _) in ball_query.iter_mut() {
            if ball.state == BallState::Created {
                ball.trigger_starting(&mut commands);
                continue;
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
