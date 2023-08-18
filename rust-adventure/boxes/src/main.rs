use crate::board::Board;
use bevy::{prelude::*, window::WindowResolution};
use board::Position;
use itertools::Itertools;
use rand::prelude::*;

mod board;
mod colors;

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct TileText;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "boxes.rs".to_string(),
                // Comment these two out when not working on the laptop...
                resolution: WindowResolution::new(574.0, 326.0),
                position: WindowPosition::new(IVec2::new(1732, 1162)),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (setup, spawn_board, apply_deferred, spawn_tiles).chain(),
        )
        .add_systems(Update, render_tile_points)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(board.make_board_sprite())
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                let pos = Position {
                    x: tile.0,
                    y: tile.1,
                };
                builder.spawn(board.make_tile_sprite(&pos, colors::TILE_PLACEHOLDER));
            }
        })
        .insert(board);
}

fn spawn_tiles(mut commands: Commands, query_board: Query<&Board>) {
    let board = query_board.single();

    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        commands
            .spawn(board.make_tile_sprite(&pos, colors::TILE))
            .with_children(|builder| {
                builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            "x",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    })
                    .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(pos);
    }
}

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts.get_mut(*entity).expect("expected Text to exist");
            let mut text_section = text
                .sections
                .first_mut()
                .expect("expect first section to be accessible as mutable");
            text_section.value = points.value.to_string()
        }
    }
}
