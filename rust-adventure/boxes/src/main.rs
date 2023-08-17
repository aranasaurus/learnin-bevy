use bevy::prelude::*;
use itertools::Itertools;

mod board;
mod colors;

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
        .add_systems(Startup, (setup, spawn_board))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands) {
    let board = board::Board::new(4);

    commands
        .spawn(board.make_board_sprite())
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(board.make_tile_sprite(tile));
            }
        })
        .insert(board);
}
