// src/menu.rs
use bevy::prelude::*;
use crate::{GameConfig, AppState, WINDOW_WIDTH, WINDOW_HEIGHT};

pub fn menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_interaction.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu);
}

#[derive(Component)]
struct MenuButton {
    action: MenuAction,
}

enum MenuAction {
    Start,
    Settings,
    Exit,
}

#[derive(Component)]
struct SettingsUI;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<GameConfig>) {
    // Root UI node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
        ..default()
    }).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Bark Squadron",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Buttons
        spawn_button(parent, "Start Game", MenuAction::Start);
        spawn_button(parent, "Settings", MenuAction::Settings);
        spawn_button(parent, "Exit", MenuAction::Exit);
    });
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, action: MenuAction) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
            ..default()
        },
        MenuButton { action },
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn menu_interaction(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut config: ResMut<GameConfig>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button.action {
                MenuAction::Start => next_state.set(AppState::Game),
                MenuAction::Settings => setup_settings(&mut commands, &config),
                MenuAction::Exit => std::process::exit(0),
            }
        }
    }
}

fn setup_settings(commands: &mut Commands, config: &GameConfig) {
    // Implement settings UI similarly, with buttons for map, day/night, difficulty
    // For brevity, simulate changing
    // In real, spawn UI elements for settings
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
