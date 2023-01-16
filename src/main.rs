use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};
use derive_more::IsVariant;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::{resource::BoardOptions, BoardPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash, IsVariant)]
pub enum AppState {
    InGame,
    Out,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Mine Sweeper".to_owned(),
                    width: 800.,
                    height: 700.,
                    ..Default::default()
                },
                ..default()
            })
            .set(LogPlugin {
                level: log::Level::DEBUG,
                ..Default::default()
            }),
    );

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app.add_state(AppState::InGame)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.,
            ..default()
        })
        .add_startup_system(camera_setup)
        .add_system(state_handling)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn state_handling(mut state: ResMut<State<AppState>>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::C) {
        log::debug!("Clearing detected! Current state: {state:?}");
        if state.current().is_in_game() {
            log::info!("Clearing game");
            state.set(AppState::Out).unwrap();
        }
    }

    if key.just_pressed(KeyCode::G) {
        log::debug!("Loading detected! Current state: {state:?}");
        if state.current().is_out() {
            log::info!("Loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
}
