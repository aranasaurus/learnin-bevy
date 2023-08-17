use crate::colors;
use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Component)]
pub struct Board {
    pub size: u8,
    pub physical_size: f32,
}

#[derive(Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Board {
    pub fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;

        Board {
            size,
            physical_size,
        }
    }

    pub fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }

    pub fn make_board_sprite(&self) -> SpriteBundle {
        let sprite_size = Vec2 {
            x: self.physical_size,
            y: self.physical_size,
        };

        SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(sprite_size),
                ..default()
            },
            ..default()
        }
    }

    pub fn make_tile_sprite(&self, tile: &Position, color: Color) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                self.cell_position_to_physical(tile.x),
                self.cell_position_to_physical(tile.y),
                1.0,
            ),
            ..default()
        }
    }
}
