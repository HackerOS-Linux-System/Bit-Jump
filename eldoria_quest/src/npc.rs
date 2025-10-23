use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Npc {
    pub position: Vec2,
    pub name: String,
    pub dialog: String,
}

impl Npc {
    pub fn new(position: Vec2, name: String, dialog: String) -> Npc {
        Npc { position, name, dialog }
    }
}
