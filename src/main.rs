#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use brickadia::save::Brick;
use maze::maze::Maze;
use utils::brick::save_bricks;

mod maze {
    pub mod maze;
    mod cell;
}
mod utils {
    pub mod sfc32;
    pub mod math;
    pub mod polygon;
    pub mod draw;
    pub mod walk2d;
    pub mod brick;
    pub mod rectangle;
}
mod metadata {
    pub mod assets;
    pub mod headers;
}

fn main() {
    let mut maze = Maze { ring_gap: 40, rings: 40, initial_divisions: 4, solution: Vec::new() };
    let seed = [11, 13, 15, 2];
    let wall_width = 10;
    let wall_height = 30;
    let granularity = 1.0;
    let solve = true;
    let bricks: Vec<Brick> = maze.generate(seed, wall_width, wall_height, granularity, solve);
    save_bricks(bricks, "maze");
}
