

#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod maze {
    pub mod maze;
    mod cell;
}
mod utils {
    pub mod sfc32;
    pub mod math;
    pub mod pixels;
    pub mod voxels;
    pub mod walk;
    pub mod brick;
    pub mod rectangle;
}
mod metadata {
    pub mod assets;
    pub mod headers;
}
mod bevy {
    pub mod lights;
    pub mod plugins;
    pub mod update;
    pub mod registry;
    pub mod camera;
    pub mod cursor;
    pub mod app;
}

use brickadia::save::Brick;
use maze::maze::Maze;
use utils::brick::save_bricks;

const VOXEL_TESTING: bool = false;

fn main() {

    if VOXEL_TESTING {
        bevy::app::application();
    }
    else {
        let mut maze = Maze { ring_gap: 10, rings: 60, initial_divisions: 4, solution: Vec::new() };
        let seed = [11, 13, 15, 2];
        let wall_width = 5;
        let wall_height = 1;
        let granularity = 1.0;
        let solve = true;
        let bricks: Vec<Brick> = maze.generate(seed, wall_width, wall_height, granularity, solve);
        save_bricks(bricks, "maze");
    }
}
