
use rayon::iter::ParallelIterator;
use brickadia::save::Brick;
use image::RgbImage;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};


use crate::metadata::assets::BrickAssets;

use super::{
    brick::{bricks_from_shapes, MAX_SIZE}, 
    pixels::{in_bounds, Bitmap, TupleUtils, BLACK, BLUE, RED}, 
    math::TupleMath, rectangle::RectUtils
};

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}



fn concave_vertex(image: &RgbImage, sx: i32, sy: i32) -> Option<(i32, i32)> {
    // Check if the pixel (sx, sy) is black
    if image.get((sx, sy)) != BLACK {
        return None;
    }

    // Iterate over horizontal and vertical directions
    for (horizontal, vertical) in [
        (Direction::Left, Direction::Up),
        (Direction::Left, Direction::Down),
        (Direction::Right, Direction::Up),
        (Direction::Right, Direction::Down),
    ] {
            // Tuple arithmetics
            let neighbor1 = (sx, sy).add(horizontal.value());
            let neighbor2 = (sx, sy).add(vertical.value());

            // Ensure both neighbors are within bounds and black
            if in_bounds(image, neighbor1)
                && in_bounds(image, neighbor2)
                && image.get(neighbor1) == BLACK
                && image.get(neighbor2) == BLACK
            {
                // Calculate the concave vertex (diagonal)
                let concave = neighbor1.add(vertical.value());

                // Return the concave vertex if it's within bounds and not black
                if in_bounds(image, concave) && image.get(concave) != BLACK {
                    return Some(concave);
                }
            }
        }

    None
}

/// steps along the x_axis until it encounters a white pixel on the same line
/// or a black pixel on the parallel line `(sy + dy)`
fn walk_x(image: &RgbImage, (sx, sy): (i32, i32), (dx, dy): (i32, i32)) -> i32 {
        let mut new_x = sx;
        while in_bounds(&image, (new_x + dx, sy)) && image.get((new_x + dx, sy)) == BLACK {
            if image.get((new_x + dx, sy + dy)) == BLACK {
                let steps = new_x - sx;
                // Indicates we are in a horizontal dip |__|
                // Move `new_x` back half the steps (`steps / 2`)
                // If x steps is odd, shorten the `side` where `dy == 1` to avoid overlaps.
                new_x -= steps / 2 + (steps % 2 != 0 && dx == 1) as i32 * dx;
                break;
            }
            new_x += dx;
        }
        new_x
}

/// steps along the y_axis until it encounters a white pixel on the same line
/// or a black pixel on the parallel line `(sy + dy)`
fn walk_y(image: &RgbImage, (sx, sy): (i32, i32), (dx, dy): (i32, i32)) -> i32 {
    let mut new_y = sy;
    while in_bounds(&image, (sx, new_y + dy)) && image.get((sx, new_y + dy)) == BLACK {
        if image.get((sx + dx, new_y + dy)) == BLACK {
            let steps = new_y - sy;
            // Indicates we are in a vertical dip
            // Move `new_y` back half the steps (`steps / 2`)
            // If y steps is odd, shorten the `side` where `dy == 1` to avoid overlaps.
            new_y -= steps / 2 + (steps % 2 != 0 && dy == 1) as i32 * dy;
            break;
        }
        new_y += dy;
    }
    new_y
}
pub fn compute_edges(bitmap: &mut Bitmap) -> (Bitmap, Vec<Vec<(i32, i32)>>) {

    let image = bitmap.image.clone();

    // A separate `bitmap` is used to record the edges.
    let mut bitmap_less_edges = bitmap.clone();
    let bitmap_less_edges_mutex = std::sync::Mutex::new(&mut bitmap_less_edges);

    let triangles: Vec<Vec<(i32, i32)>> = bitmap.image
        .par_enumerate_pixels_mut()
        .filter_map(|(x, y, _pixel)| {

            let sx = x as i32;
            let sy = y as i32;

            if let Some(concave) = concave_vertex(&image, sx, sy) {
                // Find the direction signs (-/+) relative to the starting position
                let dx = (concave.0 - sx).signum();
                let dy = (concave.1 - sy).signum();

                // Skip triangles which would have no edge
                if dx == 0 || dy == 0 {
                    return None;
                }

                let new_x = walk_x(&image, (sx, sy), (dx, dy));
                let new_y = walk_y(&image, (sx, sy), (dx, dy));

                // Skip corners violating max thresholds:
                let x_steps = (new_x - sx).abs();
                let y_steps = (new_y - sy).abs();

                // Cap length on **one** side
                let max_length = 15;
                if y_steps > max_length || x_steps > max_length {
                    return None;
                }

                // Cap width on **both** sides
                let max_width = 5;
                if y_steps > max_width && x_steps > max_width {
                    return None;
                }

                // The edges drawn as lines DO NOT MATCH the outline on a pixel grid
                // Drawn with the same points (p1, p2, p3)
                let mut p1 = (sx, sy);
                let mut p2 = (new_x, sy);
                let mut p3 = (sx, new_y);

                // Adjustments are needed to 'push' the outline into the edges:
                if dy == 1 || (dy == -1 && dx == 1) {
                    if dy == 1 {
                        p1.1 += dy;
                        p2.1 += dy;
                        p3.1 += dy;
                    }
                    if dx == 1 {
                        p1.0 += dx;
                        p2.0 += dx;
                        p3.0 += dx;
                    }
                }

                // Other threads queue for the lock to be reacquired once the rectangle is drawn
                bitmap_less_edges_mutex.lock().unwrap().rectangle(&[p1, p2, p3], RED);

                // Return the triangle
                Some(vec![p1, p2, p3])
            } else {
                None
            }
        })
        .collect();

    //let _ = bitmap_less_edges.save("edges");

    (bitmap_less_edges, triangles)
}


