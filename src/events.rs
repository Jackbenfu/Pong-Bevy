use bevy::ecs::event::Event;
use crate::components::*;

pub struct GameOverEvent(pub Side);

impl Event for GameOverEvent {}

pub struct BallOutEvent(pub Side);

impl Event for BallOutEvent {}

pub struct BallHitPaddleEvent();

impl Event for BallHitPaddleEvent {}
