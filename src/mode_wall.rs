use bevy::prelude::*;
use crate::state::*;
use crate::utils::*;

pub struct ModeWallPlugin;

impl Plugin for ModeWallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::ModeWall)
                    .with_system(back_to_menu_system)
            );
    }
}
