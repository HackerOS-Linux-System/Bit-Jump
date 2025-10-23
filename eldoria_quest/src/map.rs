use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TileType {
    Meadow,
    Forest,
    Castle,
    Village,
    Mountain, // Expanded
    River,   // Expanded
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut rng = rand::thread_rng();
        let mut tiles = vec![vec![TileType::Meadow; width]; height];

        // Expanded generation: more varied
        for y in 0..height {
            for x in 0..width {
                let rand_val = rng.gen_range(0..100);
                tiles[y][x] = if rand_val < 30 {
                    TileType::Meadow
                } else if rand_val < 50 {
                    TileType::Forest
                } else if rand_val < 60 {
                    TileType::Village
                } else if rand_val < 70 {
                    TileType::Castle
                } else if rand_val < 85 {
                    TileType::Mountain
                } else {
                    TileType::River
                };
            }
        }

        Map { tiles, width, height }
    }
}
