use std::ops::{Deref, DerefMut};

use super::tile::Tile;

use crate::components::coordinates::*;
use crate::resources::*;

const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    /*pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }*/

    pub fn set_bombs(&mut self, bomb_count: u16, rng: &mut Random) {
        self.bomb_count = bomb_count;

        let mut remaining_bombs = bomb_count;
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );

            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
                if self.is_bomb_at(coords) {
                    continue;
                }

                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }

                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }

    fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        coordinates.x < self.width
            && coordinates.y < self.height
            && self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        self.safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count() as u8
    }

    pub fn get_debug_string(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );

        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer.push_str(&line);
        buffer.push('\n');

        for line in self.iter().rev() {
            buffer.push('|');
            for tile in line.iter() {
                buffer.push_str(&tile.get_debug_string());
            }
            buffer.push_str("|\n");
        }
        buffer.push_str(&line);

        buffer
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
