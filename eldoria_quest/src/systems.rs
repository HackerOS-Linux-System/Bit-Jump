use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::map::*;
use super::quests::*;
use super::combat::*;
use super::inventory::*;
use super::factions::*;
use super::environment::*;
use super::skills::*;
use super::trading::*;
use super::events::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::NewGame), setup_game)
            .add_systems(Update, (
                player_movement,
                npc_interaction,
                enemy_ai,
                update_lighting,
                update_fog_of_war,
                toggle_pause,
                update_hud,
                restock_shops,
                work_system,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(Update, handle_pause_menu.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::InGame), cleanup_game)
            .add_systems(OnEnter(GameState::LoadGame), load_game);
    }
}

const PLAYER_SPEED: f32 = 200.0;

fn setup_game(
    mut commands: Commands,
    mut quests: ResMut<Quests>,
    mut factions: ResMut<Factions>,
    mut skills: ResMut<Skills>,
    mut inventory: ResMut<Inventory>,
    mut events: ResMut<RandomEvents>,
    class_selection: Option<Res<PlayerClassSelection>>,
) {
    inventory.capacity = 20;
    inventory.gold = 50;

    // Initialize factions
    factions.factions = vec![
        Faction { id: 0, name: "Knights of Eldoria".to_string(), reputation: 0, allied_classes: vec![SocialClass::Knight, SocialClass::Noble] },
        Faction { id: 1, name: "Shadow Guild".to_string(), reputation: 0, allied_classes: vec![SocialClass::Bandit] },
        Faction { id: 2, name: "Druid Circle".to_string(), reputation: 0, allied_classes: vec![SocialClass::Priest, SocialClass::Peasant, SocialClass::Mage] },
        Faction { id: 3, name: "Merchant League".to_string(), reputation: 0, allied_classes: vec![SocialClass::Townsfolk, SocialClass::Noble] },
        Faction { id: 4, name: "Peasant Union".to_string(), reputation: 0, allied_classes: vec![SocialClass::Peasant, SocialClass::Townsfolk] },
        Faction { id: 5, name: "Arcane Order".to_string(), reputation: 0, allied_classes: vec![SocialClass::Mage, SocialClass::Priest] },
    ];

    // Initialize skills
    skills.available_skills = vec![
        Skill { name: "Fireball".to_string(), level: 1, mana_cost: 20, cooldown: 5.0, effect: SkillEffect::Damage(30) },
        Skill { name: "Heal".to_string(), level: 1, mana_cost: 15, cooldown: 10.0, effect: SkillEffect::Heal(25) },
        Skill { name: "Speed Boost".to_string(), level: 1, mana_cost: 10, cooldown: 15.0, effect: SkillEffect::Buff(Stat::Speed, 50, 10.0) },
        Skill { name: "Defensive Aura".to_string(), level: 1, mana_cost: 25, cooldown: 20.0, effect: SkillEffect::Buff(Stat::Defense, 20, 15.0) },
        Skill { name: "Arcane Blast".to_string(), level: 1, mana_cost: 30, cooldown: 8.0, effect: SkillEffect::Damage(40) },
        Skill { name: "Mana Shield".to_string(), level: 1, mana_cost: 20, cooldown: 12.0, effect: SkillEffect::Buff(Stat::Defense, 30, 10.0) },
    ];

    // Initialize events
    events.events = vec![
        Event { description: "Bandit Ambush".to_string(), effect: EventEffect::SpawnEnemies(EnemyType::Bandit, 4), duration: 30.0 },
        Event { description: "Treasure Discovery".to_string(), effect: EventEffect::DropItems(Item { name: "Gold".to_string(), value: 50, item_type: ItemType::Currency }, 2), duration: 10.0 },
        Event { description: "Sudden Storm".to_string(), effect: EventEffect::WeatherChange(Weather::Storm), duration: 60.0 },
        Event { description: "Arcane Surge".to_string(), effect: EventEffect::SpawnEnemies(EnemyType::DarkMage, 3), duration: 20.0 },
        Event { description: "Peasant Uprising".to_string(), effect: EventEffect::FactionBoost(4, 50), duration: 20.0 },
    ];

    // Spawn player
    let player_class = class_selection.map(|c| c.0.clone()).unwrap_or(SocialClass::Peasant);
    let (health, mana, skills) = match player_class {
        SocialClass::Knight => (120, 30, vec![skills.available_skills[0].clone(), skills.available_skills[3].clone()]),
        SocialClass::Noble => (80, 50, vec![skills.available_skills[2].clone()]),
        SocialClass::Priest => (90, 70, vec![skills.available_skills[1].clone(), skills.available_skills[3].clone()]),
        SocialClass::Townsfolk => (100, 40, vec![skills.available_skills[2].clone()]),
        SocialClass::Peasant => (100, 30, vec![skills.available_skills[0].clone()]),
        SocialClass::Bandit => (110, 35, vec![skills.available_skills[0].clone()]),
        SocialClass::Mage => (80, 100, vec![skills.available_skills[4].clone(), skills.available_skills[5].clone()]),
    };
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                color: match player_class {
                    SocialClass::Knight => Color::rgb(0.0, 0.0, 1.0),
                    SocialClass::Noble => Color::rgb(0.5, 0.0, 0.5),
                    SocialClass::Priest => Color::rgb(1.0, 1.0, 1.0),
                    SocialClass::Townsfolk => Color::rgb(0.0, 0.5, 0.5),
                    SocialClass::Peasant => Color::rgb(0.5, 0.3, 0.2),
                    SocialClass::Bandit => Color::rgb(0.3, 0.1, 0.1),
                    SocialClass::Mage => Color::rgb(0.2, 0.2, 0.8),
                },
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        Player {
            health,
            mana,
            experience: 0,
            level: 1,
            faction_id: None,
            social_class: player_class,
            skills,
        },
        LightSource {
            intensity: 0.8,
            radius: 250.0,
        },
    ));

    // Spawn NPCs
    let mut rng = rand::thread_rng();
    for i in 0..300 {
        let x = rng.gen_range(0..MAP_WIDTH) as f32 * TILE_SIZE;
        let y = rng.gen_range(0..MAP_HEIGHT) as f32 * TILE_SIZE;
        let social_class = match i % 7 {
            0 => SocialClass::Knight,
            1 => SocialClass::Noble,
            2 => SocialClass::Priest,
            3 => SocialClass::Townsfolk,
            4 => SocialClass::Peasant,
            5 => SocialClass::Bandit,
            _ => SocialClass::Mage,
        };
        let quest = if i % 2 == 0 { Some(quests.active.len()) } else { None };
        let faction_id = match social_class {
            SocialClass::Knight => Some(0),
            SocialClass::Noble => Some(3),
            SocialClass::Priest => Some(2),
            SocialClass::Townsfolk => Some(3),
            SocialClass::Peasant => Some(4),
            SocialClass::Bandit => Some(1),
            SocialClass::Mage => Some(5),
        };
        let trade_inventory = if social_class == SocialClass::Townsfolk || social_class == SocialClass::Noble || social_class == SocialClass::Mage {
            vec![
                Item { name: "Sword".to_string(), value: 1, item_type: ItemType::Weapon },
                Item { name: "Potion".to_string(), value: 1, item_type: ItemType::Consumable },
                Item { name: "Gold".to_string(), value: 20, item_type: ItemType::Currency },
                Item { name: "Mana Crystal".to_string(), value: 1, item_type: ItemType::Consumable },
            ]
        } else {
            vec![]
        };
        let job_offer = if social_class == SocialClass::Townsfolk || social_class == SocialClass::Noble || social_class == SocialClass::Knight {
            Some(Job {
                description: format!("{} job for {}", social_class.to_string(), i),
                reward: Item { name: "Gold".to_string(), value: rng.gen_range(10..30), item_type: ItemType::Currency },
                duration: rng.gen_range(20.0..60.0),
            })
        } else {
            None
        };
        if let Some(id) = quest {
            quests.active.push(Quest {
                id,
                description: format!("Quest {}: {}", id, match id % 6 {
                    0 => "Collect resources",
                    1 => "Slay enemies",
                    2 => "Deliver message",
                    3 => "Explore ruins",
                    4 => "Escort NPC",
                    _ => "Defend village",
                }),
                reward: Item {
                    name: match id % 6 {
                        0 => "Sword".to_string(),
                        1 => "Shield".to_string(),
                        2 => "Potion".to_string(),
                        3 => "Gold".to_string(),
                        4 => "Holy Relic".to_string(),
                        _ => "Mana Crystal".to_string(),
                    },
                    value: 10 * (id + 1) as i32,
                    item_type: match id % 6 {
                        0 => ItemType::Weapon,
                        1 => ItemType::Armor,
                        2 => ItemType::Consumable,
                        3 => ItemType::Currency,
                        4 => ItemType::QuestItem,
                        _ => ItemType::Consumable,
                    },
                },
                completed: false,
                objectives: vec![Objective {
                    description: match id % 6 {
                        0 => "Gather materials".to_string(),
                        1 => "Kill enemies".to_string(),
                        2 => "Deliver to NPC".to_string(),
                        3 => "Explore location".to_string(),
                        4 => "Escort to destination".to_string(),
                        _ => "Defend against enemies".to_string(),
                    },
                    target: match id % 6 {
                        0 => "Wood".to_string(),
                        1 => "Bandit".to_string(),
                        2 => "Letter".to_string(),
                        3 => "Ruins".to_string(),
                        4 => "Village".to_string(),
                        _ => "Village".to_string(),
                    },
                    count: (id % 5 + 1) as i32,
                    current: 0,
                    target_type: match id % 6 {
                        0 => ObjectiveType::Collect,
                        1 => ObjectiveType::Kill,
                        2 => ObjectiveType::Deliver,
                        3 => ObjectiveType::Explore,
                        4 => ObjectiveType::Escort,
                        _ => ObjectiveType::Defend,
                    },
                }],
                faction_id,
                required_class: Some(social_class.clone()),
            });
        }
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 1.0),
                sprite: Sprite {
                    color: match social_class {
                        SocialClass::Knight => Color::rgb(0.0, 0.0, 0.8),
                        SocialClass::Noble => Color::rgb(0.6, 0.0, 0.6),
                        SocialClass::Priest => Color::rgb(0.9, 0.9, 0.9),
                        SocialClass::Townsfolk => Color::rgb(0.0, 0.6, 0.6),
                        SocialClass::Peasant => Color::rgb(0.6, 0.4, 0.3),
                        SocialClass::Bandit => Color::rgb(0.4, 0.1, 0.1),
                        SocialClass::Mage => Color::rgb(0.3, 0.3, 0.9),
                    },
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            NPC {
                name: format!("{} {}", social_class.to_string(), i),
                dialogue: vec![format!("Greetings from a {}!", social_class.to_string())],
                quest_id: quest,
                faction_id,
                social_class,
                trade_inventory,
                job_offer,
            },
        ));
    }

    // Spawn enemies
    for i in 0..150 {
        let x = rng.gen_range(0..MAP_WIDTH) as f32 * TILE_SIZE;
        let y = rng.gen_range(0..MAP_HEIGHT) as f32 * TILE_SIZE;
        let enemy_type = match i % 8 {
            0 => EnemyType::Bandit,
            1 => EnemyType::Goblin,
            2 => EnemyType::Dragon,
            3 => EnemyType::Troll,
            4 => EnemyType::Undead,
            5 => EnemyType::Wolf,
            6 => EnemyType::RogueKnight,
            _ => EnemyType::DarkMage,
        };
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 1.0),
                sprite: Sprite {
                    color: match enemy_type {
                        EnemyType::RogueKnight => Color::rgb(0.3, 0.3, 0.7),
                        EnemyType::DarkMage => Color::rgb(0.2, 0.2, 0.6),
                        _ => Color::rgb(1.0, 0.0, 0.0),
                    },
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Enemy {
                health: match enemy_type {
                    EnemyType::Dragon => 300,
                    EnemyType::Troll => 180,
                    EnemyType::RogueKnight => 150,
                    EnemyType::DarkMage => 120,
                    EnemyType::Wolf => 100,
                    _ => 60,
                },
                damage: match enemy_type {
                    EnemyType::Dragon => 35,
                    EnemyType::RogueKnight => 20,
                    EnemyType::DarkMage => 25,
                    EnemyType::Wolf => 15,
                    _ => 12,
                },
                enemy_type,
                faction_id: if i % 3 == 0 { Some(rng.gen_range(0..6)) } else { None },
            },
        ));
    }

    // Spawn items
    for _ in 0..400 {
        let x = rng.gen_range(0..MAP_WIDTH) as f32 * TILE_SIZE;
        let y = rng.gen_range(0..MAP_HEIGHT) as f32 * TILE_SIZE;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 1.0),
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.5, TILE_SIZE * 0.5)),
                    ..default()
                },
                ..default()
            },
            Item {
                name: match rng.gen_range(0..8) {
                    0 => "Wood".to_string(),
                    1 => "Stone".to_string(),
                    2 => "Apple".to_string(),
                    3 => "Potion".to_string(),
                    4 => "Gold".to_string(),
                    5 => "Iron".to_string(),
                    6 => "Wheat".to_string(),
                    _ => "Mana Crystal".to_string(),
                },
                value: match rng.gen_range(0..8) {
                    4 => 15,
                    7 => 10,
                    _ => 1,
                },
                item_type: match rng.gen_range(0..8) {
                    0 | 1 | 5 | 6 => ItemType::Material,
                    2 | 3 | 7 => ItemType::Consumable,
                    _ => ItemType::Currency,
                },
            },
        ));
    }

    // Spawn shops
    for region in commands.get_resource::<WorldMap>().unwrap().regions.iter() {
        if region.tile_type == TileType::Town || region.tile_type == TileType::Village || region.tile_type == TileType::Castle {
            let (x, y) = region.center;
            let shop_inventory = if region.tile_type == TileType::Castle {
                vec![
                    Item { name: "Greatsword".to_string(), value: 1, item_type: ItemType::Weapon },
                    Item { name: "Plate Armor".to_string(), value: 1, item_type: ItemType::Armor },
                    Item { name: "Mana Crystal".to_string(), value: 2, item_type: ItemType::Consumable },
                    Item { name: "Royal Crest".to_string(), value: 1, item_type: ItemType::QuestItem },
                ]
            } else {
                vec![
                    Item { name: "Sword".to_string(), value: 1, item_type: ItemType::Weapon },
                    Item { name: "Shield".to_string(), value: 1, item_type: ItemType::Armor },
                    Item { name: "Potion".to_string(), value: 2, item_type: ItemType::Consumable },
                    Item { name: "Apple".to_string(), value: 3, item_type: ItemType::Consumable },
                ]
            };
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz((x as f32) * TILE_SIZE, (y as f32) * TILE_SIZE, 1.0),
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.0),
                        custom_size: Some(Vec2::new(TILE_SIZE * 1.5, TILE_SIZE * 1.5)),
                        ..default()
                    },
                    ..default()
                },
                Shop {
                    inventory: shop_inventory,
                    restock_timer: 60.0,
                },
            ));
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
    camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    world_map: Res<WorldMap>,
    environment: Res<Environment>,
    settings: Res<GameSettings>,
) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(settings.key_bindings.move_left) { direction.x -= 1.0; }
        if keyboard.pressed(settings.key_bindings.move_right) { direction.x += 1.0; }
        if keyboard.pressed(settings.key_bindings.move_up) { direction.y += 1.0; }
        if keyboard.pressed(settings.key_bindings.move_down) { direction.y -= 1.0; }
        if direction.length() > 0.0 {
            direction = direction.normalize();
            let speed_modifier = match environment.weather {
                Weather::Rain | Weather::Storm | Weather::Snow => 0.7,
                Weather::Fog => 0.9,
                _ => 1.0,
            };
            let base_speed = PLAYER_SPEED * (1.0 + player.skills.iter()
                .filter(|s| matches!(s.effect, SkillEffect::Buff(Stat::Speed, _, _)))
                .map(|s| if let SkillEffect::Buff(_, value, _) = s.effect { value as f32 / 100.0 } else { 0.0 })
                .sum::<f32>());
            let new_pos = transform.translation + direction * base_speed * speed_modifier * time.delta_seconds();
            let tile_x = (new_pos.x / TILE_SIZE) as usize;
            let tile_y = (new_pos.y / TILE_SIZE) as usize;
            if tile_x < MAP_WIDTH && tile_y < MAP_HEIGHT && world_map.tiles[tile_x][tile_y] != TileType::Water {
                transform.translation = new_pos;
            }
        }
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}

