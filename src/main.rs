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

use bevy::app::{App, PluginGroup};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, Res, ResMut, Startup, WindowPlugin};
use bevy::window::{ExitCondition, Window, WindowMode};
use bevy_kira_audio::AudioPlugin;
use config::*;

use crate::menu::MenuPlugin;
use crate::mode_1p::Mode1PPlugin;
use crate::mode_2p::Mode2PPlugin;
use crate::mode_wall::ModeWallPlugin;
use crate::state::GameState;

fn setup_system(
    mut commands: Commands,
    mut config: ResMut<Config>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

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
    // TODO fixed update

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Config>()
        .add_systems(Startup, setup_system)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong".to_string(),
                    resolution: (768., 576.).into(),
                    resizable: false,
                    mode: WindowMode::Windowed,
                    ..Default::default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: false,
            })
        )
        .add_plugins(AudioPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(Mode1PPlugin)
        .add_plugins(Mode2PPlugin)
        .add_plugins(ModeWallPlugin)
        .add_state::<GameState>()
        .run();
}
