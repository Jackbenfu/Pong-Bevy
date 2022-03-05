use bevy::prelude::*;
use rand::*;

use crate::config::*;
use crate::systems_generic::*;
use crate::systems_1v1::*;
use crate::components::*;
use crate::state::*;

pub struct Mode1PPlugin;

impl Plugin for Mode1PPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::Mode1P;

        app
            .init_resource::<GameData>()
            .add_event::<GameOverEvent>()
            .add_system_set(
                SystemSet::on_enter(GAME_STATE)
                    .with_system(reset_game_data_system.label("reset_game_data"))
                    .with_system(setup_court_system)
                    .with_system(setup_scores_system)
                    .with_system(setup_instructions_system)
                    .with_system(setup_left_paddle_system.label("setup_paddle").after("reset_game_data"))
                    .with_system(setup_right_paddle_system.label("setup_paddle").after("reset_game_data"))
                    .with_system(setup_ball_system.after("setup_paddle"))
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(service_system)
                    .with_system(launch_ball_system)
                    .with_system(move_left_paddle_with_keyboard_system)
                    .with_system(move_ball_system.label("move_ball"))
                    .with_system(move_right_paddle_with_ai_system.after("move_ball"))
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
                    .with_system(cleanup_entities::<GameModeEntity>)
            );
    }
}

fn setup_instructions_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    // Goal label
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
            color: config.color_transparent.into(),
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
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Goal text
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
            color: config.color_transparent.into(),
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
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Left paddle control label
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
            color: config.color_transparent.into(),
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
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Left paddle control text
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
            color: config.color_transparent.into(),
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
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Launch label
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
            color: config.color_transparent.into(),
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
        .insert(GameModeEntity {})
        .insert(Instruction {});

    // Launch text
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
            color: config.color_transparent.into(),
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
        .insert(GameModeEntity {})
        .insert(Instruction {});
}

fn move_right_paddle_with_ai_system(
    mut paddle_query: Query<(&mut RightPaddle, &mut Transform), Without<Ball>>,
    ball_query: Query<(&Ball, &Transform)>,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let (ball, ball_transform) = ball_query.single();
    let (mut paddle_entity, mut paddle_transform) = paddle_query.single_mut();

    if ball.velocity.x <= 0. {
        paddle_entity.velocity.y = 0.;
        return;
    }

    if ball_transform.translation.x > window.width / 2. {
        paddle_entity.velocity.y = 0.;
        return;
    }

    let ball_y = ball_transform.translation.y;
    let paddle_translation_y = paddle_transform.translation.y;
    let dist: f32 = (paddle_translation_y - ball_y).abs();
    let rnd = random::<u32>() % (paddle_transform.scale.y / 2.) as u32 + 1;

    if dist < rnd as f32 {
        paddle_entity.velocity.y = 0.;
        return;
    }

    let dir = if ball_y > paddle_translation_y { config.game_paddle_speed } else { -config.game_paddle_speed };
    let bound_y = window.height / 2. - config.sprite_unit_size - paddle_transform.scale.y / 2.;

    paddle_transform.translation.y += dir * time.delta_seconds();
    paddle_transform.translation.y = paddle_transform.translation.y.min(bound_y).max(-bound_y);
}
