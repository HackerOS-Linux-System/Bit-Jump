use bevy::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use ron;
use serde::{Serialize, Deserialize};

use crate::map::{Map, TileType};
use crate::player::{Player, SocialClass};
use crate::npc::Npc;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    CharacterCreation,
    InGame,
    Settings,
    Pause,
}

#[derive(Resource)]
pub struct GameResources {
    pub map: Map,
    pub player: Player,
    pub npcs: Vec<Npc>,
    pub classes: Vec<SocialClass>,
    pub selected_class: usize,
    pub tile_size: f32,
    pub camera_offset: Vec2,
    pub message: String,
    pub logo_handle: Handle<Image>,
    pub font_handle: Handle<Font>,
    pub quests: Vec<Quest>, // Expanded: added quests
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub player: Player,
    pub map: Map,
    pub npcs: Vec<Npc>,
    pub quests: Vec<Quest>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub giver_npc: String, // Name of NPC who gave the quest
}

impl Default for GameResources {
    fn default() -> Self {
        let map = Map::new(100, 100); // Expanded map size
        let npcs = vec![
            Npc::new(Vec2::new(10.0, 10.0), "Villager".to_string(), "Hello, traveler! Quest: Find the lost sword in the forest.".to_string()),
            Npc::new(Vec2::new(20.0, 20.0), "Knight".to_string(), "Beware the forest! But if you bring me a relic, I'll reward you.".to_string()),
            Npc::new(Vec2::new(30.0, 15.0), "Merchant".to_string(), "Buy some potions? Or fetch me herbs for a quest.".to_string()),
            // More NPCs
        ];
        let classes = vec![SocialClass::Knight, SocialClass::Noble, SocialClass::Peasant, SocialClass::Merchant, SocialClass::Wizard]; // Expanded classes
        let quests = vec![
            Quest { name: "Lost Sword".to_string(), description: "Find the sword in the forest.".to_string(), completed: false, giver_npc: "Villager".to_string() },
            // More quests
        ];

        GameResources {
            map,
            player: Player::new(Vec2::new(5.0, 5.0), SocialClass::Peasant, 100, 10), // Expanded player with health and strength
            npcs,
            classes,
            selected_class: 0,
            tile_size: 32.0,
            camera_offset: Vec2::ZERO,
            message: String::new(),
            logo_handle: Handle::default(),
            font_handle: Handle::default(),
            quests,
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update_player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameResources>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let speed = 100.0 * time.delta_seconds(); // Expanded: smoother movement
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    game.player.position += direction * speed;

    // Clamp to map
    game.player.position.x = game.player.position.x.clamp(0.0, game.map.width as f32 - 1.0);
    game.player.position.y = game.player.position.y.clamp(0.0, game.map.height as f32 - 1.0);

    // Camera follow
    let mut camera_transform = query.single_mut();
    camera_transform.translation.x = game.player.position.x * game.tile_size - 400.0; // Center
    camera_transform.translation.y = game.player.position.y * game.tile_size - 300.0;
}

pub fn render_map(
    game: Res<GameResources>,
    mut commands: Commands,
    query: Query<Entity, With<TileMarker>>, // Despawn old tiles
) {
    // Despawn old map tiles
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for y in 0..game.map.height {
        for x in 0..game.map.width {
            let tile = game.map.tiles[y][x];
            let color = match tile {
                TileType::Meadow => Color::srgb(0.0, 1.0, 0.0),
                TileType::Forest => Color::srgb(0.0, 0.5, 0.0),
                TileType::Castle => Color::srgb(0.5, 0.5, 0.5),
                TileType::Village => Color::srgb(0.8, 0.6, 0.4),
                TileType::Mountain => Color::srgb(0.6, 0.6, 0.6), // Expanded terrain
                TileType::River => Color::srgb(0.0, 0.0, 1.0),
            };
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x as f32 * game.tile_size, y as f32 * game.tile_size, 0.0)),
                           sprite: Sprite {
                               color,
                               custom_size: Some(Vec2::new(game.tile_size, game.tile_size)),
                           ..default()
                           },
                           ..default()
            }).insert(TileMarker);
        }
    }
}

#[derive(Component)]
struct TileMarker;

