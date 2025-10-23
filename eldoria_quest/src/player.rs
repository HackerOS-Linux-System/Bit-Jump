use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum SocialClass {
    Knight,
    Noble,
    Peasant,
    Merchant,
    Wizard, // Expanded: new class
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub position: Vec2,
    pub social_class: SocialClass,
    pub health: i32,    // Expanded
    pub strength: i32,  // Expanded
}

impl Player {
    pub fn new(position: Vec2, social_class: SocialClass, health: i32, strength: i32) -> Player {
        Player { position, social_class, health, strength }
    }
}
