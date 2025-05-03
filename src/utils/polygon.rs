
#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<(i32, i32)>,
    pub min: (i32, i32),
    pub max: (i32, i32),
}

impl Polygon {
    /// Constructs a new polygon with minimum and maximum bounds.
    pub fn new(points: Vec<(i32, i32)>) -> Self {
        assert!(
            points.len() >= 3,
            "A polygon must have 3 points."
        );

        let (min, max) = points.iter().fold(
            ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
            |(min, max), &(x, y)| {
                (
                    (min.0.min(x), min.1.min(y)),
                    (max.0.max(x), max.1.max(y)),
                )
            },
        );

        Self { points, min, max }
    }

    /// Uses a raytracing algorithm to decide whether a point is within the polygon
    pub fn contains(&self, point: (i32, i32)) -> bool {
        // Check if the point is outside the bounding box
        if point.0 < self.min.0
            || point.0 > self.max.0
            || point.1 < self.min.1
            || point.1 > self.max.1
        {
            return false;
        }

        let mut crossings = 0;
        let n = self.points.len();

        for i in 0..n {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % n];

            // Check if the edge crosses the horizontal line
            if (p1.1 > point.1) != (p2.1 > point.1) {
                let intersection_x =
                    (p2.0 - p1.0) * (point.1 - p1.1) / (p2.1 - p1.1) + p1.0;
                if point.0 < intersection_x {
                    crossings += 1;
                }
            }
        }

        // A point is inside the polygon if the number of crossings is odd
        crossings % 2 == 1
    }
}