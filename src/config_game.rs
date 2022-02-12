pub struct GameConfig {
    pub paddle_speed: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            paddle_speed: 400.,
        }
    }
}
