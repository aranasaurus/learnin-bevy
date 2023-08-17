use bevy::prelude::*;

pub const TILE_SIZE: f32 = 40.0;
pub const TILE_SPACER: f32 = 10.0;

#[derive(Component)]
pub struct Board {
    pub size: u8,
    pub physical_size: f32,
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

    pub fn sprite_size(&self) -> Vec2 {
        Vec2 {
            x: self.physical_size,
            y: self.physical_size,
        }
    }
}
