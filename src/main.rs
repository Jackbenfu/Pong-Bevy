use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::WindowMode;

const TIME_STEP: f32 = 1.0 / 60.0;

struct Resolution {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Component)]
struct Wall {}

fn setup(
    mut commands: Commands,
    resolution: Res<Resolution>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let sprite_unit_size: f32 = 16.;

    // top wall
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., resolution.height / 2. - sprite_unit_size / 2., 0.),
                scale: Vec3::new(resolution.width, sprite_unit_size, 0.),
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
                translation: Vec3::new(0., -resolution.height / 2. + sprite_unit_size / 2., 0.),
                scale: Vec3::new(resolution.width, sprite_unit_size, 0.),
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
                translation: Vec3::new(-resolution.width / 2. + sprite_unit_size / 2. + sprite_unit_size, 0., 0.),
                scale: Vec3::new(sprite_unit_size, sprite_unit_size * 4., 0.),
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
                translation: Vec3::new(resolution.width / 2. - sprite_unit_size / 2. - sprite_unit_size, 0., 0.),
                scale: Vec3::new(sprite_unit_size, sprite_unit_size * 4., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        });

    // net
    for y in [240., 192., 144., 96., 48., 0., -48., -96., -144., -192., -240.] {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., y, 0.),
                    scale: Vec3::new(sprite_unit_size, sprite_unit_size * 2., 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            });
    }

    // ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-sprite_unit_size * 4., 0., 0.),
                scale: Vec3::new(sprite_unit_size, sprite_unit_size, 0.),
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
    wall_query: Query<(&Wall, &Transform), Without<Paddle>>,
    mut query: Query<(&Paddle, &mut Transform), Without<Wall>>,
) {
    let (paddle, mut transform) = query.single_mut();
    let mut direction = 0.;

    for (_, wall_transform) in wall_query.iter() {
        let collision = collide(
            wall_transform.translation,
            wall_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );

        if collision.is_some() {
            debug!("{:?}", collision);
            return;
        }
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::X) {
        direction -= 1.;
    }

    let translation = &mut transform.translation;
    translation.y += direction * paddle.speed * TIME_STEP;
}

fn main() {
    let resolution_width: f32 = 768.;
    let resolution_height: f32 = 576.;

    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "wgpu=warn,bevy_ecs=info".to_string(),
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Resolution { width: resolution_width, height: resolution_height })
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: resolution_width,
            height: resolution_height,
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
