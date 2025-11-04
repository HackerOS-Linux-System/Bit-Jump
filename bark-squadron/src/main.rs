use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

mod menu;
use menu::{menu_plugin, AppState};

#[derive(Resource)]
pub struct GameConfig {
    day_mode: bool,
    map: String,
    difficulty: String,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    health: i32,
    powerup_active: bool,
    powerup_timer: Timer,
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
    enemy_type: EnemyType,
}

#[derive(PartialEq)]
pub enum EnemyType {
    Normal,
    Fast,
    Boss,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

#[derive(Component)]
pub struct PowerUp {
    speed: f32,
}

#[derive(Resource)]
pub struct GameStats {
    score: u32,
    level: u32,
}

#[derive(Component)]
pub struct ScoreText;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bark Squadron".into(),
                             resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                             resizable: false,
                             ..default()
        }),
        ..default()
    }))
    .init_state::<AppState>()
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .insert_resource(GameConfig {
        day_mode: true,
        map: "Sky High".to_string(),
                     difficulty: "Normal".to_string(),
    })
    .add_plugins(menu_plugin)
    .add_systems(OnEnter(AppState::Game), setup_game)
    .add_systems(Update, (
        player_movement,
        shoot_bullets,
        move_bullets,
        spawn_enemies,
        move_enemies,
        spawn_powerups,
        move_powerups,
        check_collisions,
        update_powerup,
        update_score_text,
        level_progression,
        check_game_over,
    ).run_if(in_state(AppState::Game)))
    .run();
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
    mut clear_color: ResMut<ClearColor>,
) {
    if config.day_mode {
        clear_color.0 = Color::rgb(0.8, 0.8, 1.0); // Light blue for day
    } else {
        clear_color.0 = Color::rgb(0.1, 0.1, 0.2); // Dark for night
    }

    // Spawn camera if not already (but DefaultPlugins has it)
    commands.spawn(Camera2dBundle::default());

    // Player
    let player_speed = match config.difficulty.as_str() {
        "Easy" => 400.0,
        "Normal" => 300.0,
        "Hard" => 200.0,
        _ => 300.0,
    };
    let player_health = match config.difficulty.as_str() {
        "Easy" => 5,
        "Normal" => 3,
        "Hard" => 1,
        _ => 3,
    };
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player.png"), // Assume user provides assets or replace with shape
                    transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 50.0, 0.0),
                    ..default()
        },
        Player {
            speed: player_speed,
            health: player_health,
            powerup_active: false,
            powerup_timer: Timer::from_seconds(10.0, TimerMode::Once),
        },
    ));

    // Score text
    commands.spawn((
        TextBundle::from_section(
            "Score: 0\nLevel: 1\nHealth: 3",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
        }),
        ScoreText,
    ));

    commands.insert_resource(GameStats { score: 0, level: 1 });
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        let new_pos = transform.translation + direction * 300.0 * time.delta_seconds();
        let half_width = WINDOW_WIDTH / 2.0 - 25.0;
        let half_height = WINDOW_HEIGHT / 2.0 - 25.0;
        transform.translation.x = new_pos.x.clamp(-half_width, half_width);
        transform.translation.y = new_pos.y.clamp(-half_height, half_height);
    }
}

fn shoot_bullets(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, &Player)>,
) {
    if let Ok((transform, player)) = query.get_single() {
        if keys.just_pressed(KeyCode::Space) {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/bullet.png"), // Assume asset
                            transform: Transform::from_translation(transform.translation + Vec3::new(0.0, 20.0, 0.0)),
                            ..default()
                },
                Bullet { speed: 500.0 },
            ));
            if player.powerup_active {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("textures/bullet.png"),
                                transform: Transform::from_translation(transform.translation + Vec3::new(-10.0, 20.0, 0.0)),
                                ..default()
                    },
                    Bullet { speed: 500.0 },
                ));
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("textures/bullet.png"),
                                transform: Transform::from_translation(transform.translation + Vec3::new(10.0, 20.0, 0.0)),
                                ..default()
                    },
                    Bullet { speed: 500.0 },
                ));
            }
        }
    }
}

fn move_bullets(
    time: Res<Time>,
    mut query: Query<(&Bullet, &mut Transform)>,
                window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (bullet, mut transform) in &mut query {
        transform.translation.y += bullet.speed * time.delta_seconds();
        if transform.translation.y > WINDOW_HEIGHT / 2.0 {
            // Despawn if out of bounds
        }
    }
}