fn npc_interaction(
    player_query: Query<(&Transform, &Player)>,
    npc_query: Query<(Entity, &Transform, &NPC)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    mut quests: ResMut<Quests>,
    mut factions: ResMut<Factions>,
    mut trade: EventWriter<StartTrade>,
    mut work_writer: EventWriter<StartWork>,
) {
    if keyboard.just_pressed(settings.key_bindings.interact) {
        if let Ok((player_trans, player)) = player_query.get_single() {
            for (entity, npc_trans, npc) in npc_query.iter() {
                if player_trans.translation.distance(npc_trans.translation) < TILE_SIZE * 2.0 {
                    info!("Interacting with {}", npc.name);
                    if let Some(quest_id) = npc.quest_id {
                        let quest = &mut quests.active[quest_id];
                        if !quest.completed && quest.required_class.as_ref().map_or(true, |c| *c == player.social_class) {
                            if let Some(faction_id) = npc.faction_id {
                                if player.faction_id != Some(faction_id) && factions.factions[faction_id].reputation < 50 {
                                    info!("Need more reputation with {}", factions.factions[faction_id].name);
                                    continue;
                                }
                            }
                            // Accept quest
                        }
                    }
                    if !npc.trade_inventory.is_empty() {
                        trade.send(StartTrade { npc_entity: entity });
                    }
                    if let Some(job) = &npc.job_offer {
                        work_writer.send(StartWork { npc_entity: entity, job: job.clone() });
                    }
                }
            }
        }
    }
}

