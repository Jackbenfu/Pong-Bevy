use bevy::prelude::{Handle, Resource};
use bevy::text::Font;
use crate::{Color};
use bevy_kira_audio::{AudioSource};

#[derive(Default, Resource)]
pub struct Config {
    pub game_paddle_speed: f32,
    pub game_ball_speed_min: f32,
    pub game_ball_speed_max: f32,
    pub game_ball_speed_incr: f32,
    pub game_ball_oob_x: f32,
    pub game_1v1_score_to_win: u32,

    pub sprite_unit_size: f32,

    pub color_transparent: Color,
    pub color_white: Color,
    pub color_grey: Color,
    pub color_yellow: Color,
    pub color_green: Color,
    pub color_red: Color,

    pub font: Handle<Font>,

    pub audio_paddle_left: Handle<AudioSource>,
    pub audio_paddle_right: Handle<AudioSource>,
    pub audio_wall: Handle<AudioSource>,
}
