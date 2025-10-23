use bevy::prelude::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use super::resources::*;
use super::components::*;
use std::fs;

#[derive(Event, Clone, Copy, Default)]
pub struct SaveGame;

pub fn save_game(
    mut commands: Commands,
    mut save_events: EventReader<SaveGame>,
    player_query: Query<(&Transform, &Player)>,
                 inventory: Res<Inventory>,
                 quests: Res<Quests>,
                 factions: Res<Factions>,
                 world_map: Res<WorldMap>,
                 environment: Res<Environment>,
) {
    for _ in save_events.read() {
        if let Ok((transform, player)) = player_query.get_single() {
            let save_data = SaveData {
                player_position: transform.translation.truncate(),
                player_health: player.health,
                player_mana: player.mana,
                player_experience: player.experience,
                player_level: player.level,
                player_faction_id: player.faction_id,
                player_skills: player.skills.clone(),
                player_social_class: player.social_class,
                gold: inventory.gold,
                inventory: inventory.items.clone(),
                quests: quests.clone(),
                factions: factions.clone(),
                map: world_map.tiles.clone(),
                regions: world_map.regions.clone(),
                environment: environment.clone(),
            };
            let serialized = to_string_pretty(&save_data, PrettyConfig::default()).unwrap();
            fs::write("save.ron", serialized).expect("Failed to save game");
            info!("Game saved");
        }
    }
}

pub fn load_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(data) = fs::read_to_string("save.ron") {
        if let Ok(save_data) = from_str::<SaveData>(&data) {
            // Wyczyść świat przed załadowaniem
            commands.remove_resource::<WorldMap>();
            commands.remove_resource::<Quests>();
            commands.remove_resource::<Factions>();
            commands.remove_resource::<Inventory>();
            commands.remove_resource::<Environment>();

            // Wstaw nowe zasoby
            commands.insert_resource(WorldMap {
                tiles: save_data.map,
                regions: save_data.regions,
            });
            commands.insert_resource(Quests {
                active: save_data.quests.active,
                completed: save_data.quests.completed,
            });
            commands.insert_resource(Factions {
                factions: save_data.factions.factions,
            });
            commands.insert_resource(Inventory {
                items: save_data.inventory,
                gold: save_data.gold,
                capacity: 20,
            });
            commands.insert_resource(Environment {
                time_of_day: save_data.environment.time_of_day,
                weather: save_data.environment.weather,
            });

            // Spawn gracza
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(save_data.player_position.extend(1.0)),
                            sprite: Sprite {
                                color: Color::srgb(0.0, 0.0, 1.0),
                            custom_size: Some(Vec2::new(32.0, 32.0)),
                            ..default()
                            },
                            ..default()
                },
                Player {
                    health: save_data.player_health,
                    mana: save_data.player_mana,
                    experience: save_data.player_experience,
                    level: save_data.player_level,
                    faction_id: save_data.player_faction_id,
                    skills: save_data.player_skills,
                    social_class: save_data.player_social_class,
                },
                LightSource {
                    intensity: 0.8,
                    radius: 200.0,
                },
            ));

            next_state.set(GameState::InGame);
        }
    }
}
