use bevy::prelude::States;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Mode1P,
    Mode2P,
    ModeWall,
}
