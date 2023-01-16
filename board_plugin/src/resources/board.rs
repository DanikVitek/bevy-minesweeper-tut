use crate::{Coordinates, TileMap};
use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Rect,
    pub tile_size: f32,
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
}
