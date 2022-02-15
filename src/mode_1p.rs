use bevy::prelude::*;
use crate::state::*;
use crate::helpers_system::*;
use crate::helpers_sprite::*;
use crate::SpriteConfig;

#[derive(Component)]
struct Mode1PEntity {}

pub struct Mode1PPlugin;

impl Plugin for Mode1PPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::Mode1P;

        app
            .add_system_set(
                SystemSet::on_enter(GAME_STATE)
                    .with_system(setup_court_system)
                    .with_system(setup_player_system)
                    .with_system(setup_ai_system)
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(back_to_menu_system)
                    .with_system(move_player_system)
                    .with_system(move_ai_system)
            )
            .add_system_set(
                SystemSet::on_exit(GAME_STATE)
                    .with_system(cleanup_system::<Mode1PEntity>)
            );
    }
}

fn setup_court_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    sprite_config: Res<SpriteConfig>,
) {
    let color = sprite_config.color_white;
    let unit_size = sprite_config.unit_size;

    commands
        .spawn_bundle(create_top_wall_sprite(window.width, window.height, unit_size, color))
        .insert(Mode1PEntity {});

    commands
        .spawn_bundle(create_bottom_wall_sprite(window.width, window.height, unit_size, color))
        .insert(Mode1PEntity {});

    // net
    {
        commands
            .spawn_bundle(create_net_sprite(0., unit_size, color))
            .insert(Mode1PEntity {});

        let mut y: f32 = unit_size * 3.;
        while y < window.height / 2. {
            commands
                .spawn_bundle(create_net_sprite(y, unit_size, color))
                .insert(Mode1PEntity {});

            commands
                .spawn_bundle(create_net_sprite(-y, unit_size, color))
                .insert(Mode1PEntity {});

            y += unit_size * 3.;
        }
    }
}

fn setup_player_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    sprite_config: Res<SpriteConfig>,
) {
    commands
        .spawn_bundle(create_left_paddle_sprite(window.width, sprite_config.unit_size, sprite_config.color_white))
        .insert(Mode1PEntity {});
}

fn setup_ai_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    sprite_config: Res<SpriteConfig>,
) {
    commands
        .spawn_bundle(create_right_paddle_sprite(window.width, sprite_config.unit_size, sprite_config.color_white))
        .insert(Mode1PEntity {});
}

fn move_player_system() {
    // TODO
}

fn move_ai_system() {
    // TODO
}
