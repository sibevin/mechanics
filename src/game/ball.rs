use bevy::prelude::*;
use bevy_prototype_lyon::path::PathBuilder;
use bevy_tweening::lens::*;
use bevy_tweening::Targetable;
use circular_queue::CircularQueue;
use std::fmt;

use crate::app::ui;

pub mod bomb;
pub mod bullet;
pub mod goal;

pub const STARTING_DONE_EVENT: u64 = 0;
pub const ENDING_DONE_EVENT: u64 = 1;
const TAILING_SIZE: usize = 5;
const TAILING_WINDOW: u8 = 3;
const BALL_LINE_W: f32 = ui::FONT_SIZE / 12.0;
const BALL_OUTER_W: f32 = BALL_LINE_W * 3.0;

#[derive(Component, Debug, PartialEq)]
pub enum BallType {
    Bullet,
    Goal,
    Bomb,
}

#[derive(Debug, PartialEq)]
pub enum BallState {
    Created,
    Starting,
    Running,
    Paused,
    Ending,
    Dead,
}

#[derive(Debug, PartialEq)]
pub enum BallMovementType {
    Movable,
    Fixed,
    FixedReversed,
}

pub trait BallAbility {
    fn ball_type(&self) -> BallType;
    fn color(&self) -> Color;
    fn setup_starting_anime(&self, _commands: &mut Commands, _ball: &Ball) {}
    fn setup_ending_anime(&self, _commands: &mut Commands, _ball: &Ball) {}
    fn update_anime(&self, _commands: &mut Commands, _ball: &Ball) {}
}

pub struct BallProperty {
    pub radius: f32,
    pub pos: Vec2,
    pub v: Vec2,
    pub movement_type: BallMovementType,
}

pub struct BallAnimeParams {
    pub radius: f32,
    pub alpha: f32,
}

#[derive(Component)]
pub struct Ball {
    pub state: BallState,
    pub property: BallProperty,
    pub anime_params: BallAnimeParams,
    ability: Box<dyn BallAbility + Send + Sync>,
    tailings: CircularQueue<Vec2>,
    tailing_counter: u8,
    root_entity: Entity,
    canvas_entity: Entity,
}

fn build_ability(ball_type: &BallType) -> Box<dyn BallAbility + Send + Sync> {
    match ball_type {
        BallType::Bullet => Box::new(bullet::Ability),
        BallType::Goal => Box::new(goal::Ability),
        BallType::Bomb => Box::new(bomb::Ability),
    }
}

