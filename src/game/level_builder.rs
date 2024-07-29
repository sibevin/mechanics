use super::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct LevelBallConfig {
    pub ball_type: BallType,
    pub property: BallProperty,
    pub control_params: BallControlParams,
}

pub struct LevelConfig {
    pub name: &'static str,
    pub balls: Vec<LevelBallConfig>,
}

const BL_LIST_GAP: f32 = 150.0;
const BL_BALL_SIZE: f32 = 30.0;

pub const LEVELS: [&str; 7] = ["hello", "timing", "twice", "maze", "star", "chaser", "pool"];
lazy_static! {
    pub static ref LEVEL_MAP: HashMap<&'static str, LevelConfig> = {
        let mut m = HashMap::new();
        m.insert(
            "ball_list",
            LevelConfig {
                name: "BALL LIST",
                balls: vec![
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-BL_LIST_GAP, BL_LIST_GAP),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-BL_LIST_GAP, 0.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::Fixed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-BL_LIST_GAP, -BL_LIST_GAP),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::FixedReversed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Goal,
                        property: BallProperty {
                            pos: Vec2::new(0.0, BL_LIST_GAP),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Goal,
                        property: BallProperty {
                            pos: Vec2::new(0.0, 0.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::Fixed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Goal,
                        property: BallProperty {
                            pos: Vec2::new(0.0, -BL_LIST_GAP),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::FixedReversed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(BL_LIST_GAP, BL_LIST_GAP),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(BL_LIST_GAP, 0.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::Fixed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(BL_LIST_GAP, -BL_LIST_GAP),
                            v: Vec2::new(0.0, 0.0),
                            radius: BL_BALL_SIZE,
                            movement_type: BallMovementType::FixedReversed,
                        },
                        control_params: BallControlParams::default(),
                    },
                ],
            },
        );
        m.insert(
            "simple",
            LevelConfig {
                name: "SIMPLE",
                balls: vec![
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-300.0, 0.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: 200.0,
                            movement_type: BallMovementType::FixedReversed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-300.0, 10.0),
                            v: Vec2::new(10.0, 0.0),
                            radius: 10.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-300.0, 40.0),
                            v: Vec2::new(5.0, 0.0),
                            radius: 10.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-300.0, 70.0),
                            v: Vec2::new(5.0, 0.0),
                            radius: 15.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Stone,
                        property: BallProperty {
                            pos: Vec2::new(-350.0, 70.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: 20.0,
                            movement_type: BallMovementType::Fixed,
                        },
                        control_params: BallControlParams::default(),
                    },
                ],
            },
        );
        m.insert(
            "hit_test",
            LevelConfig {
                name: "SIMPLE",
                balls: vec![
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(0.0, 0.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: 200.0,
                            movement_type: BallMovementType::FixedReversed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(-10.0, 190.0),
                            v: Vec2::new(-10.0, -10.0),
                            radius: 10.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(0.0, 170.0),
                            v: Vec2::new(12.0, 0.0),
                            radius: 10.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(0.0, 150.0),
                            v: Vec2::new(-14.0, 0.0),
                            radius: 10.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(0.0, 0.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: 20.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Goal,
                        property: BallProperty {
                            pos: Vec2::new(-60.0, 0.0),
                            v: Vec2::new(1.0, 0.0),
                            radius: 30.0,
                            movement_type: BallMovementType::Movable,
                        },
                        control_params: BallControlParams::default(),
                    },
                    LevelBallConfig {
                        ball_type: BallType::Bomb,
                        property: BallProperty {
                            pos: Vec2::new(-60.0, -80.0),
                            v: Vec2::new(0.0, 0.0),
                            radius: 50.0,
                            movement_type: BallMovementType::Fixed,
                        },
                        control_params: BallControlParams::default(),
                    },
                    // LevelBallConfig {
                    //     ball_type: BallType::Stone,
                    //     property: BallProperty {
                    //         pos: Vec2::new(100.0, 300.0),
                    //         v: Vec2::new(-50.0, 0.0),
                    //         radius: 10.0,
                    //         movement_type: BallMovementType::Movable,
                    //     },
                    //     control_params: BallControlParams::default(),
                    // },
                    // LevelBallConfig {
                    //     ball_type: BallType::Stone,
                    //     property: BallProperty {
                    //         pos: Vec2::new(-100.0, 300.0),
                    //         v: Vec2::new(10.0, 0.0),
                    //         radius: 50.0,
                    //         movement_type: BallMovementType::Movable,
                    //     },
                    //     control_params: BallControlParams::default(),
                    // },
                ],
            },
        );
        m
    };
}