fn work_system(
    mut work_reader: EventReader<StartWork>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut inventory: ResMut<Inventory>,
    time: Res<Time>,
    world_map: Res<WorldMap>,
) {
    for event in work_reader.read() {
        if let Ok((mut player, player_trans)) = player_query.get_single_mut() {
            let tile_x = (player_trans.translation.x / TILE_SIZE) as usize;
            let tile_y = (player_trans.translation.y / TILE_SIZE) as usize;
            if tile_x < MAP_WIDTH && tile_y < MAP_HEIGHT && world_map.tiles[tile_x][tile_y] == TileType::Castle {
                inventory.gold += event.job.reward.value;
                info!("Completed job: {}. Earned {} gold.", event.job.description, event.job.reward.value);
            }
        }
    }
}

fn enemy_ai(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    environment: Res<Environment>,
    world_map: Res<WorldMap>,
) {
    if let Ok(player_trans) = player_query.get_single() {
        for (mut enemy_trans, enemy) in enemy_query.iter_mut() {
            let visibility = match environment.weather {
                Weather::Fog => 2.0,
                Weather::Snow => 3.0,
                _ => 6.0,
            };
            let direction = player_trans.translation - enemy_trans.translation;
            if direction.length() < TILE_SIZE * visibility {
                let speed = match enemy.enemy_type {
                    EnemyType::Dragon => 150.0,
                    EnemyType::RogueKnight => 130.0,
                    EnemyType::DarkMage => 140.0,
                    EnemyType::Wolf => 120.0,
                    _ => 100.0,
                };
                let new_pos = enemy_trans.translation + direction.normalize() * speed * time.delta_seconds();
                let tile_x = (new_pos.x / TILE_SIZE) as usize;
                let tile_y = (new_pos.y / TILE_SIZE) as usize;
                if tile_x < MAP_WIDTH && tile_y < MAP_HEIGHT && world_map.tiles[tile_x][tile_y] != TileType::Water {
                    enemy_trans.translation = new_pos;
                }
            }
        }
    }
}

