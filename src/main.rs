mod config_time;
mod config_game;
mod resources;
mod utils;
mod state;
mod menu;
mod mode_1p;
mod mode_2p;
mod mode_wall;

use bevy::prelude::*;
use config_time::*;
use resources::*;
use menu::*;
use mode_1p::*;
use mode_2p::*;
use mode_wall::*;

/*
#[derive(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Component)]
struct Wall {}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct AI {
    velocity: Vec3,
}

fn setup_system(mut commands: Commands) {
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

    // player paddle
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
        .insert(Player {})
        .insert(Paddle { speed: PADDLE_SPEED });

    // ai paddle
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
        })
        .insert(AI {
            velocity: Vec3::default(),

        })
        .insert(Paddle { speed: PADDLE_SPEED });

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
        })
        .insert(Ball {
            velocity: Vec3::default(),
            speed: PADDLE_SPEED,
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
}

fn player_paddle_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Paddle, &mut Transform), With<Player>>,
) {
    let (player_paddle, mut player_transform) = player_query.single_mut();
    let mut direction = 0.;

    if keyboard_input.pressed(KeyCode::S) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::X) {
        direction -= 1.;
    }

    let bound_y = SCREEN_HEIGHT / 2. - SPRITE_UNIT_SIZE - player_transform.scale.y / 2.;

    let translation = &mut player_transform.translation;
    translation.y += direction * player_paddle.speed * TIME_STEP;
    translation.y = translation.y.min(bound_y).max(-bound_y);
}

fn ai_paddle_move_system(
    ball_query: Query<(&Ball, &Transform)>,
    mut ai_query: Query<(&mut AI, &mut Transform), Without<Ball>>,
) {
    let (ball, ball_transform) = ball_query.single();
    let (mut ai_paddle, mut ai_transform) = ai_query.single_mut();

    let ball_velocity = ball.velocity;

    if ball_velocity.x < 0. {
        ai_paddle.velocity.y = 0.;
        ai_transform.translation += Vec3::default();
        return;
    }

    let ball_y = ball_transform.translation.y;
    let ai_paddle_y = ai_transform.translation.y;
    let dist = (ai_paddle_y - ball_y).abs();
    let rnd: f32 = random::<f32>() % ai_transform.scale.y + 1.;

    if dist < rnd {
        ai_paddle.velocity.y = 0.;
        ai_transform.translation += Vec3::default();
        return;
    }

    let dir = if ball_y > ai_paddle_y { PADDLE_SPEED } else { -PADDLE_SPEED };
    ai_paddle.velocity.y = dir;

    let bound_y = SCREEN_HEIGHT / 2. - SPRITE_UNIT_SIZE - ai_transform.scale.y / 2.;

    ai_transform.translation += ai_paddle.velocity * TIME_STEP;
    ai_transform.translation.y = ai_transform.translation.y.min(bound_y).max(-bound_y);
}

fn ball_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut ball_query: Query<(&Ball, &mut Transform)>,
) {
    let (ball, mut transform) = ball_query.single_mut();
    let mut direction = 0.;

    if keyboard_input.pressed(KeyCode::P) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::L) {
        direction -= 1.;
    }

    let y_bound = SCREEN_HEIGHT / 2. - SPRITE_UNIT_SIZE - transform.scale.y / 2.;

    let translation = &mut transform.translation;
    translation.y += direction * ball.speed * TIME_STEP;
    translation.y = translation.y.min(y_bound).max(-y_bound);
}
*/

fn setup_system(
    mut commands: Commands,
    mut resources: ResMut<Resources>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    resources.font = asset_server.load("fonts/Volter__28Goldfish_29.ttf");
}

fn main() {
    #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

    App::new()
        .init_resource::<Resources>()
        .init_resource::<TimeConfig>()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "info,pong_bevy=debug".to_string(),
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
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(Mode1PPlugin)
        .add_plugin(Mode2PPlugin)
        .add_plugin(ModeWallPlugin)
        .run();

    /*.add_startup_system(setup_system)
    .add_system(player_paddle_move_system)
    .add_system(ball_move_system)
    .add_system(ai_paddle_move_system)*/
}
