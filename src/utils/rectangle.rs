use imageproc::rect::Rect;

pub trait RectUtils {
    fn halve_horizontally(&self) -> [Rect; 2];
    fn halve_vertically(&self) -> [Rect; 2];
    fn corners(&self) -> [(i32, i32); 4];
    fn recursively_subdivide(self, max_size: u32) -> Vec<Rect>;
}

impl RectUtils for Rect {
    fn halve_horizontally(&self) -> [Rect; 2] {
        let half_width = self.width() / 2;
        [
            Rect::at(
                self.left(), self.top())
                .of_size(half_width, self.height()),
            
            Rect::at(
                self.left() + half_width as i32, self.top())
                .of_size(self.width() - half_width, self.height()),
        ]
    }

    fn halve_vertically(&self) -> [Rect; 2] {
        let half_height = self.height() / 2;
        [
            Rect::at(
                self.left(), self.top())
                .of_size(self.width(), half_height),

            Rect::at(
                self.left(), self.top() + half_height as i32)
                .of_size(self.width(), self.height() - half_height),
        ]
    }

    fn corners(&self) -> [(i32, i32); 4] {
        let right = self.left() + (self.width() as i32);
        let bottom = self.top() + (self.height() as i32);
        [
            (self.left(), self.top()),                         
            (right, self.top()),     
            (self.left(), bottom),     
            (right, bottom),
        ]
    }
    fn recursively_subdivide(self, max_size: u32) -> Vec<Rect> {
        // If the rectangle fits within the specified dimensions, return the rectangle.
        if self.width() <= max_size && self.height() <= max_size {
            return vec![self];
        }
    
        // Determine how to divide the rectangle (horizontally or vertically).
        let rectangles = if self.width() > max_size {
            self.halve_horizontally()
        } else {
            self.halve_vertically()
        };
    
        // Return the recursively subdivided remainders.
        rectangles
            .into_iter()
            .flat_map(|r| r.recursively_subdivide(max_size))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, ImageResult, RgbImage};
    use imageproc::drawing::draw_hollow_rect_mut;

    use crate::utils::pixels::{BLACK, WHITE};

    use super::*;

    #[test]
    fn run_subdivision() -> ImageResult<()> {

        let mut image: RgbImage = ImageBuffer::from_pixel(110, 110, WHITE);

        let rectangle = Rect::at(0, 0).of_size(100, 100);
        let max_size = 10;

        let subdivisions = rectangle.recursively_subdivide(max_size);

        for rectangle in subdivisions {
            draw_hollow_rect_mut(&mut image, rectangle, BLACK);
        }

        image.save("subdivision.png")

    }
}