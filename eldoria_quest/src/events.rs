use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::map::*;
use rand::Rng;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_events.run_if(in_state(GameState::InGame)))
            .add_event::<TriggerEvent>();
    }
}

#[derive(Event)]
pub struct TriggerEvent {
    pub event: Event,
}

fn handle_events(
    mut commands: Commands,
    mut events: EventReader<TriggerEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_trans) = player_query.get_single() {
        let mut rng = rand::thread_rng();
        for event in events.read() {
            match &event.event.effect {
                EventEffect::SpawnEnemies(enemy_type, count) => {
                    for _ in 0..*count {
                        let offset_x = rng.gen_range(-5.0..5.0) * TILE_SIZE;
                        let offset_y = rng.gen_range(-5.0..5.0) * TILE_SIZE;
                        commands.spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(
                                    player_trans.translation.x + offset_x,
                                    player_trans.translation.y + offset_y,
                                    1.0,
                                ),
                                sprite: Sprite {
                                    color: Color::rgb(1.0, 0.0, 0.0),
                                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                    ..default()
                                },
                                ..default()
                            },
                            Enemy {
                                health: match enemy_type {
                                    EnemyType::Dragon => 250,
                                    EnemyType::Troll => 150,
                                    EnemyType::Wolf => 80,
                                    _ => 50,
                                },
                                damage: match enemy_type {
                                    EnemyType::Dragon => 30,
                                    EnemyType::Wolf => 15,
                                    _ => 10,
                                },
                                enemy_type: enemy_type.clone(),
                                faction_id: None,
                            },
                        ));
                    }
                },
                EventEffect::DropItems(item, count) => {
                    for _ in 0..*count {
                        let offset_x = rng.gen_range(-3.0..3.0) * TILE_SIZE;
                        let offset_y = rng.gen_range(-3.0..3.0) * TILE_SIZE;
                        commands.spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(
                                    player_trans.translation.x + offset_x,
                                    player_trans.translation.y + offset_y,
                                    1.0,
                                ),
                                sprite: Sprite {
                                    color: Color::rgb(1.0, 1.0, 0.0),
                                    custom_size: Some(Vec2::new(TILE_SIZE * 0.5, TILE_SIZE * 0.5)),
                                    ..default()
                                },
                                ..default()
                            },
                            item.clone(),
                        ));
                    }
                },
                EventEffect::WeatherChange(_) => {}, // Handled in environment
            }
        }
    }
}
