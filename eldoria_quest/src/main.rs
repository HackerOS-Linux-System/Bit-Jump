use bevy::prelude::*;
use bevy::window::WindowMode;
mod components;
mod systems;
mod resources;
mod map;
mod menu;
mod quests;
mod combat;
mod inventory;
mod crafting;
mod factions;
mod environment;
mod skills;
mod trading;
mod events;
mod save_load;

use components::*;
use systems::*;
use resources::*;
use map::*;
use menu::*;
use quests::*;
use combat::*;
use inventory::*;
use crafting::*;
use factions::*;
use environment::*;
use skills::*;
use trading::*;
use events::*;
use save_load::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Eldoria Quest".into(),
                             resolution: (1280.0, 720.0).into(),
                             ..default()
        }),
        ..default()
    }))
    .init_state::<GameState>()
    .init_resource::<GameSettings>()
    .init_resource::<Quests>()
    .init_resource::<Inventory>()
    .init_resource::<Factions>()
    .init_resource::<Environment>()
    .init_resource::<Skills>()
    .init_resource::<RandomEvents>()
    .add_plugins(MenuPlugin)
    .add_plugins(GamePlugin)
    .add_plugins(MapPlugin)
    .add_plugins(QuestPlugin)
    .add_plugins(CombatPlugin)
    .add_plugins(InventoryPlugin)
    .add_plugins(CraftingPlugin)
    .add_plugins(FactionPlugin)
    .add_plugins(EnvironmentPlugin)
    .add_plugins(SkillsPlugin)
    .add_plugins(TradingPlugin)
    .add_plugins(EventsPlugin)
    .add_systems(Startup, setup_camera)
    .add_systems(Update, toggle_fullscreen)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
                   ..default()
    });
}

fn toggle_fullscreen(keyboard: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = if window.mode == WindowMode::Fullscreen {
                WindowMode::Windowed
            } else {
                WindowMode::Fullscreen
            };
        }
    }
}
