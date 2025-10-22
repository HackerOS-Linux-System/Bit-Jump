use bevy::prelude::*;
use super::resources::*;
use super::components::*;
use super::map::*;

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_quests.run_if(in_state(GameState::InGame)));
    }
}

fn update_quests(
    mut quests: ResMut<Quests>,
    inventory: Res<Inventory>,
    mut player: Query<(&mut Player, &Transform)>,
    enemy_query: Query<(&Enemy, &Transform)>,
    npc_query: Query<(&NPC, &Transform)>,
    world_map: Res<WorldMap>,
    mut factions: ResMut<Factions>,
) {
    let (mut player, player_trans) = player.single_mut();
    for quest in quests.active.iter_mut() {
        for objective in quest.objectives.iter_mut() {
            match objective.target_type {
                ObjectiveType::Collect => {
                    objective.current = inventory.items.iter()
                        .filter(|item| item.name == objective.target)
                        .map(|item| item.value)
                        .sum();
                },
                ObjectiveType::Kill => {
                    objective.current = enemy_query.iter()
                        .filter(|(enemy, _)| enemy.enemy_type.to_string() == objective.target)
                        .count() as i32;
                },
                ObjectiveType::Deliver => {
                    for (npc, npc_trans) in npc_query.iter() {
                        if npc_trans.translation.distance(player_trans.translation) < TILE_SIZE * 2.0 {
                            objective.current = inventory.items.iter()
                                .filter(|item| item.name == objective.target)
                                .count() as i32;
                        }
                    }
                },
                ObjectiveType::Explore => {
                    let tile_x = (player_trans.translation.x / TILE_SIZE) as usize;
                    let tile_y = (player_trans.translation.y / TILE_SIZE) as usize;
                    if tile_x < MAP_WIDTH && tile_y < MAP_HEIGHT && world_map.tiles[tile_x][tile_y].to_string() == objective.target {
                        objective.current = 1;
                    }
                },
                ObjectiveType::Escort => {
                    for (npc, npc_trans) in npc_query.iter() {
                        if npc_trans.translation.distance(player_trans.translation) < TILE_SIZE * 2.0 {
                            let tile_x = (npc_trans.translation.x / TILE_SIZE) as usize;
                            let tile_y = (npc_trans.translation.y / TILE_SIZE) as usize;
                            if tile_x < MAP_WIDTH && tile_y < MAP_HEIGHT && world_map.tiles[tile_x][tile_y].to_string() == objective.target {
                                objective.current = 1;
                            }
                        }
                    }
                },
                ObjectiveType::Defend => {
                    objective.current = enemy_query.iter()
                        .filter(|(_, trans)| {
                            let tile_x = (trans.translation.x / TILE_SIZE) as usize;
                            let tile_y = (trans.translation.y / TILE_SIZE) as usize;
                            tile_x < MAP_WIDTH && tile_y < MAP_HEIGHT && world_map.tiles[tile_x][tile_y].to_string() == objective.target
                        })
                        .count() as i32;
                },
            }
            if objective.current >= objective.count {
                quest.completed = true;
                player.experience += 150 * objective.count;
                if let Some(faction_id) = quest.faction_id {
                    factions.factions[faction_id].reputation += 30;
                }
                inventory.items.push(quest.reward.clone());
                if player.experience >= player.level * 1000 {
                    player.level += 1;
                    player.experience = 0;
                    player.health += 25;
                    player.mana += 20;
                    if player.level % 3 == 0 {
                        let new_skill = Skills::available_skills[player.skills.len() % Skills::available_skills.len()].clone();
                        player.skills.push(new_skill);
                    }
                }
            }
        }
    }
}
