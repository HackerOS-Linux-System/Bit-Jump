use bevy::prelude::*;
use noise::{Perlin, NoiseFn};
use rand::Rng;
use super::components::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::NewGame), generate_map)
            .add_systems(OnEnter(GameState::InGame), render_map);
    }
}

pub const MAP_WIDTH: usize = 1500;
pub const MAP_HEIGHT: usize = 1500;
pub const TILE_SIZE: f32 = 32.0;

fn generate_map(mut commands: Commands) {
    let perlin = Perlin::new(42);
    let mut tiles = vec![vec![TileType::Grass; MAP_HEIGHT]; MAP_WIDTH];
    let mut regions = Vec::new();
    let mut rng = rand::thread_rng();

    // Generate terrain
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let nx = x as f64 / MAP_WIDTH as f64 - 0.5;
            let ny = y as f64 / MAP_HEIGHT as f64 - 0.5;
            let elevation = perlin.get([nx * 5.0, ny * 5.0]);
            let moisture = perlin.get([nx * 10.0, ny * 10.0] + [1000.0, 1000.0]);
            let temperature = perlin.get([nx * 7.0, ny * 7.0] + [2000.0, 2000.0]);
            tiles[x][y] = match (elevation, moisture, temperature) {
                (e, m, _) if e < -0.3 => TileType::Water,
                (e, m, t) if e < 0.0 && m > 0.3 => TileType::Swamp,
                (e, m, t) if e < 0.0 && t < 0.0 => TileType::Desert,
                (e, m, _) if e < 0.0 => TileType::Grass,
                (e, m, t) if e < 0.1 && m > 0.4 => TileType::Clearing,
                (e, m, t) if e < 0.2 && m > 0.2 => TileType::Farm,
                (e, m, t) if e < 0.3 && m > 0.2 => TileType::Forest,
                (e, m, t) if e < 0.3 && t > 0.4 => TileType::Ruins,
                (e, m, t) if e < 0.5 && m > 0.1 && t < 0.3 => TileType::Village,
                (e, m, t) if e < 0.6 && t > 0.3 => TileType::Town,
                (e, m, t) if e < 0.7 => TileType::Mountain,
                _ => TileType::Castle,
            };
        }
    }

    // Generate regions
    for i in 0..200 {
        let x = rng.gen_range(20..MAP_WIDTH - 20);
        let y = rng.gen_range(20..MAP_HEIGHT - 20);
        if tiles[x][y] != TileType::Water {
            let tile_type = match i % 5 {
                0 => TileType::Dungeon,
                1 => TileType::Castle,
                2 => TileType::Town,
                3 => TileType::Village,
                _ => TileType::Clearing,
            };
            regions.push(Region {
                id: i,
                name: format!("{} {}", tile_type.to_string(), i),
                tile_type: tile_type.clone(),
                center: (x, y),
                radius: rng.gen_range(8..25),
            });
            for dx in -(regions[i].radius as i32)..=regions[i].radius as i32 {
                for dy in -(regions[i].radius as i32)..=regions[i].radius as i32 {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if nx < MAP_WIDTH && ny < MAP_HEIGHT && (dx * dx + dy * dy) <= (regions[i].radius as i32).pow(2) {
                        tiles[nx][ny] = tile_type.clone();
                    }
                }
            }
        }
    }

    commands.insert_resource(WorldMap { tiles, regions });
}

fn render_map(mut commands: Commands, world_map: Res<WorldMap>) {
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let region_id = world_map.regions.iter()
                .find(|r| {
                    let (cx, cy) = r.center;
                    let dx = x as i32 - cx as i32;
                    let dy = y as i32 - cy as i32;
                    (dx * dx + dy * dy) <= (r.radius as i32).pow(2)
                })
                .map(|r| r.id);
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz((x as f32) * TILE_SIZE, (y as f32) * TILE_SIZE, -1.0),
                sprite: Sprite {
                    color: match world_map.tiles[x][y] {
                        TileType::Grass => Color::rgb(0.2, 0.6, 0.2),
                        TileType::Forest => Color::rgb(0.1, 0.4, 0.1),
                        TileType::Mountain => Color::rgb(0.5, 0.5, 0.5),
                        TileType::Water => Color::rgb(0.0, 0.3, 0.6),
                        TileType::Castle => Color::rgb(0.7, 0.7, 0.7),
                        TileType::Dungeon => Color::rgb(0.3, 0.3, 0.3),
                        TileType::Village => Color::rgb(0.6, 0.5, 0.4),
                        TileType::Town => Color::rgb(0.8, 0.6, 0.5),
                        TileType::Clearing => Color::rgb(0.3, 0.7, 0.3),
                        TileType::Farm => Color::rgb(0.5, 0.4, 0.2),
                        TileType::Ruins => Color::rgb(0.4, 0.4, 0.4),
                        TileType::Desert => Color::rgb(0.9, 0.8, 0.4),
                        TileType::Swamp => Color::rgb(0.2, 0.3, 0.2),
                    },
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            }).insert(Tile {
                tile_type: world_map.tiles[x][y].clone(),
                visibility: 0.0,
                region_id,
            });
        }
    }
}
