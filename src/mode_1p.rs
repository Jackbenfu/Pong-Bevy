use bevy::{
    prelude::*,
    sprite::collide_aabb::*,
};

use rand::*;

use crate::state::*;
use crate::helpers_system::*;
use crate::helpers_sprite::*;
use crate::{Config};

struct GameOverEvent(Side);

#[derive(Clone, Copy, PartialEq)]
enum Side {
    Left,
    Right,
}

struct GameData {
    left_score: u32,
    right_score: u32,
    starting_side: Side,
    winner: Option<Side>,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            left_score: u32::default(),
            right_score: u32::default(),
            starting_side: Side::Left,
            winner: None,
        }
    }
}

#[derive(Component)]
struct Mode1PEntity {}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct AI {
    velocity: Vec3,
}

#[derive(Component)]
struct Ball {
    speed: f32,
    velocity: Vec3,
}

#[derive(Component)]
enum Collider {
    Paddle,
    Wall,
}

#[derive(Component)]
struct Service {}

#[derive(Component)]
struct Instruction {}

#[derive(Component)]
struct LeftScore {}

#[derive(Component)]
struct RightScore {}

pub struct Mode1PPlugin;

impl Plugin for Mode1PPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::Mode1P;

        app
            .init_resource::<GameData>()
            .add_event::<GameOverEvent>()
            .add_system_set(
                SystemSet::on_enter(GAME_STATE)
                    .with_system(setup_game_data_system.label("setup_game_data"))
                    .with_system(setup_court_system)
                    .with_system(setup_scores_system)
                    .with_system(setup_instructions_system)
                    .with_system(setup_player_system.label("setup_player").after("setup_game_data"))
                    .with_system(setup_ai_system.label("setup_player").after("setup_game_data"))
                    .with_system(setup_ball_system.after("setup_player"))
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(service_system)
                    .with_system(launch_ball_system)
                    .with_system(move_player_system)
                    .with_system(move_ball_system.label("move_ball"))
                    .with_system(move_ai_system.after("move_ball"))
                    .label("move")
                    .before("back")
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(check_ball_collision_system)
                    .with_system(check_ball_out_system)
                    .with_system(game_over_system)
                    .after("move")
                    .before("back")
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(back_to_menu_system)
                    .label("back")
            )
            .add_system_set(
                SystemSet::on_exit(GAME_STATE)
                    .with_system(cleanup_entities::<Mode1PEntity>)
            );

        // TODO stop AI moves when game over
        // TODO remove warning duplicated code
        // TODO rename Player with LeftPlayer and AI with RightPlayer
        // TODO global fixed FPS (see tip with a system from github thread)
        // TODO vsync ?
    }
}

fn setup_game_data_system(
    mut game_data: ResMut<GameData>,
) {
    game_data.left_score = 0;
    game_data.right_score = 0;
    game_data.starting_side = if random::<u32>() % 2 == 0 { Side::Left } else { Side::Right };
    game_data.winner = None;
}

fn setup_court_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    let color = config.color_white;
    let unit_size = config.sprite_unit_size;

    commands
        .spawn_bundle(create_top_wall_sprite(window.width, window.height, unit_size, color))
        .insert(Mode1PEntity {})
        .insert(Collider::Wall);

    commands
        .spawn_bundle(create_bottom_wall_sprite(window.width, window.height, unit_size, color))
        .insert(Mode1PEntity {})
        .insert(Collider::Wall);

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

fn setup_scores_system(
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
                "0",
                TextStyle {
                    font: config.font.clone(),
                    font_size: 57.,
                    color: config.color_white,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(Mode1PEntity {})
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
        .insert(Mode1PEntity {});

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
                "0",
                TextStyle {
                    font: config.font.clone(),
                    font_size: 57.,
                    color: config.color_white,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(Mode1PEntity {})
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
        .insert(Mode1PEntity {});
}

fn setup_instructions_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    // Goal left
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
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_yellow,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // Goal right
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
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_grey,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // Command left
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
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_yellow,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // Command right
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
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_grey,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // Launch left
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
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_yellow,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});

    // Launch right
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
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_grey,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Mode1PEntity {})
        .insert(Instruction {});
}

