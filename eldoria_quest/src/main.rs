use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

mod map;
mod player;
mod npc;
mod game;

use game::{GameState, GameResources, setup_camera, update_player_movement, render_map, render_entities, handle_interactions, setup_main_menu, handle_menu_buttons, setup_character_creation, handle_class_selection, setup_pause_menu, handle_pause_buttons, setup_settings, handle_settings};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Eldoria Quest".into(),
                             resolution: (800.0, 600.0).into(),
                             ..default()
        }),
        ..default()
    }))
    .init_resource::<GameResources>()
    .init_state::<GameState>()
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (
        update_player_movement.run_if(in_state(GameState::InGame)),
                          handle_interactions.run_if(in_state(GameState::InGame)),
                          render_map.run_if(in_state(GameState::InGame)),
                          render_entities.run_if(in_state(GameState::InGame)),
                          handle_menu_buttons.run_if(in_state(GameState::MainMenu)),
                          handle_class_selection.run_if(in_state(GameState::CharacterCreation)),
                          handle_pause_buttons.run_if(in_state(GameState::Pause)),
                          handle_settings.run_if(in_state(GameState::Settings)),
    ))
    .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
    .add_systems(OnEnter(GameState::CharacterCreation), setup_character_creation)
    .add_systems(OnEnter(GameState::Pause), setup_pause_menu)
    .add_systems(OnEnter(GameState::Settings), setup_settings)
    .add_systems(Update, handle_state_transitions)
    .run();
}

fn handle_state_transitions(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => next_state.set(GameState::Pause),
            GameState::Pause => next_state.set(GameState::InGame),
            GameState::Settings => next_state.set(GameState::MainMenu),
            _ => {},
        }
    }
}
