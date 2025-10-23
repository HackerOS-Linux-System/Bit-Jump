use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::map::*;

pub struct TradingPlugin;

impl Plugin for TradingPlugin {
    fn build(&mut app: &mut App) {
        app.add_systems(Update, trading_system.run_if(in_state(GameState::InGame)))
        .add_event::<StartTrade>();
    }
}

#[derive(Event, Clone)]
pub struct StartTrade {
    pub npc_entity: Entity,
}

fn trading_system(
    mut commands: Commands,
    mut trade_events: EventReader<StartTrade>,
    mut inventory: ResMut<Inventory>,
    npc_query: Query<&NPC>,
) {
    for event in trade_events.read() {
        if let Ok(npc) = npc_query.get(event.npc_entity) {
            // Simplified trading: buy first item in NPC's inventory for 10 gold
            if !npc.trade_inventory.is_empty() && inventory.gold >= 10 {
                inventory.items.push(npc.trade_inventory[0].clone());
                inventory.gold -= 10;
            }
        }
    }
}

