use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Mine Sweeper".to_owned(),
            width: 800.,
            height: 700.,
            ..Default::default()
        },
        ..default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app.add_plugin(BoardPlugin)
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.,
            ..default()
        })
        .add_startup_system(camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