fn setup_player_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
    game_data: Res<GameData>,
) {
    let entity = commands
        .spawn_bundle(create_left_paddle_sprite(window.width, config.sprite_unit_size, config.color_white))
        .insert(Mode1PEntity {})
        .insert(Player { speed: config.game_paddle_speed })
        .insert(Collider::Paddle)
        .id();

    if game_data.starting_side == Side::Left {
        commands.entity(entity).insert(Service {});
    }
}

fn setup_ai_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
    game_data: Res<GameData>,
) {
    let entity = commands
        .spawn_bundle(create_right_paddle_sprite(window.width, config.sprite_unit_size, config.color_white))
        .insert(Mode1PEntity {})
        .insert(AI { velocity: Vec3::default() })
        .insert(Collider::Paddle)
        .id();

    if game_data.starting_side == Side::Right {
        commands.entity(entity).insert(Service {});
    }
}

fn setup_ball_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    commands
        .spawn_bundle(create_ball_sprite(config.sprite_unit_size, config.color_yellow))
        .insert(Mode1PEntity {})
        .insert(Ball { speed: config.game_ball_speed_min, velocity: Vec3::default() });
}

fn service_system(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    paddle_query: Query<(&Transform, Option<&Player>, Option<&AI>), (With<Service>, Without<Ball>)>,
) {
    let paddle = paddle_query.get_single();
    if !paddle.is_ok() {
        // No serving paddle
        return;
    }

    let (paddle_transform, player, ai) = paddle.unwrap();
    let mut ball_transform = ball_query.single_mut();

    ball_transform.translation.y = paddle_transform.translation.y;

    if let Some(_) = player {
        ball_transform.translation.x = paddle_transform.translation.x + paddle_transform.scale.x + 2.;
    } else if let Some(_) = ai {
        ball_transform.translation.x = paddle_transform.translation.x - paddle_transform.scale.x - 2.;
    }
}

