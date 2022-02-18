use bevy::prelude::*;

pub fn create_top_wall_sprite(window_width: f32, window_height: f32, unit_size: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., window_height / 2. - unit_size / 2., 0.),
            scale: Vec3::new(window_width, unit_size, 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_bottom_wall_sprite(window_width: f32, window_height: f32, unit_size: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., -window_height / 2. + unit_size / 2., 0.),
            scale: Vec3::new(window_width, unit_size, 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_left_paddle_sprite(window_width: f32, unit_size: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-window_width / 2. + unit_size / 2. + unit_size, 0., 0.),
            scale: Vec3::new(unit_size, unit_size * 4., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_right_paddle_sprite(window_width: f32, unit_size: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(window_width / 2. - unit_size / 2. - unit_size, 0., 0.),
            scale: Vec3::new(unit_size, unit_size * 4., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_ball_sprite(unit_size: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(unit_size, unit_size, 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_net_sprite(pos_y: f32, unit_size: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., pos_y, 0.),
            scale: Vec3::new(unit_size, unit_size * 2., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}
