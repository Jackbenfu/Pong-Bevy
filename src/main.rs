mod config;
mod systems_generic;
mod systems_1v1;
mod components;
mod events;
mod state;
mod helpers_sprite;
mod menu;
mod mode_1p;
mod mode_2p;
mod mode_wall;

use bevy::prelude::*;
use bevy_kira_audio::{AudioPlugin};
use config::*;
use state::*;
use menu::*;
use mode_1p::*;
use mode_2p::*;
use mode_wall::*;

fn setup_system(
    mut commands: Commands,
    mut config: ResMut<Config>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    config.game_paddle_speed = 400.;
    config.game_ball_speed_min = 500.;
    config.game_ball_speed_max = 750.;
    config.game_ball_speed_incr = 5.;
    config.game_ball_oob_x = 200.;
    config.game_1v1_score_to_win = 9;

    config.sprite_unit_size = 16.;

    //config.color_transparent = Color::rgba_u8(0, 128, 0, 64); // DEBUG
    config.color_transparent = Color::rgba_u8(0, 0, 0, 0);
    config.color_white = Color::WHITE;
    config.color_grey = Color::rgb_u8(100, 100, 100);
    config.color_yellow = Color::rgb_u8(221, 173, 29);
    config.color_green = Color::rgb_u8(69, 183, 130);
    config.color_red = Color::rgb_u8(196, 89, 73);

    config.font = asset_server.load("fonts/Volter__28Goldfish_29.ttf");

    config.audio_paddle_left = asset_server.load("sounds/left.wav");
    config.audio_paddle_right = asset_server.load("sounds/right.wav");
    config.audio_wall = asset_server.load("sounds/wall.wav");
}

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::ERROR,
            filter: "error,pong_bevy=error".to_string(),
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
        .init_resource::<Config>()
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(Mode1PPlugin)
        .add_plugin(Mode2PPlugin)
        .add_plugin(ModeWallPlugin)
        .add_state(GameState::Menu)
        .run();
}
