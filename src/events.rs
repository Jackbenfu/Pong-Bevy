use crate::components::*;

pub struct GameOverEvent(pub Side);

pub struct BallOutEvent(pub Side);

pub struct BallHitPaddleEvent();
