use bevy::prelude::*;

use crate::state::*;
use crate::helpers_system::*;

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
