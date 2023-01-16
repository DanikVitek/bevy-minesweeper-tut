use crate::Board;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    prelude::*,
};

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
) {
    let window = windows.get_primary().expect("Failed to get primary window");

    for event in button_evr.iter() {
        let ButtonState::Released = event.state else {
            continue;
        };

        if !matches!(event.button, MouseButton::Left | MouseButton::Right) {
            continue;
        }

        let Some(pos) = window.cursor_position() else {
            continue;
        };

        log::trace!("Mouse button pressed: {:?} at {pos}", event.button);
        let Some(coordinates) = board.mouse_position(window, pos) else {
            continue;
        };

        match event.button {
            MouseButton::Left => {
                log::info!("Trying to uncover tile on {coordinates}");
                // TODO: generate an event
            }
            MouseButton::Right => {
                log::info!("Trying to mark tile on {coordinates}");
                // TODO: generate an event
            }
            _ => (),
        }
    }
}
