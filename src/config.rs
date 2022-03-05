use crate::{Color, Font, Handle};

pub struct Config {
    pub game_paddle_speed: f32,
    pub game_ball_speed_min: f32,
    pub game_ball_speed_max: f32,
    pub game_ball_speed_incr: f32,
    pub game_ball_oob_x: f32,

    pub game_score_to_win: u32,

    pub sprite_unit_size: f32,

    pub color_transparent: Color,
    pub color_white: Color,
    pub color_grey: Color,
    pub color_yellow: Color,
    pub color_green: Color,
    pub color_red: Color,

    pub font: Handle<Font>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_paddle_speed: f32::default(),
            game_ball_speed_min: f32::default(),
            game_ball_speed_max: f32::default(),
            game_ball_speed_incr: f32::default(),
            game_ball_oob_x: f32::default(),
            game_score_to_win: u32::default(),
            sprite_unit_size: f32::default(),
            color_transparent: Color::default(),
            color_white: Color::default(),
            color_grey: Color::default(),
            color_yellow: Color::default(),
            color_green: Color::default(),
            color_red: Color::default(),
            font: Handle::default(),
        }
    }
}
