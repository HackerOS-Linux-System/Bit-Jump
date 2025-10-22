use bevy::prelude::*;
use super::components::*;
use super::resources::*;

pub struct FactionPlugin;

impl Plugin for FactionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, faction_interaction.run_if(in_state(GameState::InGame)));
    }
}

fn faction_interaction(
    mut player_query: Query<&mut Player>,
    npc_query: Query<(&Transform, &NPC)>,
    mut factions: ResMut<Factions>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    player_trans: Query<&Transform, With<Player>>,
) {
    if keyboard.just_pressed(settings.key_bindings.interact) {
        if let Ok(player_pos) = player_trans.get_single() {
            let mut player = player_query.single_mut();
            for (npc_trans, npc) in npc_query.iter() {
                if player_pos.translation.distance(npc_trans.translation) < map::TILE_SIZE * 2.0 {
                    if let Some(faction_id) = npc.faction_id {
                        if player.faction_id.is_none() && factions.factions[faction_id].reputation >= 100 && 
                           factions.factions[faction_id].allied_classes.contains(&player.social_class) {
                            player.faction_id = Some(faction_id);
                            info!("Joined faction: {}", factions.factions[faction_id].name);
                        }
                    }
                }
            }
        }
    }
}
