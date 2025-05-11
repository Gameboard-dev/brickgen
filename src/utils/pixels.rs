use std::any::type_name;

use image::{ImageBuffer, Rgb, RgbImage};
use image::imageops::FilterType; use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

use imageproc::{drawing::{draw_filled_circle_mut, draw_line_segment_mut, draw_polygon_mut}, point::Point};
use num_traits::ToPrimitive;

use super::math::bounds;

//use super::polygon::Polygon;

pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const BLUE: Rgb<u8> = Rgb([0, 0, 255]);


pub fn in_bounds(image: &RgbImage, (x, y): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32
}

#[derive(Clone)]
pub struct Bitmap {
    pub image: RgbImage,
}

impl Bitmap {
    pub fn new(image_width: usize) -> Self {
        Self {
            image: ImageBuffer::from_pixel(image_width as u32, image_width as u32, WHITE),
        }
    }
    pub fn from_image(image: RgbImage) -> Self {
        Self {
            image,
        }
    }
    

    /// Draws an arc using cubic Bézier approximation.
    pub fn arc(
        &mut self,
        centre: (f64, f64),
        arc_radius: f64,
        angle_begin: f64,
        angle_end: f64,
        rgb: Rgb<u8>,
        stroke_width: u32
    ) {
        // Determine the number of sample points based on the arc length.
        let arc_length = (angle_end - angle_begin).abs() * arc_radius;
        let num_samples = arc_length.ceil() as i32;
        let (cx, cy) = centre;

        let circle_width = stroke_width as i32 / 2;
        
        for i in 0..=num_samples {
            let theta = angle_begin + (angle_end - angle_begin) * (i as f64) / (num_samples as f64);
            let x = (cx + arc_radius * theta.cos()).round() as i32;
            let y = (cy + arc_radius * theta.sin()).round() as i32;
            draw_filled_circle_mut(&mut self.image, (x, y), circle_width, rgb);
        }
    }


    pub fn line<T: ToPrimitive>(
        &mut self,
        (x0, y0): (T, T),
        (x1, y1): (T, T),
        rgb: Rgb<u8>,
        stroke_width: u32
        ) -> &mut Self {
        let err_msg = &format!("Conversion failed: `{}`", type_name::<T>());
        let x0 = x0.to_f32().expect(err_msg);
        let y0 = y0.to_f32().expect(err_msg);
        let x1 = x1.to_f32().expect(err_msg);
        let y1 = y1.to_f32().expect(err_msg);
    
        if stroke_width == 1 {
            draw_line_segment_mut(&mut self.image, (x0, y0), (x1, y1), rgb);

        } else { // Draw diagonal lines of variable stroke_width as filled polygons
            let (dx, dy) = (x1 - x0, y1 - y0);
            let line_length = (dx * dx + dy * dy).sqrt();
            if line_length != 0.0 {

                // Calculate the normalized vector of the line [-1, 1]
                let perp_dx = -dy / line_length;
                let perp_dy = dx / line_length;
    
                let half_stroke = stroke_width as f32 / 2.0;
    
                // Calculate the four corners of the rotated rectangle
                let corner = |x: f32, y: f32, sign: f32| {
                    Point::new((x + sign * perp_dx * half_stroke).round() as i32,
                               (y + sign * perp_dy * half_stroke).round() as i32)
                };
                
                let p1 = corner(x0, y0, 1.0);
                let p2 = corner(x0, y0, -1.0);
                let p3 = corner(x1, y1, -1.0);
                let p4 = corner(x1, y1, 1.0);
                
                // Draw the thick line as a filled convex polygon.
                draw_polygon_mut(&mut self.image, &[p1, p2, p3, p4], rgb);
            }
        }
        self
    }

    pub fn rectangle(
        &mut self,
        points: &[(i32, i32)],
        rgb: Rgb<u8>,
    ) {
        let (min, max) = bounds(points);

        let length = max.x.saturating_sub(min.x) as u32;
        let width = max.y.saturating_sub(min.y) as u32;
    
        if length == 0 || width == 0 {
            return;
        }
        
        let rectangle = Rect::at(min.x, min.y).of_size(
            length, // Width
            width, // Height
        );
    
        draw_hollow_rect_mut(&mut self.image, rectangle, rgb);
    }

    // Downscales the image by applying a kernel convolution to the image pixels.
    pub fn downscale(&mut self, factor: u32) {

        println!("Downscaling by {}", factor);
        
        if factor <= 1 {
            return;
        }

        // The dimensions of the current image.
        let (width, height) = self.image.dimensions();

        // Downscale using nearest neighbor uniform kernel convolution.
        let new_width = width / factor;
        let new_height = height / factor;
        let downscaled = image::imageops::resize(&self.image, new_width, new_height, FilterType::Nearest);

        // Upscale back to the original size.
        let pixelated = image::imageops::resize(&downscaled, width, height, FilterType::Nearest);

        // Replace image with the pixelated version.
        self.image = pixelated;
    }

    pub fn save(&self, name: &str) -> &Self {
        println!("Saving {}.png", name);
        self.image
            .save(&(name.to_string() + ".png"))
            .expect("Failed to save image");
        self
    }
}

pub trait TupleUtils {
    fn get(&self, coord: (i32, i32)) -> Rgb<u8>;
    fn put(&mut self, coord: (i32, i32), color: Rgb<u8>);
}

impl TupleUtils for RgbImage {
    fn get(&self, (x, y): (i32, i32)) -> Rgb<u8> {
        *self.get_pixel(x as u32, y as u32)
    }
    fn put(&mut self, (x, y): (i32, i32), color: Rgb<u8>) {
        self.put_pixel(x as u32, y as u32, color)
    }
}