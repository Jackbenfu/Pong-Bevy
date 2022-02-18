use bevy::prelude::*;
use rand::*;
use crate::state::*;
use crate::resources::*;
use crate::helpers_system::*;
use crate::helpers_sprite::*;
use crate::{GameConfig, SpriteConfig};

#[derive(Component)]
struct Mode1PEntity {}

#[derive(Component)]
struct Wall {}

#[derive(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct AI {
    velocity: Vec3,
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Instruction {}

#[derive(Component)]
struct Serving {}

pub struct Mode1PPlugin;

impl Plugin for Mode1PPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::Mode1P;

        app
            .add_system_set(
                SystemSet::on_enter(GAME_STATE)
                    .with_system(setup_court_system)
                    .with_system(setup_player_system)
                    .with_system(setup_ai_system)
                    .with_system(setup_ball_system)
                    .with_system(setup_scores_system)
                    .with_system(setup_instructions_system)
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(back_to_menu_system)
                    .with_system(hide_instructions_system)
                    .with_system(set_starting_player_system)
                    .with_system(stick_ball_to_serving_paddle_system)
                    .with_system(launch_ball_system)
                    .with_system(move_player_system)
                    .with_system(move_ai_system)
                    .with_system(move_ball_system)
            )
            .add_system_set(
                SystemSet::on_exit(GAME_STATE)
                    .with_system(cleanup_system::<Mode1PEntity>)
            );
    }
}

fn setup_court_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    sprite_config: Res<SpriteConfig>,
) {
    let color = sprite_config.color_white;
    let unit_size = sprite_config.unit_size;

    commands
        .spawn_bundle(create_top_wall_sprite(window.width, window.height, unit_size, color))
        .insert(Mode1PEntity {})
        .insert(Wall {});

    commands
        .spawn_bundle(create_bottom_wall_sprite(window.width, window.height, unit_size, color))
        .insert(Mode1PEntity {})
        .insert(Wall {});

    // net
    {
        commands
            .spawn_bundle(create_net_sprite(0., unit_size, color))
            .insert(Mode1PEntity {});

        let mut y: f32 = unit_size * 3.;
        while y < window.height / 2. {
            commands
                .spawn_bundle(create_net_sprite(y, unit_size, color))
                .insert(Mode1PEntity {});

            commands
                .spawn_bundle(create_net_sprite(-y, unit_size, color))
                .insert(Mode1PEntity {});

            y += unit_size * 3.;
        }
    }
}

fn setup_player_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    sprite_config: Res<SpriteConfig>,
    game_config: Res<GameConfig>,
) {
    commands
        .spawn_bundle(create_left_paddle_sprite(window.width, sprite_config.unit_size, sprite_config.color_white))
        .insert(Mode1PEntity {})
        .insert(Paddle { speed: game_config.paddle_speed })
        .insert(Player {});
}

fn setup_ai_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    sprite_config: Res<SpriteConfig>,
    game_config: Res<GameConfig>,
) {
    commands
        .spawn_bundle(create_right_paddle_sprite(window.width, sprite_config.unit_size, sprite_config.color_white))
        .insert(Mode1PEntity {})
        .insert(Paddle { speed: game_config.paddle_speed })
        .insert(AI { velocity: Vec3::default() });
}

fn setup_ball_system(
    mut commands: Commands,
    sprite_config: Res<SpriteConfig>,
    game_config: Res<GameConfig>,
) {
    commands
        .spawn_bundle(create_ball_sprite(sprite_config.unit_size, sprite_config.color_yellow))
        .insert(Mode1PEntity {})
        .insert(Ball { velocity: Vec3::default(), speed: game_config.paddle_speed });
}

