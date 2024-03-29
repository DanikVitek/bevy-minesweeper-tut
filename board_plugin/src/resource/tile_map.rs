use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};
use rand::Rng;

use crate::{component::Coordinates, resource::Tile};

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "debug")]
use bevy::prelude::{Reflect, ReflectResource};

/// Base tile map
#[derive(Debug, Clone, Deref, DerefMut, Resource)]
#[cfg_attr(
    feature = "debug",
    derive(Reflect, InspectorOptions, Default),
    reflect(Resource, InspectorOptions)
)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    #[deref]
    #[deref_mut]
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generates an empty map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .map(|_| (0..width).map(|_| Tile::default()).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{buffer}{line}\n");
        for line in self.iter().rev() {
            buffer = format!("{buffer}|");
            for tile in line.iter() {
                buffer = format!("{buffer}{}", tile.console_output());
            }
            buffer = format!("{buffer}|\n");
        }
        format!("{buffer}{line}")
    }

    /// Getter for `width`
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Getter for `height`
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Getter for `bomb_count`
    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    /// Delta coordinates for all 8 square neighbors
    const SQUARE_COORDINATES: [(i8, i8); 8] = [
        // Bottom left
        (-1, -1),
        // Bottom
        (0, -1),
        // Bottom right
        (1, -1),
        // Left
        (-1, 0),
        // Right
        (1, 0),
        // Top Left
        (-1, 1),
        // Top
        (0, 1),
        // Top right
        (1, 1),
    ];

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        impl std::ops::Add<(i8, i8)> for Coordinates {
            type Output = Self;

            fn add(self, (x, y): (i8, i8)) -> Self::Output {
                let x = ((self.x as i16) + x as i16) as u16;
                let y = ((self.y as i16) + y as i16) as u16;
                Self { x, y }
            }
        }

        Self::SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_bomb_at(&self, Coordinates { x, y }: Coordinates) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        };
        self.map[y as usize][x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }
        let res = self
            .safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();
        res as u8
    }

    /// Places bombs and bomb neighbor tiles
    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = rand::thread_rng();
        // Place bombs
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }
        // Place bomb neighbors
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
}
