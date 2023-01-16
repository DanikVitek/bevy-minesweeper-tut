use crate::{Coordinates, TileMap};
use bevy::{prelude::*, utils::HashMap};

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Resource)]
#[cfg_attr(
    feature = "debug",
    derive(InspectorOptions, Reflect, Default),
    reflect(InspectorOptions, Resource)
)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Rect,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
}

impl Board {
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        self.bounds.contains(position).then(|| {
            // World space to board space
            let coordinates = position - self.bounds.min;
            Coordinates {
                x: (coordinates.x / self.tile_size) as u16,
                y: (coordinates.y / self.tile_size) as u16,
            }
        })
    }

    /// Retrieves a covered tile entity
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coords)
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coords)
    }

    /// We retrieve the adjacent covered tile entities of `coord`
    pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}
