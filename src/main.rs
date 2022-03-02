mod config;
mod helpers_system;
mod helpers_sprite;
mod state;
mod menu;
mod mode_1p;
mod mode_2p;
mod mode_wall;

use bevy::prelude::*;
use config::*;
use menu::*;
use mode_1p::*;
use mode_2p::*;
use mode_wall::*;

use crate::state::GameState;

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

    config.game_score_to_win = 2;

    config.sprite_unit_size = 16.;

    config.color_white = Color::WHITE;
    config.color_grey = Color::rgb(100. / 255., 100. / 255., 100. / 255.);
    config.color_yellow = Color::rgb(221. / 255., 173. / 255., 29. / 255.);
    config.color_green = Color::rgb(69. / 255., 183. / 255., 130. / 255.);
    config.color_red = Color::rgb(196. / 255., 89. / 255., 73. / 255.);

    config.font = asset_server.load("fonts/Volter__28Goldfish_29.ttf");
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
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
        .init_resource::<Config>()
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(Mode1PPlugin)
        .add_plugin(Mode2PPlugin)
        .add_plugin(ModeWallPlugin)
        .add_state(GameState::Mode1P)
        .run();
}
