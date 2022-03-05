use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::*;

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
            .add_system_set(
                SystemSet::on_enter(GAME_STATE)
                    .with_system(setup_background_system)
                    .with_system(setup_title_system)
                    .with_system(setup_copyright_system)
                    .with_system(setup_buttons_system)
            )
            .add_system_set(
                SystemSet::on_update(GAME_STATE)
                    .with_system(hover_buttons_system)
                    .with_system(click_1_player_button_system)
                    .with_system(click_2_players_button_system)
                    .with_system(click_wall_mode_button_system)
            )
            .add_system_set(
                SystemSet::on_exit(GAME_STATE)
                    .with_system(cleanup_entities::<MenuEntity>)
            );

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system_set(
            SystemSet::on_update(GAME_STATE)
                .with_system(click_quit_button_system)
        );
    }
}

fn setup_background_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    let color = config.color_grey;
    let unit_size = config.sprite_unit_size;

    commands
        .spawn_bundle(create_top_wall_sprite(window.width, window.height, unit_size, color))
        .insert(MenuEntity {});

    commands
        .spawn_bundle(create_bottom_wall_sprite(window.width, window.height, unit_size, color))
        .insert(MenuEntity {});

    commands
        .spawn_bundle(create_left_paddle_sprite(window.width, unit_size, color))
        .insert(MenuEntity {});

    commands
        .spawn_bundle(create_right_paddle_sprite(window.width, unit_size, color))
        .insert(MenuEntity {});

    // Net
    {
        commands
            .spawn_bundle(create_net_sprite(0., unit_size, color))
            .insert(MenuEntity {});

        let mut y: f32 = unit_size * 3.;
        while y < window.height / 2. {
            commands
                .spawn_bundle(create_net_sprite(y, unit_size, color))
                .insert(MenuEntity {});

            commands
                .spawn_bundle(create_net_sprite(-y, unit_size, color))
                .insert(MenuEntity {});

            y += unit_size * 3.;
        }
    }
}

fn setup_title_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(416.), Val::Px(128.)),
                position: Rect {
                    right: Val::Px((window.width - 416.) / 2.),
                    top: Val::Px(128.),
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
                    "PONG",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 144.,
                        color: config.color_white,
                    },
                    Default::default(),
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
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(config.sprite_unit_size * 2.),
                    right: Val::Px(config.sprite_unit_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "© 2022 Jackbenfu",
                TextStyle {
                    font: config.font.clone(),
                    font_size: 18.,
                    color: config.color_grey,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(MenuEntity {});
}

fn setup_buttons_system(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    // 1 player button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(256.), Val::Px(48.)),
                position: Rect {
                    right: Val::Px((window.width - 256.) / 2.),
                    top: Val::Px(272.),
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
                    "1 player",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButton1Player {});

    // 2 players button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(256.), Val::Px(48.)),
                position: Rect {
                    right: Val::Px((window.width - 256.) / 2.),
                    top: Val::Px(336.),
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
                    "2 players",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButton2Players {});

    // Wall mode button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(256.), Val::Px(48.)),
                position: Rect {
                    right: Val::Px((window.width - 256.) / 2.),
                    top: Val::Px(400.),
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
                    "Wall mode",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                    Default::default(),
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
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(256.), Val::Px(48.)),
                position: Rect {
                    right: Val::Px((window.width - 256.) / 2.),
                    top: Val::Px(464.),
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
                    "Quit",
                    TextStyle {
                        font: config.font.clone(),
                        font_size: 36.,
                        color: config.color_white,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButtonQuit {});
}

fn hover_buttons_system(
    mut windows: ResMut<Windows>,
    mut interaction_query: Query<(&Interaction, &Children), With<MenuButton>>,
    mut text_query: Query<&mut Text>,
    config: Res<Config>,
) {
    let window = windows.get_primary_mut().unwrap();
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

    window.set_cursor_icon(if hovered { CursorIcon::Hand } else { CursorIcon::Default });
}

fn click_1_player_button_system(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButton1Player>>,
) {
    match *interaction_query.single() {
        Interaction::Clicked => {
            state.set(GameState::Mode1P).unwrap();
        }
        _ => {}
    }
}

fn click_2_players_button_system(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButton2Players>>,
) {
    match *interaction_query.single() {
        Interaction::Clicked => {
            state.set(GameState::Mode2P).unwrap();
        }
        _ => {}
    }
}

fn click_wall_mode_button_system(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButtonWallMode>>,
) {
    match *interaction_query.single() {
        Interaction::Clicked => {
            state.set(GameState::ModeWall).unwrap();
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
        Interaction::Clicked => {
            app_exit_events.send(AppExit);
        }
        _ => {}
    }
}