fn setup_scores_system(
    mut commands: Commands,
    resources: Res<Resources>,
    sprite_config: Res<SpriteConfig>,
) {
    // left score
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
                "0",
                TextStyle {
                    font: resources.font.clone(),
                    font_size: 57.,
                    color: sprite_config.color_white,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(Mode1PEntity {});

    // left score max
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
                "/9",
                TextStyle {
                    font: resources.font.clone(),
                    font_size: 21.,
                    color: sprite_config.color_grey,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(Mode1PEntity {});

    // right score
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
                "0",
                TextStyle {
                    font: resources.font.clone(),
                    font_size: 57.,
                    color: sprite_config.color_white,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(Mode1PEntity {});

    // right score max
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
                "/9",
                TextStyle {
                    font: resources.font.clone(),
                    font_size: 21.,
                    color: sprite_config.color_grey,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(Mode1PEntity {});
}

fn setup_instructions_system(
    mut commands: Commands,
    resources: Res<Resources>,
    sprite_config: Res<SpriteConfig>,
) {
    // goal left
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: Rect {
                    bottom: Val::Px(192.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Goal",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 30.,
                        color: sprite_config.color_yellow,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // goal right
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: Rect {
                    bottom: Val::Px(192.),
                    left: Val::Px(416.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "beat the AI!",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 30.,
                        color: sprite_config.color_grey,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // command left
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: Rect {
                    bottom: Val::Px(144.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "S or X",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 30.,
                        color: sprite_config.color_yellow,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // command right
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: Rect {
                    bottom: Val::Px(144.),
                    left: Val::Px(416.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "move left paddle",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 30.,
                        color: sprite_config.color_grey,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // launch left
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: Rect {
                    bottom: Val::Px(96.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "SPACEBAR",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 30.,
                        color: sprite_config.color_yellow,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // launch right
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: Rect {
                    bottom: Val::Px(96.),
                    left: Val::Px(416.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "launch the ball",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 30.,
                        color: sprite_config.color_grey,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});
}

fn set_starting_player_system(
    mut commands: Commands,
    player_query: Query<Entity, Added<Player>>,
    ai_query: Query<Entity, Added<AI>>,
) {
    if random::<i32>() % 2 == 0 {
        let player = player_query.get_single();
        if player.is_ok() {
            commands.entity(player.unwrap()).insert(Serving {});
        }
    } else {
        let ai = ai_query.get_single();
        if ai.is_ok() {
            commands.entity(ai.unwrap()).insert(Serving {});
        }
    }
}

fn stick_ball_to_serving_paddle_system(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    paddle_query: Query<(&Transform, Option<&Player>, Option<&AI>), (With<Serving>, Without<Ball>)>,
) {
    let paddle = paddle_query.get_single();
    if !paddle.is_ok() {
        return;
    }

    let (paddle_transform, player, ai) = paddle.unwrap();
    let mut ball_transform = ball_query.single_mut();

    ball_transform.translation.y = paddle_transform.translation.y;

    if let Some(_) = player {
        ball_transform.translation.x = paddle_transform.translation.x + paddle_transform.scale.x + 2.;
    } else if let Some(_) = ai {
        ball_transform.translation.x = paddle_transform.translation.x - ball_transform.scale.x - 2.;
    }
}

fn launch_ball_system(
    mut commands: Commands,
    mut ball_query: Query<&mut Ball, With<Ball>>,
    paddle_query: Query<(Entity, Option<&Player>, Option<&AI>), With<Serving>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if !keyboard.just_released(KeyCode::Space) {
        return;
    }

    let paddle = paddle_query.get_single();
    if !paddle.is_ok() {
        return;
    }

    let (paddle_entity, player, ai) = paddle.unwrap();
    let mut ball = ball_query.single_mut();

    if random::<i32>() % 2 == 0 {
        ball.velocity.y = -0.25;
    } else {
        ball.velocity.y = 0.25;
    }

    if let Some(_) = player {
        ball.velocity.x = 1.;
    } else if let Some(_) = ai {
        ball.velocity.x = -1.;
    }

    ball.velocity = ball.velocity.normalize();
    ball.velocity.x *= ball.speed;
    ball.velocity.y *= ball.speed;

    commands.entity(paddle_entity).remove::<Serving>();
}

fn hide_instructions_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    query: Query<Entity, With<Instruction>>,
) {
    if !keyboard.just_released(KeyCode::Space) {
        return;
    }

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn move_player_system(
    mut player_query: Query<(&Paddle, &mut Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    sprite_config: Res<SpriteConfig>,
) {
    let (player_paddle, mut player_transform) = player_query.single_mut();
    let mut direction = 0.;

    if keyboard.pressed(KeyCode::S) {
        direction += 1.;
    }

    if keyboard.pressed(KeyCode::X) {
        direction -= 1.;
    }

    let bound_y = window.height / 2. - sprite_config.unit_size - player_transform.scale.y / 2.;

    let translation = &mut player_transform.translation;
    translation.y += direction * player_paddle.speed * time.delta_seconds();
    translation.y = translation.y.min(bound_y).max(-bound_y);
}

fn move_ai_system(
    mut ai_query: Query<(&Paddle, &mut Transform), With<AI>>,
    keyboard: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    sprite_config: Res<SpriteConfig>,
) {
    // TODO change to follow ball
    let (ai_paddle, mut ai_transform) = ai_query.single_mut();
    let mut direction = 0.;

    if keyboard.pressed(KeyCode::P) {
        direction += 1.;
    }

    if keyboard.pressed(KeyCode::L) {
        direction -= 1.;
    }

    let bound_y = window.height / 2. - sprite_config.unit_size - ai_transform.scale.y / 2.;

    let translation = &mut ai_transform.translation;
    translation.y += direction * ai_paddle.speed * time.delta_seconds();
    translation.y = translation.y.min(bound_y).max(-bound_y);
}

fn move_ball_system(
    mut ball_query: Query<(&Ball, &mut Transform)>,
    time: Res<Time>,
) {
    let (ball, mut transform) = ball_query.single_mut();
    let velocity = ball.velocity;

    if velocity.x != 0. || velocity.y != 0. {
        transform.translation.x = transform.translation.x + velocity.x * time.delta_seconds();
        transform.translation.y = transform.translation.y + velocity.y * time.delta_seconds();
    }
}

/*
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

fn setup_results_system(
    mut commands: Commands,
    resources: Res<Resources>,
    sprite_config: Res<SpriteConfig>,
) {
    // win
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "WIN",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 66.,
                        color: sprite_config.color_green,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {});

    // lose
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "LOSE",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 66.,
                        color: sprite_config.color_red,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {});
}
*/
