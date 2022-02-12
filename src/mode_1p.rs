use bevy::prelude::*;
use crate::state::*;
use crate::utils::*;

pub struct Mode1PPlugin;

impl Plugin for Mode1PPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Mode1P)
                    .with_system(back_to_menu_system)
            );
    }
}