fn update_lighting(
    light_query: Query<(&Transform, &LightSource)>,
    mut tile_query: Query<(&Transform, &mut Tile, &mut Sprite)>,
    environment: Res<Environment>,
) {
    let ambient_light = match environment.time_of_day {
        t if t < 0.25 || t > 0.75 => 0.2,
        t if t < 0.3 || t > 0.7 => 0.4,
        _ => 0.8,
    } * match environment.weather {
        Weather::Fog | Weather::Storm | Weather::Snow => 0.6,
        Weather::Rain => 0.8,
        Weather::Clear => 1.0,
    };
    for (tile_trans, mut tile, mut sprite) in tile_query.iter_mut() {
        let mut total_light = ambient_light;
        for (light_trans, light) in light_query.iter() {
            let distance = tile_trans.translation.distance(light_trans.translation);
            if distance < light.radius {
                total_light += light.intensity * (1.0 - distance / light.radius);
            }
        }
        tile.visibility = total_light.min(1.0);
        sprite.color.set_alpha(tile.visibility);
    }
}

fn update_fog_of_war(
    player_query: Query<&Transform, With<Player>>,
    mut tile_query: Query<(&Transform, &mut Tile)>,
) {
    if let Ok(player_trans) = player_query.get_single() {
        for (tile_trans, mut tile) in tile_query.iter_mut() {
            let distance = player_trans.translation.distance(tile_trans.translation);
            if distance < TILE_SIZE * 20.0 {
                tile.visibility = (tile.visibility + 0.1).min(1.0);
            }
        }
    }
}