fn launch_ball_system(
    mut commands: Commands,
    mut ball_query: Query<&mut Ball, With<Ball>>,
    paddle_query: Query<(Entity, Option<&Player>, Option<&AI>), With<Service>>,
    instructions_query: Query<Entity, With<Instruction>>,
    keyboard: Res<Input<KeyCode>>,
    game_data: Res<GameData>,
) {
    if game_data.winner.is_some() {
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

    commands.entity(paddle_entity).remove::<Service>();

    // Also hide instructions
    for instruction_entity in instructions_query.iter() {
        commands.entity(instruction_entity).despawn_recursive();
    }
}

fn move_player_system(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let mut direction = 0.;

    if keyboard.pressed(KeyCode::S) {
        direction += 1.;
    }

    if keyboard.pressed(KeyCode::X) {
        direction -= 1.;
    }

    let (player_entity, mut player_transform) = player_query.single_mut();
    let bound_y = window.height / 2. - config.sprite_unit_size - player_transform.scale.y / 2.;

    let translation = &mut player_transform.translation;
    translation.y += direction * player_entity.speed * time.delta_seconds();
    translation.y = translation.y.min(bound_y).max(-bound_y);
}

fn move_ball_system(
    mut ball_query: Query<(&Ball, &mut Transform)>,
    time: Res<Time>,
) {
    let (ball, mut transform) = ball_query.single_mut();
    let velocity = ball.velocity;

    if velocity.x != 0. || velocity.y != 0. {
        transform.translation += velocity * time.delta_seconds();
    }
}

fn move_ai_system(
    mut ai_query: Query<(&mut AI, &mut Transform), Without<Ball>>,
    ball_query: Query<(&Ball, &Transform)>,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let (ball, ball_transform) = ball_query.single();
    let (mut ai_entity, mut ai_transform) = ai_query.single_mut();

    if ball.velocity.x <= 0. {
        ai_entity.velocity.y = 0.;
        return;
    }

    if ball_transform.translation.x > window.width / 2. {
        ai_entity.velocity.y = 0.;
        return;
    }

    let ball_y = ball_transform.translation.y;
    let ai_translation_y = ai_transform.translation.y;
    let dist: f32 = (ai_translation_y - ball_y).abs();
    let rnd = random::<u32>() % (ai_transform.scale.y) as u32 + 1;

    if dist < rnd as f32 {
        ai_entity.velocity.y = 0.;
        return;
    }

    let dir = if ball_y > ai_translation_y { config.game_paddle_speed } else { -config.game_paddle_speed };
    let bound_y = window.height / 2. - config.sprite_unit_size - ai_transform.scale.y / 2.;

    ai_transform.translation.y += dir * time.delta_seconds();
    ai_transform.translation.y = ai_transform.translation.y.min(bound_y).max(-bound_y);
}

fn check_ball_collision_system(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    collider_query: Query<(&Transform, &Collider)>,
    config: Res<Config>,
) {
    let (mut ball, ball_transform) = ball_query.single_mut();

    for (collider_transform, collider) in collider_query.iter() {
        if let Some(_) = collide(
            ball_transform.translation,
            ball_transform.scale.truncate(),
            collider_transform.translation,
            collider_transform.scale.truncate(),
        ) {
            match *collider {
                Collider::Paddle => {
                    let hit_factor = (ball_transform.translation.y - collider_transform.translation.y) / collider_transform.scale.y;

                    let mut new_ball_vel = Vec3::default();
                    new_ball_vel.x = if ball.velocity.x > 0. { -1. } else { 1. };
                    new_ball_vel.y = hit_factor * 2.;
                    new_ball_vel = new_ball_vel.normalize();

                    ball.velocity.x = new_ball_vel.x * ball.speed;
                    ball.velocity.y = new_ball_vel.y * ball.speed;

                    if config.game_ball_speed_max > ball.speed {
                        ball.speed += config.game_ball_speed_incr;
                    }
                }
                Collider::Wall => {
                    ball.velocity.y = -ball.velocity.y;
                }
            }

            break;
        }
    }
}

fn check_ball_out_system(
    mut commands: Commands,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    mut left_player_query: Query<(Entity, &mut Transform), (With<Player>, Without<AI>, Without<Ball>)>,
    mut right_player_query: Query<(Entity, &mut Transform), (With<AI>, Without<Player>, Without<Ball>)>,
    mut left_score_query: Query<&mut Text, (With<LeftScore>, Without<RightScore>)>,
    mut right_score_query: Query<&mut Text, (With<RightScore>, Without<LeftScore>)>,
    mut game_data: ResMut<GameData>,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    if game_data.winner.is_some() {
        return;
    }

    let mut ball_out = false;

    let (mut ball, ball_transform) = ball_query.single_mut();
    let (right_player_entity, mut right_player_transform) = right_player_query.single_mut();
    let (left_player_entity, mut left_player_transform) = left_player_query.single_mut();

    if ball_transform.translation.x < -window.width / 2. - config.game_ball_oob_x {
        game_data.right_score += 1;

        if game_data.right_score == config.game_score_to_win {
            game_data.winner = Some(Side::Right);
        } else {
            commands.entity(right_player_entity).insert(Service {});
        }

        right_score_query.single_mut().sections[0].value = format!("{}", game_data.right_score);
        ball_out = true;
    } else if ball_transform.translation.x > window.width / 2. + config.game_ball_oob_x {
        game_data.left_score += 1;

        if game_data.left_score == config.game_score_to_win {
            game_data.winner = Some(Side::Left);
        } else {
            commands.entity(left_player_entity).insert(Service {});
        }

        left_score_query.single_mut().sections[0].value = format!("{}", game_data.left_score);
        ball_out = true;
    }

    match &game_data.winner {
        None => {
            if ball_out {
                left_player_transform.translation.y = 0.;
                right_player_transform.translation.y = 0.;
                ball.velocity = Vec3::default();
            }
        }
        Some(side) => {
            game_over_event.send(GameOverEvent(*side))
        }
    }
}

fn game_over_system(
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
                color: Color::rgba(0., 0., 0., 0.).into(),
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
            .insert(Mode1PEntity {});

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
                color: Color::rgba(0., 0., 0., 0.).into(),
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
            .insert(Mode1PEntity {});

        break; // We only take the first event
    }
}
