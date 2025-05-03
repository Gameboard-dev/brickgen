use std::fs::File;

use brickadia::{save::{Brick, BrickColor, Color, Direction, Rotation, Size}, util::rotation, write::SaveWriter};
use brickadia::util::octree::Point;
use imageproc::rect::Rect;

use crate::metadata::{assets::BrickAssets, headers};

use super::{draw::Bitmap, math::bounds, walk2d::{compute_edges, rectangular_decomposition}};

pub const BLACK_BRICK: Color = Color { r: 0, b: 0, g: 0, a: 0 };

pub const MAX_SIZE: i32 = 500;

/// calculates an orientation around a vertex
pub fn orientation(
    vertex: (i32, i32), 
    min: Point,
    max: Point,
) -> (Direction, Rotation) {
    if vertex == (min.x, min.y) {
        (Direction::ZPositive, Rotation::Deg0)
    } else if vertex == (min.x, max.y) {
        (Direction::ZNegative, Rotation::Deg180)
    } else if vertex == (max.x, max.y) {
        (Direction::ZPositive, Rotation::Deg180)
    } else if vertex == (max.x, min.y) {
        (Direction::ZNegative, Rotation::Deg0)
    } else {
        (Direction::ZPositive, Rotation::Deg0)
    }
}

pub fn bricks_from_shapes(bricks: &mut Vec<Brick>, shapes: Vec<Vec<(i32, i32)>>, height: u32, pivot: Option<usize>, asset_name_index: u32) {

    for vertices in shapes {

        let (min, max) = bounds(&vertices);
        let length = max.y - min.y;
        let width = max.x - min.x;

        //let concave_vertex = vertices[0];
        let mut direction;
        let mut rotation;
        
        // centre of width in position units is width.
        let pos_x = min.x * 2 + width;
        let pos_y = min.y * 2 + length;   

        let mut brick = Brick {
            color: BrickColor::Unique(BLACK_BRICK),
            size: Size::Procedural(width as u32, length as u32, height as u32),
            asset_name_index,
            position: (pos_x, pos_y, height as i32),
            ..Default::default()
        };

        if let Some(pivot_index) = pivot {
            (direction, rotation) = orientation(vertices[pivot_index], min, max);
            brick.direction = direction;
            brick.rotation = rotation;
        }

        bricks.push(brick);
    }
}






pub fn save_bricks(bricks: Vec<Brick>, name: &str) {

    let (mut savedata, path) = headers::savedata(name.to_string());

    savedata.bricks = bricks;

    println!("Writing save to {} with {} bricks", path.to_string_lossy(), savedata.bricks.len());

    SaveWriter::new(File::create(path).unwrap(), savedata)
        .write()
        .unwrap();
}

