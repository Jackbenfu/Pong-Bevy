use crate::Color;

pub struct SpriteConfig {
    pub unit_size: f32,
    pub color_white: Color,
    pub color_grey: Color,
    pub color_yellow: Color,
}

impl Default for SpriteConfig {
    fn default() -> Self {
        Self {
            unit_size: 16.,
            color_white: Color::WHITE,
            color_grey: Color::rgb(100. / 255., 100. / 255., 100. / 255.),
            color_yellow: Color::rgb(221. / 255., 173. / 255., 29. / 255.),
        }
    }
}
