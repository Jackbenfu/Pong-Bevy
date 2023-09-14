use bevy::prelude::*;
use rand::*;

use crate::config::*;
use crate::systems_generic::*;
use crate::components::*;
use crate::events::*;
use crate::state::*;
use crate::helpers_sprite::*;

pub struct ModeWallPlugin;

impl Plugin for ModeWallPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::ModeWall;

        app
            .init_resource::<GameData>()
            .add_event::<GameOverEvent>()
            .add_event::<BallOutEvent>()
            .add_event::<BallHitPaddleEvent>()
            .add_system_set(
                SystemSet::on_enter(GAME_STATE)
                    .with_system(reset_game_data_system.label("reset_game_data"))
                    .with_system(setup_court_system)
                    .with_system(setup_score_system)
                    .with_system(setup_instructions_system)
                    .with_system(setup_left_paddle_system.label("setup_paddle").after("reset_game_data"))
                    .with_system(setup_ball_system.after("setup_paddle"))
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(launch_ball_system)
                    .with_system(move_left_paddle_with_keyboard_system)
                    .with_system(move_ball_system.label("move_ball"))
                    .label("move")
                    .before("back")
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(check_ball_collision_system)
                    .with_system(increment_score_system.after("check_ball_out"))
                    .with_system(check_ball_out_system.label("check_ball_out"))
                    .with_system(check_game_over_system.label("check_game_over").after("check_ball_out"))
                    .with_system(game_over_system.after("check_game_over"))
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
                    .with_system(cleanup_entities::<GameModeEntity>)
            );
    }
}

fn reset_game_data_system(
    mut game_data: ResMut<GameData>,
) {
    game_data.left_score = 0;
    game_data.game_over = None;
}

fn setup_court_system(
    mut commands: Commands,
    windows: Res<Windows>,
    config: Res<Config>,
) {
    let window = windows.get_primary().unwrap();
    let color = config.color_white;
    let unit_size = config.sprite_unit_size;

    commands
        .spawn(create_top_wall_sprite(window.width(), window.height(), unit_size, color))
        .insert(GameModeEntity {})
        .insert(SoundEmitter { source: config.audio_wall.clone() })
        .insert(Collider::Wall);

    commands
        .spawn(create_bottom_wall_sprite(window.width(), window.height(), unit_size, color))
        .insert(GameModeEntity {})
        .insert(SoundEmitter { source: config.audio_wall.clone() })
        .insert(Collider::Wall);

    commands
        .spawn(create_right_wall_sprite(window.width(), window.height(), unit_size, color))
        .insert(GameModeEntity {})
        .insert(SoundEmitter { source: config.audio_wall.clone() })
        .insert(Collider::Wall);
}

fn setup_score_system(
    mut commands: Commands,
    windows: Res<Windows>,
    config: Res<Config>,
) {
    let window = windows.get_primary().unwrap();

    // Score
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(416.), Val::Px(64.)),
                position: UiRect {
                    right: Val::Px((window.width() - 416.) / 2.),
                    top: Val::Px(48.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("{}", 0),
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 57.,
                        color: config.color_white,
                    },
                ),
                ..Default::default()
            })
                .insert(GameModeEntity {})
                .insert(LeftScore {});
        });
}

fn setup_instructions_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    // Goal label
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(324.), Val::Px(48.)),
                position: UiRect {
                    top: Val::Px(128.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Goal",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_yellow,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Goal text
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: UiRect {
                    top: Val::Px(128.),
                    left: Val::Px(356.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "score a max!",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_grey,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Left paddle control label
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(324.), Val::Px(48.)),
                position: UiRect {
                    top: Val::Px(176.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "S or X",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_yellow,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Left paddle control text
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: UiRect {
                    top: Val::Px(176.),
                    left: Val::Px(356.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "move the paddle",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_grey,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Launch label
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(324.), Val::Px(48.)),
                position: UiRect {
                    top: Val::Px(226.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "SPACEBAR",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_yellow,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Launch text
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(352.), Val::Px(48.)),
                position: UiRect {
                    top: Val::Px(226.),
                    left: Val::Px(356.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: config.color_transparent.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "launch the ball",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 30.,
                        color: config.color_grey,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameModeEntity {})
        .insert(Instruction {});
}

fn setup_left_paddle_system(
    mut commands: Commands,
    windows: Res<Windows>,
    config: Res<Config>,
) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn(create_left_paddle_sprite(window.width(), config.sprite_unit_size, config.color_white))
        .insert(GameModeEntity {})
        .insert(LeftPaddle { speed: config.game_paddle_speed })
        .insert(SoundEmitter { source: config.audio_paddle_left.clone() })
        .insert(Collider::Paddle)
        .insert(Service {});
}

fn setup_ball_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    commands
        .spawn(create_ball_sprite(config.sprite_unit_size, Vec3::new(-128., -80., 0.), config.color_yellow))
        .insert(GameModeEntity {})
        .insert(Ball { speed: config.game_ball_speed_min, velocity: Vec3::default() });
}

fn launch_ball_system(
    mut commands: Commands,
    mut ball_query: Query<&mut Ball, With<Ball>>,
    paddle_query: Query<Entity, With<Service>>,
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

    let paddle_entity = paddle.unwrap();
    let mut ball = ball_query.single_mut();

    if random::<i32>() % 2 == 0 {
        ball.velocity.y = -0.25;
    } else {
        ball.velocity.y = 0.25;
    }

    ball.velocity.x = 1.;

    ball.velocity = ball.velocity.normalize();
    ball.velocity.x *= ball.speed;
    ball.velocity.y *= ball.speed;

    commands.entity(paddle_entity).remove::<Service>();

    // Also hide instructions
    for instruction_entity in instructions_query.iter() {
        commands.entity(instruction_entity).despawn_recursive();
    }
}

fn increment_score_system(
    mut ball_hit_paddle_event: EventReader<BallHitPaddleEvent>,
    mut left_score_query: Query<&mut Text, (With<LeftScore>, Without<RightScore>)>,
    mut game_data: ResMut<GameData>,
) {
    if game_data.game_over.is_some() {
        return;
    }

    for _ in ball_hit_paddle_event.iter() {
        game_data.left_score += 1;
        left_score_query.single_mut().sections[0].value = format!("{}", game_data.left_score);
    }
}

fn check_game_over_system(
    mut ball_out_event: EventReader<BallOutEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
    game_data: Res<GameData>,
) {
    if game_data.game_over.is_some() {
        return;
    }

    for _ in ball_out_event.iter() {
        game_over_event.send(GameOverEvent(Side::Right));
    }
}

fn game_over_system(
    mut commands: Commands,
    mut game_over_event: EventReader<GameOverEvent>,
    windows: Res<Windows>,
    config: Res<Config>,
) {
    let window = windows.get_primary().unwrap();

    for _ in game_over_event.iter() {
        commands
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(544.), Val::Px(128.)),
                    position: UiRect {
                        right: Val::Px((window.width() - 544.) / 2.),
                        top: Val::Px(128.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: config.color_transparent.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "GAME OVER",
                        TextStyle {
                            font: config.font.clone(),
                            font_size: 90.,
                            color: config.color_white,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(GameModeEntity {});
    }
}