pub fn render_entities(
    game: Res<GameResources>,
    mut commands: Commands,
    query: Query<Entity, With<EntityMarker>>,
) {
    // Despawn old entities
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Render player
    let player_color = match game.player.social_class {
        SocialClass::Knight => Color::srgb(1.0, 0.0, 0.0),
        SocialClass::Noble => Color::srgb(1.0, 1.0, 0.0),
        SocialClass::Peasant => Color::srgb(0.5, 0.3, 0.1),
        SocialClass::Merchant => Color::srgb(0.0, 1.0, 1.0),
        SocialClass::Wizard => Color::srgb(0.5, 0.0, 1.0), // New class color
    };
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(game.player.position.x * game.tile_size, game.player.position.y * game.tile_size, 1.0)),
                   sprite: Sprite {
                       color: player_color,
                       custom_size: Some(Vec2::new(game.tile_size, game.tile_size)),
                   ..default()
                   },
                   ..default()
    }).insert(EntityMarker);

    // Render NPCs
    for npc in &game.npcs {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(npc.position.x * game.tile_size, npc.position.y * game.tile_size, 1.0)),
                       sprite: Sprite {
                           color: Color::srgb(0.0, 0.0, 1.0),
                       custom_size: Some(Vec2::new(game.tile_size, game.tile_size)),
                       ..default()
                       },
                       ..default()
        }).insert(EntityMarker);
    }

    // Render message if any
    if !game.message.is_empty() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(&game.message, TextStyle {
                font: game.font_handle.clone(),
                                     font_size: 20.0,
                                     color: Color::WHITE,
            }),
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 2.0)),
                       ..default()
        }).insert(EntityMarker);
    }
}

#[derive(Component)]
struct EntityMarker;

pub fn handle_interactions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameResources>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let player_pos = game.player.position;
        let mut interacting_npc: Option<(String, String)> = None;
        for npc in &game.npcs {
            if (npc.position - player_pos).length() < 1.5 {
                interacting_npc = Some((npc.name.clone(), npc.dialog.clone()));
                break;
            }
        }
        if let Some((name, dialog)) = interacting_npc {
            game.message = format!("{} says: {}", name, dialog);
            let mut quest_index: Option<usize> = None;
            for (i, quest) in game.quests.iter().enumerate() {
                if quest.giver_npc == name && !quest.completed {
                    quest_index = Some(i);
                    break;
                }
            }
            if let Some(index) = quest_index {
                game.message.push_str(&format!("\nQuest: {}", game.quests[index].description));
                if game.map.tiles[player_pos.y as usize][player_pos.x as usize] == TileType::Forest {
                    game.quests[index].completed = true;
                    game.message.push_str("\nQuest completed!");
                }
            }
        } else {
            game.message = String::new();
        }
    }
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameResources>,
) {
    game.logo_handle = asset_server.load("images/eldoria_quest.png");
    game.font_handle = asset_server.load("fonts/FiraSans-Bold.ttf"); // Assume a font in assets/fonts

    // UI camera if needed, but DefaultPlugins include it

    // Root node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
                   height: Val::Percent(100.0),
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   flex_direction: FlexDirection::Column,
                   ..default()
        },
        ..default()
    }).with_children(|parent| {
        // Logo
        parent.spawn(ImageBundle {
            image: UiImage::new(game.logo_handle.clone()),
                     style: Style {
                         width: Val::Px(200.0),
                     height: Val::Px(100.0),
                     margin: UiRect::all(Val::Px(20.0)),
                     ..default()
                     },
                     ..default()
        });

        // Buttons
        spawn_button(parent, "New Game", ButtonType::NewGame, &game.font_handle);
        spawn_button(parent, "Load", ButtonType::Load, &game.font_handle);
        spawn_button(parent, "Settings", ButtonType::Settings, &game.font_handle);
        spawn_button(parent, "Exit", ButtonType::Exit, &game.font_handle);
    }).insert(MenuMarker);
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, button_type: ButtonType, font: &Handle<Font>) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
                 height: Val::Px(50.0),
                 margin: UiRect::all(Val::Px(10.0)),
                 justify_content: JustifyContent::Center,
                 align_items: AlignItems::Center,
                 ..default()
        },
        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                 ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: font.clone(),
                                              font_size: 20.0,
                                              color: Color::WHITE,
            },
        ));
    }).insert(button_type);
}

#[derive(Component)]
enum ButtonType {
    NewGame,
    Load,
    Settings,
    Exit,
    Start,
    Resume,
    Save,
    MainMenu,
}

#[derive(Component)]
struct MenuMarker;

