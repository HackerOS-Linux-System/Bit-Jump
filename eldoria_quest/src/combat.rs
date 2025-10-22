use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::map::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, combat_system.run_if(in_state(GameState::InGame)));
    }
}

fn combat_system(
    mut player_query: Query<(&Transform, &mut Player)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    mut commands: Commands,
    mut quests: ResMut<Quests>,
) {
    if let Ok((player_trans, mut player)) = player_query.get_single_mut() {
        if keyboard.just_pressed(settings.key_bindings.attack) {
            let damage = 20 * player.level * match settings.difficulty {
                Difficulty::Easy => 1.2,
                Difficulty::Normal => 1.0,
                Difficulty::Hard => 0.8,
            } as i32 * match player.social_class {
                SocialClass::Knight => 1.5,
                SocialClass::Bandit => 1.3,
                SocialClass::Mage => 1.7,
                _ => 1.0,
            } as i32;
            let defense = player.skills.iter()
                .filter(|s| matches!(s.effect, SkillEffect::Buff(Stat::Defense, _, _)))
                .map(|s| if let SkillEffect::Buff(_, value, _) = s.effect { value } else { 0 })
                .sum::<i32>();
            for (entity, enemy_trans, mut enemy) in enemy_query.iter_mut() {
                if player_trans.translation.distance(enemy_trans.translation) < TILE_SIZE * 2.0 {
                    enemy.health -= damage;
                    if enemy.health <= 0 {
                        commands.entity(entity).despawn();
                        player.experience += match enemy.enemy_type {
                            EnemyType::Dragon => 300,
                            EnemyType::Troll => 180,
                            EnemyType::RogueKnight => 150,
                            EnemyType::DarkMage => 120,
                            EnemyType::Wolf => 100,
                            _ => 60,
                        };
                        for quest in quests.active.iter_mut() {
                            for objective in quest.objectives.iter_mut() {
                                if objective.target_type == ObjectiveType::Kill && objective.target == enemy.enemy_type.to_string() {
                                    objective.current += 1;
                                }
                            }
                        }
                    } else {
                        let enemy_damage = (enemy.damage * match settings.difficulty {
                            Difficulty::Easy => 0.8,
                            Difficulty::Normal => 1.0,
                            Difficulty::Hard => 1.2,
                        } as i32 - defense).max(1);
                        player.health -= enemy_damage;
                        if player.health <= 0 {
                            commands.insert_resource(NextState::Pending(GameState::MainMenu));
                        }
                    }
                }
            }
        }
    }
}