fn restock_shops(
    mut shop_query: Query<&mut Shop>,
    time: Res<Time>,
    world_map: Res<WorldMap>,
) {
    for mut shop in shop_query.iter_mut() {
        shop.restock_timer -= time.delta_seconds();
        if shop.restock_timer <= 0.0 {
            shop.inventory = world_map.regions.iter()
                .find(|r| {
                    let (x, y) = r.center;
                    let shop_x = (shop.transform.translation.x / TILE_SIZE) as usize;
                    let shop_y = (shop.transform.translation.y / TILE_SIZE) as usize;
                    (shop_x as i32 - x as i32).pow(2) + (shop_y as i32 - y as i32).pow(2) <= (r.radius as i32).pow(2)
                })
                .map_or_else(
                    || vec![
                        Item { name: "Sword".to_string(), value: 1, item_type: ItemType::Weapon },
                        Item { name: "Shield".to_string(), value: 1, item_type: ItemType::Armor },
                        Item { name: "Potion".to_string(), value: 2, item_type: ItemType::Consumable },
                        Item { name: "Apple".to_string(), value: 3, item_type: ItemType::Consumable },
                    ],
                    |r| if r.tile_type == TileType::Castle {
                        vec![
                            Item { name: "Greatsword".to_string(), value: 1, item_type: ItemType::Weapon },
                            Item { name: "Plate Armor".to_string(), value: 1, item_type: ItemType::Armor },
                            Item { name: "Mana Crystal".to_string(), value: 2, item_type: ItemType::Consumable },
                            Item { name: "Royal Crest".to_string(), value: 1, item_type: ItemType::QuestItem },
                        ]
                    } else {
                        vec![
                            Item { name: "Sword".to_string(), value: 1, item_type: ItemType::Weapon },
                            Item { name: "Shield".to_string(), value: 1, item_type: ItemType::Armor },
                            Item { name: "Potion".to_string(), value: 2, item_type: ItemType::Consumable },
                            Item { name: "Apple".to_string(), value: 3, item_type: ItemType::Consumable },
                        ]
                    },
                );
            shop.restock_timer = 60.0;
        }
    }
}

