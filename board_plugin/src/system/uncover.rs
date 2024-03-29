use crate::{
    component::{Bomb, BombNeighbor, Coordinates, Uncover},
    event::TileTriggerEvent,
    resource::Board,
};
use bevy::{log, prelude::*};

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    tile_trigger_evr
        .iter()
        .filter_map(|TileTriggerEvent(coords)| board.tile_to_uncover(coords))
        .for_each(|entity| {
            commands.entity(*entity).insert(Uncover);
        })
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    // We iterate through tile covers to uncover
    for (entity, parent) in children.iter() {
        // we destroy the tile cover entity
        commands.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{e}");
                continue;
            }
        };

        // We remove the entity from the board covered tile map
        match board.try_uncover_tile(coords) {
            Some(e) => log::debug!("Uncovered tile {coords} (entity: {e:?})"),
            None => log::debug!("Tried to uncover an already uncovered tile"),
        }
        if bomb.is_some() {
            log::info!("Boom!");
            // TODO: Add explosion event
        }
        // If the tile is empty..
        else if bomb_counter.is_none() {
            // .. We propagate the uncovering by adding the `Uncover` component to adjacent tiles
            // which will then be removed next frame
            board
                .adjacent_covered_tiles(*coords)
                .into_iter()
                .for_each(|entity| {
                    commands.entity(entity).insert(Uncover);
                });
        }
    }
}