fn spawn_enemies(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<GameStats>,
    config: Res<GameConfig>,
) {
    static mut TIMER: f32 = 0.0;
    unsafe {
        TIMER += time.delta_seconds();
        let spawn_interval = match config.difficulty.as_str() {
            "Easy" => 1.0,
            "Normal" => 0.5,
            "Hard" => 0.3,
            _ => 0.5,
        };
        if TIMER > spawn_interval {
            TIMER = 0.0;
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 25.0..WINDOW_WIDTH / 2.0 - 25.0);
            let enemy_type = if stats.level >= 5 && rng.gen_bool(0.1) {
                EnemyType::Boss
            } else if stats.level >= 3 && rng.gen_bool(0.3) {
                EnemyType::Fast
            } else {
                EnemyType::Normal
            };
            let speed = match (&enemy_type, config.difficulty.as_str()) {
                (EnemyType::Boss, _) => 100.0,
                (EnemyType::Fast, _) => 300.0,
                (_, "Easy") => 150.0,
                (_, "Normal") => 200.0,
                (_, "Hard") => 250.0,
                _ => 200.0,
            };
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(match enemy_type {
                        EnemyType::Boss => "textures/boss.png",
                        EnemyType::Fast => "textures/fast_enemy.png",
                        _ => "textures/enemy.png",
                    }),
                    transform: Transform::from_xyz(x, WINDOW_HEIGHT / 2.0 + 50.0, 0.0),
                            ..default()
                },
                Enemy { speed, enemy_type },
            ));
        }
    }
}

fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    for (enemy, mut transform) in &mut query {
        transform.translation.y -= enemy.speed * time.delta_seconds();
    }
}

fn spawn_powerups(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.01) { // Low chance
        let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 25.0..WINDOW_WIDTH / 2.0 - 25.0);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/powerup.png"),
                        transform: Transform::from_xyz(x, WINDOW_HEIGHT / 2.0 + 50.0, 0.0),
                        ..default()
            },
            PowerUp { speed: 150.0 },
        ));
    }
}

fn move_powerups(
    time: Res<Time>,
    mut query: Query<(&PowerUp, &mut Transform)>,
) {
    for (powerup, mut transform) in &mut query {
        transform.translation.y -= powerup.speed * time.delta_seconds();
    }
}

fn check_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform, Entity), With<Player>>,
                    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
                    enemy_query: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
                    powerup_query: Query<(Entity, &Transform), With<PowerUp>>,
                    mut stats: ResMut<GameStats>,
) {
    if let Ok((mut player, player_transform, player_entity)) = player_query.get_single_mut() {
        // Bullet-Enemy collisions
        for (bullet_entity, bullet_transform) in &bullet_query {
            for (enemy_entity, enemy_transform, enemy) in &enemy_query {
                if bullet_transform.translation.distance(enemy_transform.translation) < 30.0 {
                    commands.entity(bullet_entity).despawn();
                    commands.entity(enemy_entity).despawn();
                    stats.score += match enemy.enemy_type {
                        EnemyType::Boss => 10,
                        EnemyType::Fast => 3,
                        _ => 1,
                    };
                }
            }
        }

        // Player-Enemy collisions
        for (enemy_entity, enemy_transform, enemy) in &enemy_query {
            if player_transform.translation.distance(enemy_transform.translation) < 30.0 {
                player.health -= match enemy.enemy_type {
                    EnemyType::Boss => 3,
                    EnemyType::Fast => 2,
                    _ => 1,
                };
                commands.entity(enemy_entity).despawn();
            }
        }

        // Player-Powerup collisions
        for (powerup_entity, powerup_transform) in &powerup_query {
            if player_transform.translation.distance(powerup_transform.translation) < 30.0 {
                player.powerup_active = true;
                player.powerup_timer.reset();
                commands.entity(powerup_entity).despawn();
            }
        }
    }
}

fn update_powerup(
    time: Res<Time>,
    mut query: Query<&mut Player>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        if player.powerup_active {
            player.powerup_timer.tick(time.delta());
            if player.powerup_timer.finished() {
                player.powerup_active = false;
            }
        }
    }
}

fn level_progression(
    mut stats: ResMut<GameStats>,
) {
    if stats.score >= stats.level * 20 {
        stats.level += 1;
    }
}

fn update_score_text(
    stats: Res<GameStats>,
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {}\nLevel: {}\nHealth: {}", stats.score, stats.level, player.health);
        }
    }
}

fn check_game_over(
    player_query: Query<&Player>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(player) = player_query.get_single() {
        if player.health <= 0 {
            next_state.set(AppState::Menu);
        }
    }
}

