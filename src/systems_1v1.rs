use bevy::prelude::*;
use rand::*;

use crate::config::*;
use crate::components::*;
use crate::helpers_sprite::*;

/// Resets game data to default values.
pub fn reset_game_data_system(
    mut game_data: ResMut<GameData>,
) {
    game_data.left_score = 0;
    game_data.right_score = 0;
    game_data.starting_side = if random::<u32>() % 2 == 0 { Side::Left } else { Side::Right };
    game_data.game_over = None;
}

/// Builds the court for 1v1 modes (1P and 2P).
pub fn setup_court_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    let color = config.color_white;
    let unit_size = config.sprite_unit_size;

    commands
        .spawn_bundle(create_top_wall_sprite(window.width, window.height, unit_size, color))
        .insert(GameModeEntity {})
        .insert(Collider::Wall);

    commands
        .spawn_bundle(create_bottom_wall_sprite(window.width, window.height, unit_size, color))
        .insert(GameModeEntity {})
        .insert(Collider::Wall);

    // Net
    {
        commands
            .spawn_bundle(create_net_sprite(0., unit_size, color))
            .insert(GameModeEntity {});

        let mut y: f32 = unit_size * 3.;
        while y < window.height / 2. {
            commands
                .spawn_bundle(create_net_sprite(y, unit_size, color))
                .insert(GameModeEntity {});

            commands
                .spawn_bundle(create_net_sprite(-y, unit_size, color))
                .insert(GameModeEntity {});

            y += unit_size * 3.;
        }
    }
}

/// Builds the scores for 1v1 modes (1P and 2P).
pub fn setup_scores_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    // Left score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(310.),
                    top: Val::Px(48.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                format!("{}", 0),
                TextStyle {
                    font: config.font.clone(),
                    font_size: 57.,
                    color: config.color_white,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(GameModeEntity {})
        .insert(LeftScore {});

    // Left score max
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(330.),
                    top: Val::Px(88.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                format!("/{}", config.game_score_to_win),
                TextStyle {
                    font: config.font.clone(),
                    font_size: 21.,
                    color: config.color_grey,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(GameModeEntity {});

    // Right score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(426.),
                    top: Val::Px(48.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                format!("{}", 0),
                TextStyle {
                    font: config.font.clone(),
                    font_size: 57.,
                    color: config.color_white,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(GameModeEntity {})
        .insert(RightScore {});

    // Right score max
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(450.),
                    top: Val::Px(88.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                format!("/{}", config.game_score_to_win),
                TextStyle {
                    font: config.font.clone(),
                    font_size: 21.,
                    color: config.color_grey,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(GameModeEntity {});
}

pub fn setup_left_paddle_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
    game_data: Res<GameData>,
) {
    let entity = commands
        .spawn_bundle(create_left_paddle_sprite(window.width, config.sprite_unit_size, config.color_white))
        .insert(GameModeEntity {})
        .insert(LeftPaddle { speed: config.game_paddle_speed })
        .insert(Collider::Paddle)
        .id();

    if game_data.starting_side == Side::Left {
        commands.entity(entity).insert(Service {});
    }
}

pub fn setup_right_paddle_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
    game_data: Res<GameData>,
) {
    let entity = commands
        .spawn_bundle(create_right_paddle_sprite(window.width, config.sprite_unit_size, config.color_white))
        .insert(GameModeEntity {})
        .insert(RightPaddle { velocity: Vec3::default(), speed: config.game_paddle_speed })
        .insert(Collider::Paddle)
        .id();

    if game_data.starting_side == Side::Right {
        commands.entity(entity).insert(Service {});
    }
}

pub fn setup_ball_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    commands
        .spawn_bundle(create_ball_sprite(config.sprite_unit_size, config.color_yellow))
        .insert(GameModeEntity {})
        .insert(Ball { speed: config.game_ball_speed_min, velocity: Vec3::default() });
}

pub fn service_system(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    paddle_query: Query<(&Transform, Option<&LeftPaddle>, Option<&RightPaddle>), (With<Service>, Without<Ball>)>,
) {
    let paddle = paddle_query.get_single();
    if !paddle.is_ok() {
        // No serving paddle
        return;
    }

    let (paddle_transform, left_paddle, right_paddle) = paddle.unwrap();
    let mut ball_transform = ball_query.single_mut();

    ball_transform.translation.y = paddle_transform.translation.y;

    if let Some(_) = left_paddle {
        ball_transform.translation.x = paddle_transform.translation.x + paddle_transform.scale.x + 2.;
    } else if let Some(_) = right_paddle {
        ball_transform.translation.x = paddle_transform.translation.x - paddle_transform.scale.x - 2.;
    }
}

pub fn launch_ball_system(
    mut commands: Commands,
    mut ball_query: Query<&mut Ball, With<Ball>>,
    paddle_query: Query<(Entity, Option<&LeftPaddle>, Option<&RightPaddle>), With<Service>>,
    instructions_query: Query<Entity, With<Instruction>>,
    keyboard: Res<Input<KeyCode>>,
    game_data: Res<GameData>,
) {
    if game_data.game_over.is_some() {
        return;
    }

    if !keyboard.just_released(KeyCode::Space) {
        return;
    }

    let paddle = paddle_query.get_single();
    if paddle.is_err() {
        // Already launched?
        return;
    }

    let (paddle_entity, left_paddle, right_paddle) = paddle.unwrap();
    let mut ball = ball_query.single_mut();

    if random::<i32>() % 2 == 0 {
        ball.velocity.y = -0.25;
    } else {
        ball.velocity.y = 0.25;
    }

    if let Some(_) = left_paddle {
        ball.velocity.x = 1.;
    } else if let Some(_) = right_paddle {
        ball.velocity.x = -1.;
    }

    ball.velocity = ball.velocity.normalize();
    ball.velocity.x *= ball.speed;
    ball.velocity.y *= ball.speed;

    commands.entity(paddle_entity).remove::<Service>();

    // Also hide instructions
    for instruction_entity in instructions_query.iter() {
        commands.entity(instruction_entity).despawn_recursive();
    }
}

pub fn game_over_system(
    mut commands: Commands,
    mut game_over_event: EventReader<GameOverEvent>,
    config: Res<Config>,
) {
    const WIN_TEXT: &str = "WIN";
    const LOSE_TEXT: &str = "LOSE";

    for event in game_over_event.iter() {
        let left_text: &str;
        let right_text: &str;
        let left_color: Color;
        let right_color: Color;

        match event.0 {
            Side::Left => {
                left_text = WIN_TEXT;
                right_text = LOSE_TEXT;
                left_color = config.color_green;
                right_color = config.color_red;
            }
            Side::Right => {
                left_text = LOSE_TEXT;
                right_text = WIN_TEXT;
                left_color = config.color_red;
                right_color = config.color_green;
            }
        }

        // Left
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(352.), Val::Px(48.)),
                    position: Rect {
                        bottom: Val::Px(186.),
                        left: Val::Px(0.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: config.color_transparent.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        left_text,
                        TextStyle {
                            font: config.font.clone(),
                            font_size: 66.,
                            color: left_color,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            })
            .insert(GameModeEntity {});

        // Right
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(352.), Val::Px(48.)),
                    position: Rect {
                        bottom: Val::Px(186.),
                        left: Val::Px(416.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: config.color_transparent.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        right_text,
                        TextStyle {
                            font: config.font.clone(),
                            font_size: 66.,
                            color: right_color,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            })
            .insert(GameModeEntity {});

        break; // We only take the first event
    }
}