pub fn rectangular_decomposition(bitmap_less_edges: &mut Bitmap) -> Vec<Vec<(i32, i32)>> {

    let (image_width, image_height) = bitmap_less_edges.image.dimensions();

    // Rectangles defined by their corners.
    let mut rectangles: Vec<Vec<(i32, i32)>> = Vec::new();

    // Parallel computation appears to be unnecessary here?
    for y in 0..image_height as i32 {
        // top left starting position
        for x in 0..image_width as i32 {

            // Skip over irrelevant pixels
            if bitmap_less_edges.image.get((x, y)) != BLACK {
                continue; 
            }

            // The dimensions of the rectangle
            let (mut width, mut height) = (0, 0);

            // Expand horizontally
            while (x + width) < image_width as i32 {
                if bitmap_less_edges.image.get((x + width, y)) != BLACK {
                    break;
                }
                width += 1;
            }

            // Expand vertically
            'vertical: while (y + height) < image_height as i32 {
                for dx in x..(x + width) {
                    if bitmap_less_edges.image.get((dx, y + height)) != BLACK {
                        break 'vertical
                    }
                }
                height += 1;
            }

            let rectangle = Rect::at(x, y).of_size(width as u32, height as u32);

            // Mark the filled space with a single rectangle which contains subrectangles.
            draw_filled_rect_mut(&mut bitmap_less_edges.image, rectangle, BLUE);
            
            // Perform recursive subdivision to conform to Brickadia resize constraints.
            if width >= MAX_SIZE || height >= MAX_SIZE {
                let splits: Vec<Rect> = rectangle.recursively_subdivide(MAX_SIZE as u32);
                for r in splits {
                    rectangles.push(r.corners().to_vec());
                }
            } else {
                rectangles.push(rectangle.corners().to_vec());
            }
            
            
        }
    }

    //let _ = bitmap_less_edges.save("rectangles");

    rectangles

}

pub fn brick_pixels(mut image: &mut Bitmap, height: u32) -> Vec<Brick> {

    let (mut bitmap_less_edges, triangles) = compute_edges(&mut image);

    let rectangles: Vec<Vec<(i32, i32)>> = rectangular_decomposition(&mut bitmap_less_edges);

    let mut bricks: Vec<Brick> = Vec::new();

    // Modify `bricks` in place to add microbricks and microwedges
    bricks_from_shapes(&mut bricks, rectangles, height, None, BrickAssets::MicroBrick.index() as u32);

    let pivot_index = 0; // The right angle vertex
    bricks_from_shapes(&mut bricks, triangles, height, Some(pivot_index), BrickAssets::MicroWedge.index() as u32);

    bricks

}


#[cfg(test)]
mod tests {

    use image::{DynamicImage, RgbImage};

    use crate::utils::{brick::save_bricks, pixels::Bitmap};

    use super::brick_pixels;

    #[test]
    fn compute_image() {

        let img: DynamicImage = image::open("maze.png").expect("There is no `maze.png` in `/src`");
        let rgb_image: RgbImage = img.to_rgb8();
        let mut bitmap = Bitmap::from_image(rgb_image);
        let brick_height = 100;
        let bricks = brick_pixels(&mut bitmap, brick_height);
        save_bricks(bricks, "maze");

    }
}




