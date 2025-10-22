use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::components::*;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct GameSettings {
    pub graphics_quality: GraphicsQuality,
    pub fullscreen: bool,
    pub difficulty: Difficulty,
    pub key_bindings: KeyBindings,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GraphicsQuality {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyBindings {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub interact: KeyCode,
    pub attack: KeyCode,
    pub pick_up: KeyCode,
    pub use_item: KeyCode,
    pub craft: KeyCode,
    pub trade: KeyCode,
    pub skill_1: KeyCode,
    pub skill_2: KeyCode,
    pub skill_3: KeyCode,
    pub work: KeyCode,
}

#[derive(Resource)]
pub struct WorldMap {
    pub tiles: Vec<Vec<TileType>>,
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: usize,
    pub name: String,
    pub tile_type: TileType,
    pub center: (usize, usize),
    pub radius: usize,
}

#[derive(Resource, Default)]
pub struct Quests {
    pub active: Vec<Quest>,
    pub completed: Vec<Quest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: usize,
    pub description: String,
    pub reward: Item,
    pub completed: bool,
    pub objectives: Vec<Objective>,
    pub faction_id: Option<usize>,
    pub required_class: Option<SocialClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub description: String,
    pub target: String,
    pub count: i32,
    pub current: i32,
    pub target_type: ObjectiveType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    Collect,
    Kill,
    Deliver,
    Explore,
    Escort,
    Defend,
}

#[derive(Resource, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
    pub gold: i32,
}

#[derive(Resource, Default)]
pub struct Factions {
    pub factions: Vec<Faction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: usize,
    pub name: String,
    pub reputation: i32,
    pub allied_classes: Vec<SocialClass>,
}

#[derive(Resource, Default)]
pub struct Environment {
    pub time_of_day: f32,
    pub weather: Weather,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Weather {
    #[default]
    Clear,
    Rain,
    Fog,
    Storm,
    Snow,
}

#[derive(Resource, Default)]
pub struct Skills {
    pub available_skills: Vec<Skill>,
}

#[derive(Resource, Default)]
pub struct RandomEvents {
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub description: String,
    pub effect: EventEffect,
    pub duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventEffect {
    SpawnEnemies(EnemyType, i32),
    DropItems(Item, i32),
    WeatherChange(Weather),
    FactionBoost(usize, i32),
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct SaveData {
    pub player_position: Vec2,
    pub player_health: i32,
    pub player_mana: i32,
    pub player_experience: i32,
    pub player_level: i32,
    pub player_faction_id: Option<usize>,
    pub player_social_class: SocialClass,
    pub player_skills: Vec<Skill>,
    pub inventory: Vec<Item>,
    pub gold: i32,
    pub quests: Quests,
    pub factions: Factions,
    pub map: Vec<Vec<TileType>>,
    pub regions: Vec<Region>,
    pub environment: Environment,
}
