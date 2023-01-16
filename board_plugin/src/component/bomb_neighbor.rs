use bevy::prelude::Component;

#[cfg(feature = "debug")]
use bevy::prelude::{Reflect, ReflectResource, Resource};
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
use derive_more::From;

/// Bomb neighbor component
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Default, From)]
#[cfg_attr(
    feature = "debug",
    derive(Resource, InspectorOptions, Reflect),
    reflect(InspectorOptions, Resource)
)]
pub struct BombNeighbor {
    /// Number of neighbor bombs
    pub count: u8,
}
