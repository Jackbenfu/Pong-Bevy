use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::ElementState;
use crate::state::*;

pub fn cleanup_system<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn back_to_menu_system(
    mut state: ResMut<State<GameState>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed && key_code == KeyCode::Escape {
                state.set(GameState::Menu).unwrap();
            }
        }
    }
}
