use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::map::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pickup_items, use_items).run_if
