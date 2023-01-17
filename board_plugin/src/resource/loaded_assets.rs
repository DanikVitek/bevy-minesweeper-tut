use bevy::prelude::*;

#[derive(Debug, Clone, Resource, PartialEq, Eq, Hash)]
pub struct LoadedAssets {
    pub bomb_image: Handle<Image>,
    pub font: Handle<Font>,
}
