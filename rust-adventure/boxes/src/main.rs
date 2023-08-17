use crate::board::Board;
use bevy::prelude::*;
use board::Position;
use itertools::Itertools;
use rand::prelude::*;

mod board;
mod colors;

#[derive(Component)]
struct Points {
    value: u32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "boxes.rs".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (setup, spawn_board, apply_deferred, spawn_tiles).chain(),
        )
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
            .insert(Points { value: 2 })
            .insert(pos);
    }
}
