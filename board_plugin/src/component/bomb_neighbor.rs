use bevy::prelude::Component;
use derive_more::From;

#[cfg(feature = "debug")]
use bevy::prelude::Reflect;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

/// Bomb neighbor component
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Default, From)]
#[cfg_attr(
    feature = "debug",
    derive(InspectorOptions, Reflect),
    reflect(InspectorOptions)
)]
pub struct BombNeighbor {
    /// Number of neighbor bombs
    pub count: u8,
}
