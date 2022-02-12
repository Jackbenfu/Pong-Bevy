use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::app::*;
use crate::resources::*;
use crate::utils::*;
use crate::state::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_background_system)
                    .with_system(setup_title_system)
                    .with_system(setup_copyright_system)
                    .with_system(setup_buttons_system)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(hover_buttons_system)
                    .with_system(click_1_player_button_system)
                    .with_system(click_2_players_button_system)
                    .with_system(click_wall_mode_button_system)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(cleanup_system::<MenuEntity>)
            )
            .add_state(GameState::Menu);

        #[cfg(not(target_arch = "wasm32"))]
            app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(click_quit_button_system)
        );
    }
}

const SPRITE_UNITY_SIZE: f32 = 16.;
const GREY: Color = Color::rgb(100. / 255., 100. / 255., 100. / 255.);

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

fn setup_background_system(
    window: Res<WindowDescriptor>,
    mut commands: Commands,
) {
    // top wall
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., window.height / 2. - SPRITE_UNITY_SIZE / 2., 0.),
                scale: Vec3::new(window.width, SPRITE_UNITY_SIZE, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: GREY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MenuEntity {});

    // bottom wall
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -window.height / 2. + SPRITE_UNITY_SIZE / 2., 0.),
                scale: Vec3::new(window.width, SPRITE_UNITY_SIZE, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: GREY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MenuEntity {});

    // left paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-window.width / 2. + SPRITE_UNITY_SIZE / 2. + SPRITE_UNITY_SIZE, 0., 0.),
                scale: Vec3::new(SPRITE_UNITY_SIZE, SPRITE_UNITY_SIZE * 4., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: GREY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MenuEntity {});

    // right paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(window.width / 2. - SPRITE_UNITY_SIZE / 2. - SPRITE_UNITY_SIZE, 0., 0.),
                scale: Vec3::new(SPRITE_UNITY_SIZE, SPRITE_UNITY_SIZE * 4., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: GREY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MenuEntity {});

    // net
    let mut y: f32 = 0.;
    while y < window.height / 2. {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., y, 0.),
                    scale: Vec3::new(SPRITE_UNITY_SIZE, SPRITE_UNITY_SIZE * 2., 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: GREY,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(MenuEntity {});

        if y != 0. {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0., -y, 0.),
                        scale: Vec3::new(SPRITE_UNITY_SIZE, SPRITE_UNITY_SIZE * 2., 0.),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: GREY,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MenuEntity {});
        }

        y += SPRITE_UNITY_SIZE * 3.;
    }
}

fn setup_title_system(
    resources: Res<Resources>,
    window: Res<WindowDescriptor>,
    mut commands: Commands,
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "PONG",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 144.,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {});
}

fn setup_copyright_system(
    resources: Res<Resources>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(SPRITE_UNITY_SIZE + 15.),
                    right: Val::Px(15.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Copyright © 2022 Jackbenfu",
                TextStyle {
                    font: resources.font.clone(),
                    font_size: 18.,
                    color: GREY,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(MenuEntity {});
}

fn setup_buttons_system(
    resources: Res<Resources>,
    window: Res<WindowDescriptor>,
    mut commands: Commands,
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "1 player",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 36.,
                        color: Color::WHITE,
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "2 players",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 36.,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButton2Players {});

    // wall mode button
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Wall mode",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 36.,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(MenuEntity {})
        .insert(MenuButton {})
        .insert(MenuButtonWallMode {});

    // quit button
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
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Quit",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: 36.,
                        color: Color::WHITE,
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
) {
    const YELLOW: Color = Color::rgb(221. / 255., 173. / 255., 29. / 255.);

    let window = windows.get_primary_mut().unwrap();
    let mut hovered: bool = false;

    for (interaction, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Hovered => {
                text.sections[0].style.color = YELLOW;
                hovered = true;
            }
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE;
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
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Mode1P).unwrap();
            }
            _ => {}
        }
    }
}

fn click_2_players_button_system(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButton2Players>>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Mode2P).unwrap();
            }
            _ => {}
        }
    }
}

fn click_wall_mode_button_system(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, With<MenuButtonWallMode>>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::ModeWall).unwrap();
            }
            _ => {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn click_quit_button_system(
    mut app_exit_events: EventWriter<AppExit>,
    interaction_query: Query<&Interaction, With<MenuButtonQuit>>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                app_exit_events.send(AppExit);
            }
            _ => {}
        }
    }
}
