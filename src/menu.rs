use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::*;
use bevy::window::PrimaryWindow;

use crate::config::*;
use crate::systems_generic::*;
use crate::state::*;
use crate::helpers_sprite::*;

#[derive(Component)]
struct MenuEntity {}

#[derive(Component)]
struct MenuButton {}

#[derive(Component)]
struct MenuButton1Player {}

#[derive(Component)]
struct MenuButton2Players {}

#[derive(Component)]
struct MenuButtonWallMode {}

#[derive(Component)]
struct MenuButtonQuit {}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        const GAME_STATE: GameState = GameState::Menu;

        app
            .add_systems(OnEnter(GAME_STATE), (
                setup_background_system,
                setup_title_system,
                setup_copyright_system,
                setup_buttons_system
            ))
            .add_systems(Update, (
                hover_buttons_system,
                click_1_player_button_system,
                click_2_players_button_system,
                click_wall_mode_button_system
            ).run_if(in_state(GAME_STATE)))
            .add_systems(OnExit(GAME_STATE), cleanup_entities::<MenuEntity>);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Update, click_quit_button_system.run_if(in_state(GAME_STATE)));
    }
}

fn setup_background_system(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    config: Res<Config>,
) {
    let window = window.get_single().unwrap();
    let color = config.color_grey;
    let unit_size = config.sprite_unit_size;

    commands
        .spawn(create_top_wall_sprite(window.width(), window.height(), unit_size, color))
        .insert(MenuEntity {});

    commands
        .spawn(create_bottom_wall_sprite(window.width(), window.height(), unit_size, color))
        .insert(MenuEntity {});

    commands
        .spawn(create_left_paddle_sprite(window.width(), unit_size, color))
        .insert(MenuEntity {});

    commands
        .spawn(create_right_paddle_sprite(window.width(), unit_size, color))
        .insert(MenuEntity {});

    // Net
    {
        commands
            .spawn(create_net_sprite(0., unit_size, color))
            .insert(MenuEntity {});

        let mut y: f32 = unit_size * 3.;
        while y < window.height() / 2. {
            commands
                .spawn(create_net_sprite(y, unit_size, color))
                .insert(MenuEntity {});

            commands
                .spawn(create_net_sprite(-y, unit_size, color))
                .insert(MenuEntity {});

            y += unit_size * 3.;
        }
    }
}

fn setup_title_system(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    config: Res<Config>,
) {
    let window = window.get_single().unwrap();

    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(416.),
                height: Val::Px(128.),
                right: Val::Px((window.width() - 416.) / 2.),
                top: Val::Px(128.),
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
                    "PONG",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 144.,
                        color: config.color_white,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {});
}

fn setup_copyright_system(
    mut commands: Commands,
    config: Res<Config>,
) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                bottom: Val::Px(config.sprite_unit_size * 2.),
                right: Val::Px(config.sprite_unit_size),
                ..Default::default()
            },
            text: Text::from_section(
                "© 2022 Jackbenfu",
                TextStyle {
                    font: config.font.clone(),
                    font_size: 18.,
                    color: config.color_grey,
                },
            ),
            ..Default::default()
        })
        .insert(MenuEntity {});
}

fn setup_buttons_system(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    config: Res<Config>,
) {
    let window = window.get_single().unwrap();

    // 1 player button
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(256.),
                height: Val::Px(48.),
                right: Val::Px((window.width() - 256.) / 2.),
                top: Val::Px(272.),
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
                    "1 player",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButton1Player {});

    // 2 players button
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(256.),
                height: Val::Px(48.),
                right: Val::Px((window.width() - 256.) / 2.),
                top: Val::Px(336.),
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
                    "2 players",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButton2Players {});

    // Wall mode button
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(256.),
                height: Val::Px(48.),
                right: Val::Px((window.width() - 256.) / 2.),
                top: Val::Px(400.),
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
                    "Wall mode",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButtonWallMode {});

    // Quit button
    #[cfg(not(target_arch = "wasm32"))]
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(256.),
                height: Val::Px(48.),
                right: Val::Px((window.width() - 256.) / 2.),
                top: Val::Px(464.),
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
                    "Quit",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButtonQuit {});
}

fn hover_buttons_system(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut interaction_query: Query<(&Interaction, &Children), With<MenuButton>>,
    mut text_query: Query<&mut Text>,
    config: Res<Config>,
) {
    let mut window = window.get_single_mut().unwrap();
    let mut hovered: bool = false;

    for (interaction, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Hovered => {
                text.sections[0].style.color = config.color_yellow;
                hovered = true;
            }
            Interaction::None => {
                text.sections[0].style.color = config.color_white;
            }
            _ => {}
        }
    }

    window.cursor.icon = if hovered { CursorIcon::Hand } else { CursorIcon::Default };
}

fn click_1_player_button_system(
    mut state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButton1Player>>,
) {
    match *interaction_query.single() {
        Interaction::Pressed => {
            state.set(GameState::Mode1P);
        }
        _ => {}
    }
}

fn click_2_players_button_system(
    mut state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButton2Players>>,
) {
    match *interaction_query.single() {
        Interaction::Pressed => {
            state.set(GameState::Mode2P);
        }
        _ => {}
    }
}

fn click_wall_mode_button_system(
    mut state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButtonWallMode>>,
) {
    match *interaction_query.single() {
        Interaction::Pressed => {
            state.set(GameState::ModeWall);
        }
        _ => {}
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn click_quit_button_system(
    mut app_exit_events: EventWriter<AppExit>,
    interaction_query: Query<&Interaction, With<MenuButtonQuit>>,
) {
    match *interaction_query.single() {
        Interaction::Pressed => {
            app_exit_events.send(AppExit);
        }
        _ => {}
    }
}
