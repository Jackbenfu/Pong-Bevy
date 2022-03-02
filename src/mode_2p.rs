use bevy::prelude::*;

use crate::state::*;
use crate::helpers_system::*;

pub struct Mode2PPlugin;

impl Plugin for Mode2PPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Mode2P)
                    .with_system(back_to_menu_system)
            );
    }
}