impl Ball {
    pub fn create_sprite(
        ball_type: BallType,
        parent: &mut ChildBuilder,
        bundle: impl Bundle,
        property: BallProperty,
    ) {
        let ability = build_ability(&ball_type);
        let mut canvas_entity: Entity = Entity::PLACEHOLDER;
        let mut root_entity = parent.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(property.pos.x, property.pos.y, 0.0),
                sprite: Sprite {
                    color: ability.color(),
                    ..default()
                },
                ..default()
            },
            bundle,
        ));
        root_entity.with_children(|parent| {
            canvas_entity = parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    sprite: Sprite {
                        color: ability.color(),
                        ..default()
                    },
                    ..default()
                })
                .id();
        });
        let ball = Ball::new(ball_type, property, root_entity.id(), canvas_entity);
        root_entity.insert(ball);
    }
    pub fn new(
        ball_type: BallType,
        property: BallProperty,
        root_entity: Entity,
        canvas_entity: Entity,
    ) -> Self {
        Self {
            ability: build_ability(&ball_type),
            property,
            anime_params: BallAnimeParams {
                radius: 0.0,
                alpha: 0.0,
            },
            state: BallState::Created,
            tailings: CircularQueue::with_capacity(TAILING_SIZE),
            tailing_counter: 0,
            root_entity,
            canvas_entity,
        }
    }
    pub fn ball_type(&self) -> BallType {
        self.ability.ball_type()
    }
    pub fn update_pos(&mut self, pos: Vec2) {
        self.property.pos = pos;
        self.record_tailing(self.property.pos);
    }
    pub fn update_v(&mut self, v: Vec2) {
        self.property.v = v;
    }
    pub fn root_entity(&self) -> Entity {
        self.root_entity
    }
    pub fn canvas_entity(&self) -> Entity {
        self.canvas_entity
    }
    pub fn travel(&mut self) -> Vec2 {
        if self.property.movement_type == BallMovementType::Movable {
            self.update_pos(self.property.pos + self.property.v);
        }
        self.property.pos
    }
    pub fn jump(&mut self, pos: Vec2) {
        self.property.pos = pos;
        self.record_tailing(self.property.pos);
    }
    fn tailings(&self) -> Option<&CircularQueue<Vec2>> {
        Some(&self.tailings)
    }
    fn record_tailing(&mut self, pos: Vec2) {
        if self.tailing_counter == 0 {
            self.tailings.push(pos);
            self.tailing_counter = TAILING_WINDOW;
        } else {
            self.tailing_counter -= 1;
        }
    }
    pub fn trigger_anime(&mut self, state: BallState) {
        self.state = state
    }
    pub fn trigger_starting(&mut self, commands: &mut Commands) {
        self.ability.setup_starting_anime(commands, self);
        self.trigger_anime(BallState::Starting);
    }
    pub fn update_anime(&self, commands: &mut Commands) {
        self.ability.update_anime(commands, self);
    }
    pub fn trigger_ending(&mut self, commands: &mut Commands) {
        self.ability.setup_ending_anime(commands, self);
        self.trigger_anime(BallState::Ending);
    }
}

impl fmt::Debug for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ball")
            .field("ball_type", &self.ball_type())
            .field("pos", &self.property.pos)
            .field("v", &self.property.v)
            .field("state", &self.state)
            .finish()
    }
}

struct BallAnimeLens {
    start_radius: f32,
    start_color_alpha: f32,
    end_radius: f32,
    end_color_alpha: f32,
}

impl Lens<Ball> for BallAnimeLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Ball>, ratio: f32) {
        target.anime_params.radius =
            self.start_radius + (self.end_radius - self.start_radius) * ratio;
        target.anime_params.alpha =
            self.start_color_alpha + (self.end_color_alpha - self.start_color_alpha) * ratio;
    }
}

const FIXED_BG_LINE_W: f32 = ui::FONT_SIZE / 36.0;
const FIXED_BG_LINE_P: f32 = FIXED_BG_LINE_W * 6.0;

fn build_fixed_bg_path(radius: f32) -> PathBuilder {
    let mut path_builder = PathBuilder::new();
    let line_count = (radius / FIXED_BG_LINE_P).floor() as u8;
    let rotation = Quat::from_rotation_z(45.0_f32.to_radians());
    path_builder.move_to(
        rotation
            .mul_vec3(Vec2::new(radius, 0.0).extend(0.0))
            .truncate(),
    );
    path_builder.line_to(
        rotation
            .mul_vec3(Vec2::new(-radius, 0.0).extend(0.0))
            .truncate(),
    );
    for i in 1..=line_count {
        let line_y = FIXED_BG_LINE_P * i as f32;
        let line_x = (radius * radius - line_y * line_y).sqrt();
        path_builder.move_to(
            rotation
                .mul_vec3(Vec2::new(line_x, line_y).extend(0.0))
                .truncate(),
        );
        path_builder.line_to(
            rotation
                .mul_vec3(Vec2::new(-line_x, line_y).extend(0.0))
                .truncate(),
        );
        path_builder.move_to(
            rotation
                .mul_vec3(Vec2::new(line_x, -line_y).extend(0.0))
                .truncate(),
        );
        path_builder.line_to(
            rotation
                .mul_vec3(Vec2::new(-line_x, -line_y).extend(0.0))
                .truncate(),
        );
    }
    return path_builder;
}
