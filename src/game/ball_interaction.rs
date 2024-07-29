use super::ball::*;
use bevy::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct HitMoveInfo {
    pub hit_type: HitType,
    pub opponent_entity: Entity,
    pub opponent_property: BallProperty,
}

#[derive(Clone)]
pub enum HitAction {
    Move(HitMoveInfo),
    Success,
    Failure,
}

#[derive(Clone, PartialEq)]
pub enum HitType {
    None,
    Outside,
    Inside,
}

type EntityBall<'a> = (Rc<RefCell<Entity>>, Rc<RefCell<&'a Ball>>);

pub fn build_hit_map(
    ball_query: &Query<(Entity, &mut Ball, &mut Transform), With<Ball>>,
) -> HashMap<Entity, Vec<HitAction>> {
    let mut balls: Vec<EntityBall> = Vec::new();
    for (e, b, _) in ball_query.iter() {
        balls.push((Rc::new(RefCell::new(e)), Rc::new(RefCell::new(b))));
    }
    let mut hit_map: HashMap<Entity, Vec<HitAction>> = HashMap::new();
    for (i, (e1, b1)) in balls.iter().enumerate() {
        if b1.borrow().state != BallState::Running {
            continue;
        }
        for (j, (e2, b2)) in balls.iter().enumerate() {
            if b2.borrow().state != BallState::Running {
                continue;
            }
            if j > i {
                let hit_type = detect_hit(&b1.borrow(), &b2.borrow());
                if hit_type != HitType::None {
                    record_hit_action(
                        &mut hit_map,
                        hit_type,
                        *e1.borrow(),
                        &b1.borrow(),
                        *e2.borrow(),
                        &b2.borrow(),
                    )
                }
            }
        }
    }
    hit_map
}

pub fn detect_hit(b1: &Ball, b2: &Ball) -> HitType {
    let pos1 = b1.property.pos;
    let pos2 = b2.property.pos;
    let r1 = b1.property.radius;
    let r2 = b2.property.radius;
    if b1.property.movement_type != BallMovementType::FixedReversed
        && b2.property.movement_type != BallMovementType::FixedReversed
    {
        if pos1.distance(pos2) > r1 + r2 {
            return HitType::None;
        } else {
            return HitType::Outside;
        }
    }
    if r1 >= r2 {
        if pos1.distance(pos2) > r1 - r2 {
            return HitType::Inside;
        } else {
            return HitType::None;
        }
    } else {
        if pos1.distance(pos2) > r2 - r1 {
            return HitType::Inside;
        } else {
            return HitType::None;
        }
    }
}

fn record_hit_action(
    hit_map: &mut HashMap<Entity, Vec<HitAction>>,
    hit_type: HitType,
    e1: Entity,
    b1: &Ball,
    e2: Entity,
    b2: &Ball,
) {
    let hit_status: &str;
    let e1_action: HitAction;
    let e2_action: HitAction;
    match b1.ball_type() {
        BallType::Stone => match b2.ball_type() {
            BallType::Stone => {
                hit_status = "hit";
            }
            BallType::Goal => {
                hit_status = "success";
            }
            BallType::Bomb => {
                hit_status = "failure";
            }
        },
        BallType::Goal => match b2.ball_type() {
            BallType::Stone => {
                hit_status = "success";
            }
            BallType::Goal => {
                hit_status = "hit";
            }
            BallType::Bomb => {
                hit_status = "hit";
            }
        },
        BallType::Bomb => match b2.ball_type() {
            BallType::Stone => {
                hit_status = "failure";
            }
            BallType::Goal => {
                hit_status = "hit";
            }
            BallType::Bomb => {
                hit_status = "hit";
            }
        },
    }
    match hit_status {
        "hit" => {
            e1_action = HitAction::Move(HitMoveInfo {
                hit_type: hit_type.clone(),
                opponent_entity: e2,
                opponent_property: b2.property.clone(),
            });
            e2_action = HitAction::Move(HitMoveInfo {
                hit_type: hit_type.clone(),
                opponent_entity: e1,
                opponent_property: b1.property.clone(),
            });
        }
        "success" => {
            e1_action = HitAction::Success;
            e2_action = HitAction::Success;
        }
        "failure" => {
            e1_action = HitAction::Failure;
            e2_action = HitAction::Failure;
        }
        _ => return,
    }
    if let Some(actions) = hit_map.get_mut(&e1) {
        actions.push(e1_action);
    } else {
        hit_map.insert(e1, vec![e1_action]);
    }
    if let Some(actions) = hit_map.get_mut(&e2) {
        actions.push(e2_action);
    } else {
        hit_map.insert(e2, vec![e2_action]);
    }
}

pub fn calcuate_v_after_hit(hit_type: &HitType, bp1: &BallProperty, bp2: &BallProperty) -> Vec2 {
    let v1 = bp1.v;
    let v2 = bp2.v;
    let m1 = bp1.radius.powi(2);
    let m2 = bp2.radius.powi(2);
    let vp = if *hit_type == HitType::Outside {
        bp1.pos - bp2.pos
    } else {
        bp2.pos - bp1.pos
    };
    if bp2.movement_type == BallMovementType::Movable {
        v1 - 2.0 * m2 / (m1 + m2) * (v1 - v2).dot(vp) / vp.length().powi(2) * vp
    } else {
        if bp2.movement_type == BallMovementType::FixedReversed && v1.dot(vp) > 0.0 {
            // NOTE: If v1 is toward center, keep v1 not change to make sure ball will not move
            // outside
            v1
        } else {
            v1 - 2.0 * v1.dot(vp) / vp.length().powi(2) * vp
        }
    }
}

// pub fn calcuate_v_after_hit(hit_type: &HitType, bp1: &BallProperty, bp2: &BallProperty) -> Vec2 {
//     let v1 = bp1.v;
//     if v1.length() == 0.0 {
//         return v1;
//     }
//     let v2 = bp2.v;
//     let vp = if *hit_type == HitType::Outside {
//         bp2.pos - bp1.pos
//     } else {
//         bp1.pos - bp2.pos
//     };
//     let dv = (v1.project_onto(vp.normalize().perp()) * 2.0 - v1).normalize();
//     if v2.length() == 0.0 {
//         dv * v1.length()
//     } else {
//         let m1 = bp1.radius * bp1.radius;
//         let m2 = bp2.radius * bp2.radius;
//         dv * ((m1 - m2) * v1.length() + 2.0 * m2 * v2.length()) / (m1 + m2)
//     }
// }
