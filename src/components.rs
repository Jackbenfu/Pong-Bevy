use bevy::prelude::*;

use crate::Vec3;

pub struct GameData {
    pub left_score: u32,
    pub right_score: u32,
    pub starting_side: Side,
    pub game_over: Option<Side>,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            left_score: u32::default(),
            right_score: u32::default(),
            starting_side: Side::Left,
            game_over: None,
        }
    }
}

/// Event sent when a game mode ends.
pub struct GameOverEvent(pub Side);

/// Added to all entites in game modes (used to properly despawn them on exit).
#[derive(Component)]
pub struct GameModeEntity {}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
}

/// Left player, always controlled by human.
#[derive(Component)]
pub struct LeftPaddle {
    pub speed: f32,
}

/// Right player, controlled by either AI or human.
#[derive(Component)]
pub struct RightPaddle {
    /// For AI control.
    pub velocity: Vec3,
    /// For Human control.
    pub speed: f32,
}

/// Ball data.
#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub velocity: Vec3,
}

/// Types of colliders.
#[derive(Component)]
pub enum Collider {
    Paddle,
    Wall,
}

/// Added to the paddle to serve.
#[derive(Component)]
pub struct Service {}

#[derive(Component)]
pub struct LeftScore {}

#[derive(Component)]
pub struct RightScore {}

#[derive(Component)]
pub struct Instruction {}
