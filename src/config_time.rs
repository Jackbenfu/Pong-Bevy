pub struct TimeConfig {
    pub delta_time: f32,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            delta_time: 1. / 60.,
        }
    }
}
