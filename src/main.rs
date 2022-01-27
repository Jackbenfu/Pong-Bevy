use bevy::prelude::*;
use bevy::window::WindowMode;

// Timing
const TIME_STEP: f32 = 1. / 60.;

// Display
const SCREEN_WIDTH: f32 = 768.;
const SCREEN_HEIGHT: f32 = 576.;
const SPRITE_UNIT_SIZE: f32 = 16.;

#[derive(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Component)]
struct Wall {}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // top wall
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., SCREEN_HEIGHT / 2. - SPRITE_UNIT_SIZE / 2., 0.),
                scale: Vec3::new(SCREEN_WIDTH, SPRITE_UNIT_SIZE, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall {});

    // bottom wall
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -SCREEN_HEIGHT / 2. + SPRITE_UNIT_SIZE / 2., 0.),
                scale: Vec3::new(SCREEN_WIDTH, SPRITE_UNIT_SIZE, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall {});

    // left paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-SCREEN_WIDTH / 2. + SPRITE_UNIT_SIZE / 2. + SPRITE_UNIT_SIZE, 0., 0.),
                scale: Vec3::new(SPRITE_UNIT_SIZE, SPRITE_UNIT_SIZE * 4., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle { speed: 400. });

    // right paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(SCREEN_WIDTH / 2. - SPRITE_UNIT_SIZE / 2. - SPRITE_UNIT_SIZE, 0., 0.),
                scale: Vec3::new(SPRITE_UNIT_SIZE, SPRITE_UNIT_SIZE * 4., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        });

    // net
    let mut y: f32 = 0.;
    while y < SCREEN_HEIGHT / 2. {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., y, 0.),
                    scale: Vec3::new(SPRITE_UNIT_SIZE, SPRITE_UNIT_SIZE * 2., 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            });

        if y != 0. {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0., -y, 0.),
                        scale: Vec3::new(SPRITE_UNIT_SIZE, SPRITE_UNIT_SIZE * 2., 0.),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }

        y += SPRITE_UNIT_SIZE * 3.;
    }

    // ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-SPRITE_UNIT_SIZE * 4., 0., 0.),
                scale: Vec3::new(SPRITE_UNIT_SIZE, SPRITE_UNIT_SIZE, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb_u8(221, 173, 29),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn left_paddle_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    let (paddle, mut transform) = query.single_mut();
    let mut direction = 0.;

    if keyboard_input.pressed(KeyCode::S) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::X) {
        direction -= 1.;
    }

    let y_bound = SCREEN_HEIGHT / 2. - SPRITE_UNIT_SIZE - transform.scale.y / 2.;

    let translation = &mut transform.translation;
    translation.y += direction * paddle.speed * TIME_STEP;
    translation.y = translation.y.min(y_bound).max(-y_bound);
}

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "info,pong_bevy=debug".to_string(),
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(left_paddle_move_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
