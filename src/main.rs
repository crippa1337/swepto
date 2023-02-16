mod grid;
use grid::*;
use macroquad::prelude::*;

#[macroquad::main("Swepto")]
async fn main() {
    let mut grid = Grid::new(20);

    loop {
        grid.draw();
        next_frame().await
    }
}
