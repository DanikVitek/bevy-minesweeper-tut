pub mod component;
mod event;
pub mod resource;
mod system;

use bevy::{log, math::Vec3Swizzles, prelude::*, sprite::Anchor, utils::HashMap};

use component::{Bomb, BombNeighbor, Coordinates, Uncover};
use event::TileTriggerEvent;
use resource::{Board, BoardOptions, BoardPosition, Tile, TileMap, TileSize};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_type::<Coordinates>()
            .register_type::<Bomb>()
            .register_type::<BombNeighbor>()
            .register_type::<Uncover>()
            .register_type::<TileMap>()
            .register_type::<Board>()
            .register_type::<BoardOptions>()
            .register_type::<TileSize>();

        app.add_startup_system(Self::create_board)
            .add_system(system::input::input_handling)
            .add_event::<TileTriggerEvent>()
            .add_system(system::uncover::trigger_event_handler)
            .add_system(system::uncover::uncover_tiles);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Res<Windows>,
        asset_server: Res<AssetServer>,
    ) {
        let board_options = match board_options {
            Some(o) => o.clone(),
            None => Default::default(),
        };
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");

        // Tilemap generation
        let mut tile_map = TileMap::empty(board_options.map_size.0, board_options.map_size.1);
        tile_map.set_bombs(board_options.bomb_count);
        let tile_map = tile_map;
        #[cfg(feature = "debug")]
        // Tilemap debugging
        log::info!("{}", tile_map.console_output());

        let window = windows.get_primary().expect("Failed to get primary window");

        // We define the size of our tiles in world space
        let tile_size = match board_options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        // We deduce the size of the complete board
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // We define the board anchor position (bottom left)
        let board_position = match board_options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        Self::spawn_board(
            commands,
            board_position,
            board_size,
            tile_map,
            tile_size,
            board_options,
            font,
            bomb_image,
        );
    }

    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        // We retrieve the text and the correct color
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );
        // We generate a text bundle
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0., 0., 2.),
            ..Default::default()
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn_board(
        mut commands: Commands,
        board_position: Vec3,
        board_size: Vec2,
        tile_map: TileMap,
        tile_size: f32,
        board_options: BoardOptions,
        font: Handle<Font>,
        bomb_image: Handle<Image>,
    ) {
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()) as usize);

        let mut safe_start = None;

        commands
            .spawn_empty()
            .insert(Name::new("Board"))
            .insert(TransformBundle::from_transform(
                Transform::from_translation(board_position),
            ))
            .insert(VisibilityBundle {
                visibility: Visibility::VISIBLE,
                ..Default::default()
            })
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    board_options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                    Color::DARK_GRAY,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            });

        if board_options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map,
            bounds: {
                let min = board_position.xy();
                Rect {
                    min,
                    max: min + board_size,
                }
            },
            tile_size,
            covered_tiles,
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        // Tiles
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut cmd = parent.spawn_empty();
                cmd.insert(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coordinates)
                // We add the cover sprites
                .with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 3.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);

                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb).with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 2.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(n) => {
                        cmd.insert(BombNeighbor { count: *n })
                            .with_children(|parent| {
                                parent.spawn(Self::bomb_count_text_bundle(
                                    *n,
                                    font.clone(),
                                    size - padding,
                                ));
                            });
                    }
                    Tile::Empty => (),
                };
            }
        }
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn adaptative_tile_size(
        window: &Window,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
