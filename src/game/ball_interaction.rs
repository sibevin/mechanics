use super::ball::*;
use bevy::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
#[derive(Debug, Default, Copy, Clone)]
pub struct PosV {
    pub pos: Vec2,
    pub v: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub enum HitAction {
    None,
    Move(PosV),
    Success,
    Failure,
}

#[derive(Debug, PartialEq)]
pub enum HitType {
    None,
    Outside,
    Inside,
}

type EntityBall<'a> = (Rc<RefCell<Entity>>, Rc<RefCell<&'a Ball>>);

pub fn build_hit_map(
    ball_query: &Query<(Entity, &mut Ball, &mut Transform), With<Ball>>,
) -> HashMap<Entity, HitAction> {
    let mut balls: Vec<EntityBall> = Vec::new();
    for (e, b, _) in ball_query.iter() {
        balls.push((Rc::new(RefCell::new(e)), Rc::new(RefCell::new(b))));
    }
    let mut hit_map: HashMap<Entity, HitAction> = HashMap::new();
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
    if pos1.distance(pos2) > r1 + r2 {
        return HitType::None;
    }
    if r1 >= r2 {
        if pos1.distance(pos2) > r1 {
            return HitType::Outside;
        } else if pos1.distance(pos2) > r1 - r2 {
            return HitType::Inside;
        } else {
            return HitType::None;
        }
    } else {
        if pos1.distance(pos2) > r2 {
            return HitType::Outside;
        } else if pos1.distance(pos2) > r2 - r1 {
            return HitType::Inside;
        } else {
            return HitType::None;
        }
    }
}

fn record_hit_action(
    hit_map: &mut HashMap<Entity, HitAction>,
    hit_type: HitType,
    e1: Entity,
    b1: &Ball,
    e2: Entity,
    b2: &Ball,
) {
    let mut e1_action: HitAction = hit_map.get(&e1).copied().unwrap_or(HitAction::None);
    let mut e2_action: HitAction = hit_map.get(&e2).copied().unwrap_or(HitAction::None);
    match b1.ball_type() {
        BallType::Bullet => match b2.ball_type() {
            BallType::Bullet => {
                let pos_vs = calculate_pos_v_after_hitting(&hit_type, b1, b2);
                e1_action = HitAction::Move(pos_vs.0);
                e2_action = HitAction::Move(pos_vs.1);
            }
            BallType::Goal => {
                e1_action = HitAction::Success;
                e2_action = HitAction::Success;
            }
            BallType::Bomb => {
                e1_action = HitAction::Failure;
                e2_action = HitAction::Failure;
            }
        },
        BallType::Goal => match b2.ball_type() {
            BallType::Goal => {
                let pos_vs = calculate_pos_v_after_hitting(&hit_type, b1, b2);
                e1_action = HitAction::Move(pos_vs.0);
                e2_action = HitAction::Move(pos_vs.1);
            }
            BallType::Bomb => {
                let pos_vs = calculate_pos_v_after_hitting(&hit_type, b1, b2);
                e1_action = HitAction::Move(pos_vs.0);
                e2_action = HitAction::Move(pos_vs.1);
            }
            _ => (),
        },
        BallType::Bomb => match b2.ball_type() {
            BallType::Bomb => {
                let pos_vs = calculate_pos_v_after_hitting(&hit_type, b1, b2);
                e1_action = HitAction::Move(pos_vs.0);
                e2_action = HitAction::Move(pos_vs.1);
            }
            _ => (),
        },
    }
    hit_map.insert(e1, e1_action);
    hit_map.insert(e2, e2_action);
}

fn calculate_pos_v_after_hitting(hit_type: &HitType, b1: &Ball, b2: &Ball) -> (PosV, PosV) {
    let mut pv1_new = PosV::default();
    let mut pv2_new = PosV::default();
    let v1_new = calcuate_v_after_hitting(b1, b2, hit_type);
    pv1_new.v = v1_new;
    pv1_new.pos = b1.property.pos + v1_new;
    let v2_new = calcuate_v_after_hitting(b2, b1, hit_type);
    pv2_new.v = v2_new;
    pv2_new.pos = b2.property.pos + v2_new;
    (pv1_new, pv2_new)
}

fn calcuate_v_after_hitting(b1: &Ball, b2: &Ball, hit_type: &HitType) -> Vec2 {
    let v1 = b1.property.v;
    if v1.length() == 0.0 {
        return v1;
    }
    let v2 = b2.property.v;
    let vp = if *hit_type == HitType::Outside {
        b2.property.pos - b1.property.pos
    } else {
        b1.property.pos - b2.property.pos
    };
    let dv = (v1.project_onto(vp.normalize().perp()) * 2.0 - v1).normalize();
    if v2.length() == 0.0 {
        dv * v1.length()
    } else {
        let m1 = b1.property.radius * b1.property.radius;
        let m2 = b2.property.radius * b2.property.radius;
        dv * ((m1 - m2) * v1.length() + 2.0 * m2 * v2.length()) / (m1 + m2)
    }
}
