use bevy::prelude::*;

pub struct Resources {
    pub font: Handle<Font>,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            font: Handle::default(),
        }
    }
}
