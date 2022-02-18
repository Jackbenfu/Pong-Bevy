mod config_sprite;
mod config_game;
mod resources;
mod helpers_system;
mod helpers_sprite;
mod state;
mod menu;
mod mode_1p;
mod mode_2p;
mod mode_wall;

use bevy::prelude::*;
use config_sprite::*;
use config_game::*;
use resources::*;
use menu::*;
use mode_1p::*;
use mode_2p::*;
use mode_wall::*;
use crate::state::GameState;

fn setup_system(
    mut commands: Commands,
    mut resources: ResMut<Resources>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    resources.font = asset_server.load("fonts/Volter__28Goldfish_29.ttf");
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .init_resource::<Resources>()
        .init_resource::<SpriteConfig>()
        .init_resource::<GameConfig>()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "error,pong_bevy=debug".to_string(),
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 768.,
            height: 576.,
            resizable: false,
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(Mode1PPlugin)
        .add_plugin(Mode2PPlugin)
        .add_plugin(ModeWallPlugin)
        .add_state(GameState::Mode1P)
        .run();
}