fn update_hud(
    mut commands: Commands,
    player_query: Query<&Player>,
    inventory: Res<Inventory>,
    quests: Res<Quests>,
    factions: Res<Factions>,
    query: Query<Entity, With<Node>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if let Ok(player) = player_query.get_single() {
        commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Class: {}\nHealth: {}\nMana: {}\nLevel: {} (XP: {})\nGold: {}", 
                    player.social_class.to_string(), player.health, player.mana, player.level, player.experience, inventory.gold),
                TextStyle { font_size: 20.0, color: Color::WHITE, ..default() },
            ));
            parent.spawn(TextBundle::from_section(
                format!("Inventory ({} / {}):", inventory.items.len(), inventory.capacity),
                TextStyle { font_size: 20.0, color: Color::WHITE, ..default() },
            ));
            for item in &inventory.items {
                parent.spawn(TextBundle::from_section(
                    format!("{}: {}", item.name, item.value),
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
            }
            parent.spawn(TextBundle::from_section(
                "Active Quests:",
                TextStyle { font_size: 20.0, color: Color::WHITE, ..default() },
            ));
            for quest in &quests.active {
                parent.spawn(TextBundle::from_section(
                    format!("{}: {}/{}", quest.description, quest.objectives[0].current, quest.objectives[0].count),
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
            }
            parent.spawn(TextBundle::from_section(
                "Factions:",
                TextStyle { font_size: 20.0, color: Color::WHITE, ..default() },
            ));
            for faction in &factions.factions {
                parent.spawn(TextBundle::from_section(
                    format!("{}: {} rep", faction.name, faction.reputation),
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
            }
        });
    }
}

fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn setup_pause_menu(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Paused",
            TextStyle { font_size: 60.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Resume", ButtonAction::Back);
        spawn_button(parent, "Save Game", ButtonAction::Save);
        spawn_button(parent, "Settings", ButtonAction::Settings);
        spawn_button(parent, "Quit to Menu", ButtonAction::Quit);
    });
}

fn handle_pause_menu(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut save_writer: EventWriter<SaveGame>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::Back => next_state.set(GameState::InGame),
                ButtonAction::Save => save_writer.send(SaveGame),
                ButtonAction::Settings => next_state.set(GameState::Settings),
                ButtonAction::Quit => next_state.set(GameState::MainMenu),
                _ => {},
            }
        }
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, (Without<Camera>, Without<Node>)>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