pub fn handle_menu_buttons(
    mut interaction_query: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
                           mut next_state: ResMut<NextState<GameState>>,
                           mut game: ResMut<GameResources>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                ButtonType::NewGame => next_state.set(GameState::CharacterCreation),
                ButtonType::Load => load_game(&mut *game, &mut *next_state),
                ButtonType::Settings => next_state.set(GameState::Settings),
                ButtonType::Exit => std::process::exit(0),
                _ => {},
            }
        }
    }
    // Despawn menu when transitioning, but OnExit can be used
}

pub fn setup_character_creation(
    mut commands: Commands,
    game: Res<GameResources>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
                   height: Val::Percent(100.0),
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   flex_direction: FlexDirection::Column,
                   ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Choose your class",
            TextStyle {
                font: game.font_handle.clone(),
                                              font_size: 30.0,
                                              color: Color::WHITE,
            },
        ));

        for class in &game.classes {
            let class_name = format!("{:?}", class);
            spawn_button(parent, &class_name, ButtonType::Start, &game.font_handle); // Simplified, use selection
        }

        spawn_button(parent, "Start", ButtonType::Start, &game.font_handle);
    }).insert(CharCreationMarker);
}

#[derive(Component)]
struct CharCreationMarker;

pub fn handle_class_selection(
    mut interaction_query: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
                              mut game: ResMut<GameResources>,
                              mut next_state: ResMut<NextState<GameState>>,
                              keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Keyboard selection
    if keyboard_input.just_pressed(KeyCode::ArrowUp) && game.selected_class > 0 {
        game.selected_class -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) && game.selected_class < game.classes.len() - 1 {
        game.selected_class += 1;
    }
    if keyboard_input.just_pressed(KeyCode::Enter) {
        game.player.social_class = game.classes[game.selected_class];
        // Expanded: set stats based on class
        match game.player.social_class {
            SocialClass::Knight => { game.player.strength = 20; game.player.health = 150; }
            SocialClass::Wizard => { game.player.strength = 5; game.player.health = 80; } // New
            _ => {},
        }
        next_state.set(GameState::InGame);
    }

    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let ButtonType::Start = button_type {
                game.player.social_class = game.classes[game.selected_class];
                next_state.set(GameState::InGame);
            }
        }
    }
}

pub fn setup_pause_menu(
    mut commands: Commands,
    game: Res<GameResources>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
                   height: Val::Percent(100.0),
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   flex_direction: FlexDirection::Column,
                   ..default()
        },
        ..default()
    }).with_children(|parent| {
        spawn_button(parent, "Resume", ButtonType::Resume, &game.font_handle);
        spawn_button(parent, "Save", ButtonType::Save, &game.font_handle);
        spawn_button(parent, "Main Menu", ButtonType::MainMenu, &game.font_handle);
    }).insert(PauseMarker);
}

#[derive(Component)]
struct PauseMarker;

pub fn handle_pause_buttons(
    mut interaction_query: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
                            mut next_state: ResMut<NextState<GameState>>,
                            game: Res<GameResources>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                ButtonType::Resume => next_state.set(GameState::InGame),
                ButtonType::Save => save_game(&game),
                ButtonType::MainMenu => next_state.set(GameState::MainMenu),
                _ => {},
            }
        }
    }
}

pub fn setup_settings(
    mut commands: Commands,
    game: Res<GameResources>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
                   height: Val::Percent(100.0),
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Settings - Press Esc to return\n(No sound settings)",
                                              TextStyle {
                                                  font: game.font_handle.clone(),
                                              font_size: 20.0,
                                              color: Color::WHITE,
                                              },
        ));
    }).insert(SettingsMarker);
}

#[derive(Component)]
struct SettingsMarker;

pub fn handle_settings() {
    // Placeholder, add more if needed
}

pub fn save_game(game: &Res<GameResources>) {
    let save_data = SaveData {
        player: game.player.clone(),
        map: game.map.clone(),
        npcs: game.npcs.clone(),
        quests: game.quests.clone(),
    };
    if let Ok(mut file) = File::create("save.ron") {
        if let Ok(ron_string) = ron::to_string(&save_data) {
            let _ = file.write_all(ron_string.as_bytes());
        }
    }
}

pub fn load_game(game: &mut GameResources, next_state: &mut NextState<GameState>) {
    if Path::new("save.ron").exists() {
        if let Ok(mut file) = File::open("save.ron") {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(save_data) = ron::from_str::<SaveData>(&contents) {
                    game.player = save_data.player;
                    game.map = save_data.map;
                    game.npcs = save_data.npcs;
                    game.quests = save_data.quests;
                    // Transition to InGame
                    next_state.set(GameState::InGame);
                }
            }
        }
    }
}
