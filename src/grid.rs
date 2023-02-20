use macroquad::prelude::*;
const TILE_SIZE: f32 = 30.0;

#[derive(Clone, Copy)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub bomb: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            x: 0.0,
            y: 0.0,
            bomb: false,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, TILE_SIZE, TILE_SIZE, WHITE);
        draw_rectangle_lines(self.x, self.y, TILE_SIZE, TILE_SIZE, 1.0, BLACK)
    }
}

#[derive(Clone)]
pub struct Grid {
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let mut tiles = vec![Tile::new(); size * size];

        // Center the grid
        let width_offset = (screen_width() / 2.0) - (size as f32 * TILE_SIZE) / 2.0;
        let height_offset = (screen_height() / 2.0) - (size as f32 * TILE_SIZE) / 2.0;

        for (i, tile) in tiles.iter_mut().enumerate() {
            // Row can be indexed by modulo
            // Column can be indexed by division
            tile.x = (i % size) as f32 * TILE_SIZE + width_offset;
            tile.y = (i / size) as f32 * TILE_SIZE + height_offset;
        }

        Grid { tiles }
    }

    pub fn draw(&self) {
        for tile in &self.tiles {
            tile.draw();
        }
    }
}
