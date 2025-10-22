use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub mana: i32,
    pub experience: i32,
    pub level: i32,
    pub faction_id: Option<usize>,
    pub social_class: SocialClass,
    pub skills: Vec<Skill>,
}

#[derive(Component)]
pub struct NPC {
    pub name: String,
    pub dialogue: Vec<String>,
    pub quest_id: Option<usize>,
    pub faction_id: Option<usize>,
    pub social_class: SocialClass,
    pub trade_inventory: Vec<Item>,
    pub job_offer: Option<Job>,
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub damage: i32,
    pub enemy_type: EnemyType,
    pub faction_id: Option<usize>,
}

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub visibility: f32,
    pub region_id: Option<usize>,
}

#[derive(Component)]
pub struct Item {
    pub name: String,
    pub value: i32,
    pub item_type: ItemType,
}

#[derive(Component)]
pub struct LightSource {
    pub intensity: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct Shop {
    pub inventory: Vec<Item>,
    pub restock_timer: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TileType {
    Grass,
    Forest,
    Mountain,
    Water,
    Castle,
    Dungeon,
    Village,
    Town,
    Clearing,
    Farm,
    Ruins,
    Desert,
    Swamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnemyType {
    Bandit,
    Goblin,
    Dragon,
    Troll,
    Undead,
    Wolf,
    RogueKnight,
    DarkMage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SocialClass {
    Knight,
    Noble,
    Priest,
    Townsfolk,
    Bandit,
    Peasant,
    Mage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    QuestItem,
    Material,
    Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: i32,
    pub mana_cost: i32,
    pub cooldown: f32,
    pub effect: SkillEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillEffect {
    Damage(i32),
    Heal(i32),
    Buff(Stat, i32, f32),
    Debuff(Stat, i32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stat {
    Health,
    Mana,
    Speed,
    Defense,
    Magic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub description: String,
    pub reward: Item,
    pub duration: f32,
}
