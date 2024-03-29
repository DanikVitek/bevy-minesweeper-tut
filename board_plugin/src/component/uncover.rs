use bevy::prelude::Component;

#[cfg(feature = "debug")]
use bevy::prelude::Reflect;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

/// Uncover component, indicates a covered tile that should be uncovered
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Default)]
#[cfg_attr(
    feature = "debug",
    derive(InspectorOptions, Reflect),
    reflect(InspectorOptions)
)]
pub struct Uncover;
