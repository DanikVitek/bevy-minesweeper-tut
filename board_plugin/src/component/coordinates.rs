use bevy::prelude::Component;
use derive_more::{Add, Display, From, Sub};

#[cfg(feature = "debug")]
use bevy::prelude::{Reflect, FromReflect};
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

#[derive(
    Component,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Add,
    Sub,
    Display,
    From,
    Default,
)]
#[cfg_attr(
    feature = "debug",
    derive(InspectorOptions, Reflect, FromReflect),
    reflect(InspectorOptions)
)]
#[display(fmt = "({}, {})", x, y)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}
