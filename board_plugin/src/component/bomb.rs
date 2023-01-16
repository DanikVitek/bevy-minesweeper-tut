use bevy::prelude::Component;

#[cfg(feature = "debug")]
use bevy::prelude::Reflect;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

/// Bomb component
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Default)]
#[cfg_attr(
    feature = "debug",
    derive(InspectorOptions, Reflect),
    reflect(InspectorOptions)
)]
pub struct Bomb;
