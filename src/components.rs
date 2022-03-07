use bevy::prelude::*;

use crate::Vec3;

pub struct GameOverEvent(pub Side);

pub struct BallOutEvent(pub Side);

pub struct BallHitPaddleEvent();

#[derive(Default)]
pub struct GameData {
    pub left_score: u32,
    pub right_score: u32,
    pub starting_side: Side,
    pub game_over: Option<Side>,
}

#[derive(Component)]
pub struct GameModeEntity {}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Side::Left
    }
}

#[derive(Component)]
pub struct LeftPaddle {
    pub speed: f32,
}

#[derive(Component)]
pub struct RightPaddle {
    /// For AI control.
    pub velocity: Vec3,
    /// For Human control.
    pub speed: f32,
}

#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub velocity: Vec3,
}

#[derive(Component)]
pub enum Collider {
    Paddle,
    Wall,
}

#[derive(Component)]
pub struct Service {}

#[derive(Component)]
pub struct LeftScore {}

#[derive(Component)]
pub struct RightScore {}

#[derive(Component)]
pub struct Instruction {}
