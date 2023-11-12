use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::*;

use crate::config::*;
use crate::systems_generic::*;
use crate::systems_1v1::*;
use crate::components::*;
use crate::events::*;
use crate::state::*;

pub struct Mode1PPlugin;

#[derive(Debug, Eq, PartialEq, Clone, Hash, SystemSet)]
enum Set {
    ResetGameData,
    SetupPaddle,
    MoveBall,
    Move,
    IncrementScore,
    CheckBallOut,
    CheckGameOver,
    Back,
}

impl Plugin for Mode1PPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::Mode1P;

        app
            .init_resource::<GameData>()
            .add_event::<GameOverEvent>()
            .add_event::<BallOutEvent>()
            .add_event::<BallHitPaddleEvent>()
            .add_systems(OnEnter(GAME_STATE), (
                reset_game_data_system.in_set(Set::ResetGameData),
                setup_court_system,
                setup_scores_system,
                setup_instructions_system,
                setup_left_paddle_system.in_set(Set::SetupPaddle).after(Set::ResetGameData),
                setup_right_paddle_system.in_set(Set::SetupPaddle).after(Set::ResetGameData),
                setup_ball_system.after(Set::SetupPaddle)
            ))
            .add_systems(Update, (
                service_system,
                launch_ball_system,
                move_left_paddle_with_keyboard_system,
                move_ball_system.in_set(Set::MoveBall),
                move_right_paddle_with_ai_system.after(Set::MoveBall)
            ).run_if(in_state(GAME_STATE)).in_set(Set::Move).before(Set::Back))
            .add_systems(
                Update, (
                    check_ball_collision_system,
                    check_ball_out_system.in_set(Set::CheckBallOut),
                    increment_score_system.in_set(Set::IncrementScore).after(Set::CheckBallOut),
                    check_game_over_system.in_set(Set::CheckGameOver).after(Set::IncrementScore),
                    game_over_system.after(Set::CheckGameOver),
                ).run_if(in_state(GAME_STATE)).after(Set::Move).before(Set::Back))
            .add_systems(Update, back_to_menu_system.in_set(Set::Back).run_if(in_state(GAME_STATE)))
            .add_systems(OnExit(GAME_STATE), cleanup_entities::<GameModeEntity>);
    }
}

fn setup_instructions_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    // Goal label
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(352.),
                height: Val::Px(48.),
                bottom: Val::Px(192.),
                left: Val::Px(0.),
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
                width: Val::Px(352.),
                height: Val::Px(48.),
                bottom: Val::Px(192.),
                left: Val::Px(416.),
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
                    "beat the AI!",
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
                width: Val::Px(352.),
                height: Val::Px(48.),
                bottom: Val::Px(144.),
                left: Val::Px(0.),
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
                width: Val::Px(352.),
                height: Val::Px(48.),
                bottom: Val::Px(144.),
                left: Val::Px(416.),
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
                    "move left paddle",
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
                width: Val::Px(352.),
                height: Val::Px(48.),
                bottom: Val::Px(96.),
                left: Val::Px(0.),
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
                width: Val::Px(352.),
                height: Val::Px(48.),
                bottom: Val::Px(96.),
                left: Val::Px(416.),
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

fn move_right_paddle_with_ai_system(
    mut paddle_query: Query<(&mut RightPaddle, &mut Transform), Without<Ball>>,
    ball_query: Query<(&Ball, &Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let (ball, ball_transform) = ball_query.single();
    let (mut paddle_entity, mut paddle_transform) = paddle_query.single_mut();

    if ball.velocity.x <= 0. {
        paddle_entity.velocity.y = 0.;
        return;
    }

    let window = window.get_single().unwrap();

    if ball_transform.translation.x > window.width() / 2. {
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
    let bound_y = window.height() / 2. - config.sprite_unit_size - paddle_transform.scale.y / 2.;

    paddle_transform.translation.y += dir * time.delta_seconds();
    paddle_transform.translation.y = paddle_transform.translation.y.min(bound_y).max(-bound_y);
}
