#[cfg(feature = "debug")]
use colored::Colorize;
#[cfg(feature = "debug")]
use bevy::prelude::{FromReflect, Reflect};

use derive_more::IsVariant;

/// Enum describing a Minesweeper tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, IsVariant)]
#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
pub enum Tile {
    /// Empty tile
    #[default]
    Empty,
    /// Is a bomb
    Bomb,
    /// Is a bomb neighbor
    BombNeighbor(u8),
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}