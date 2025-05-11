use brickadia::save::Brick;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::{pixels::BLACK, walk::brick_pixels};

use crate::utils::{pixels::{Bitmap, RED}, math::TupleMath, sfc32::SFC32};
use super::cell::Cell;


const MAX_RING: usize = 9;

pub struct Maze {
    pub ring_gap: u32,
    pub rings: usize,
    pub initial_divisions: usize,
    pub solution: Vec<(usize, usize)>,
}
impl Maze {

    pub fn divisions_in_ring(&self, ring: usize) -> usize {
        let ring = if  ring > MAX_RING {MAX_RING + ring % 2} else {ring};
        self.initial_divisions << (ring >> 1)
    }

    fn open_wall_between(
        &self, 
        cells: &mut [Vec<Cell>],
        (ring_a, div_a): (usize, usize),
        (ring_b, div_b): (usize, usize),
    ) {
        if ring_a == ring_b {
            // Same ring: Determine the preceding division and open its right wall
            let next = (div_a + 1) % self.divisions_in_ring(ring_a);
            let target = if next == div_b { div_a } else { div_b };
            cells[ring_a][target].right_wall = false;
    
        } else {
            // Different rings: Open the inner wall of the outermost ring cell
            let (outer_ring, outer_division) = if ring_a > ring_b {
                (ring_a, div_a)
            } else {
                (ring_b, div_b)
            };
    
            // Open the inner wall of the corresponding cell in the outer ring
            cells[outer_ring][outer_division].inner_wall = false;
        }
    }

    /// Computes and returns `Vec<(ring, division)>` of all unvisited neighbours
    fn unvisited_neighbours(
        &self, 
        cells: &[Vec<Cell>],
        ring: usize, 
        division: usize, 
    ) -> Vec<(usize, usize)> {
        let mut unvisited = Vec::with_capacity(4);
        let ring_divisions = self.divisions_in_ring(ring);
    
        // Check the next clockwise and anticlockwise neighbors in the same ring
        let clockwise = (division + 1) % ring_divisions;
        if !cells[ring][clockwise].visited {
            unvisited.push((ring, clockwise));
        }
    
        let anticlockwise = (division + ring_divisions - 1) % ring_divisions;
        if !cells[ring][anticlockwise].visited {
            unvisited.push((ring, anticlockwise));
        }
    
        // Check neighboring division on the previous ring if not on the innermost ring
        if ring > 0 {
            let inner_divisions = self.divisions_in_ring(ring - 1);
            let mapped_inner_division = (division * inner_divisions) / ring_divisions;
            if !cells[ring - 1][mapped_inner_division].visited {
                unvisited.push((ring - 1, mapped_inner_division));
            }
        }
    
        // Check the neighboring divisions on the next ring if not on the outermost ring
        if ring < self.rings - 1 {
            let outer_divisions = self.divisions_in_ring(ring + 1);
            let mapped_outer_division = (division * outer_divisions) / ring_divisions;
            if !cells[ring + 1][mapped_outer_division].visited {
                unvisited.push((ring + 1, mapped_outer_division));
            }
        }
    
        unvisited
    }

    pub fn get_cells(&mut self, seed: [u32; 4]) -> Vec<Vec<Cell>> {

        let mut cells: Vec<Vec<Cell>> = (0..self.rings).map(|ring| {vec![Cell::new(); self.divisions_in_ring(ring)]}).collect();

        // A closure for generating random floats (f64) using bitshift operations
        let mut s_random = SFC32::new(seed);
    
        // Mark all cells in the centre as visited and remove their radial walls
        cells[0].iter_mut().for_each(|cell| {
            cell.right_wall = false;
            cell.visited = true;
        });
    
        // Mark the starting cell in the outermost ring as visited and remove its outer wall
        cells[self.rings - 1][0].visited = true;
        cells[self.rings - 1][0].outer_wall = false;
    
        let mut centre_ring_division = 0;
    
        // Begin with outermost cell
        let mut ring = self.rings - 1;
        let mut division = 0;
    
        // Backtracking queue for maze generation
        let mut backtrack_path = vec![];

        loop {
            // Randomly choose an unvisited neighbor of the current cell
            let unvisited: Vec<(usize, usize)> = self.unvisited_neighbours(&cells, ring, division);
    
            if !unvisited.is_empty() {

                backtrack_path.push((ring, division));

                // Pick a random unvisited neighboring cell
                let neighbour_index: usize = s_random.rand_between(0, unvisited.len());
                let (next_ring, next_division) = unvisited[neighbour_index];
    
                // Open the wall between the current cell and the chosen neighboring cell
                self.open_wall_between(&mut cells, (ring, division), (next_ring, next_division));
    
                // Move to the neighboring cell and mark it as visited
                ring = next_ring;
                division = next_division;
                cells[ring][division].visited = true;
    
                // Record the centre and the final division in the solution
                if ring == 1 {
                    centre_ring_division = division;
                    self.solution = backtrack_path.clone();
                    self.solution.push((ring, division));
                    self.solution.push((0, division));
                }

            } else if let Some((prev_ring, prev_division)) = backtrack_path.pop() {
                // If there are no unvisited neighbors, backtrack to the previous cell
                ring = prev_ring;
                division = prev_division;

            } else {
                // No unvisited neighbors and no cells to backtrack to
                // Assume the maze is completed
                break;
            }
        }
    
        // Remove the inner wall at the centre to open it up
        cells[1][centre_ring_division].inner_wall = false;

        cells

    }

    pub fn draw_solution(&self, bitmap: &Bitmap, centre: (f64, f64)) {

        let mut bitmap = bitmap.clone();

        if self.solution.is_empty() {
            println!("ERROR: Empty solution!");
            return;
        }

        let mut prev: Option<(f64, f64)> = None;
        let stroke_width = 1;
    
        for &(ring, division) in &self.solution {

            let angle_per_division = 2.0 * std::f64::consts::PI / self.divisions_in_ring(ring) as f64;

            let angular_offset = angle_per_division * division as f64 + angle_per_division / 2.0;
            let radial_offset = self.ring_gap as f64 * ring as f64 + self.ring_gap as f64 / 2.0;

            let pole = (angular_offset.cos(), angular_offset.sin()); // UNIT CIRCLE POS [-1, 1]
            let point = centre.add(pole.mul(radial_offset)); // CARTESIAN COORDS

            if let Some(prev) = prev {
                bitmap.line(prev, point, RED, stroke_width);
            }

            prev = Some(point);

        }

        bitmap.save("solution");
    }
    
    pub fn arcs_and_walls(
        &self,
        cells: Vec<Vec<Cell>>,
        wall_width: u32, // STROKE WIDTH
        centre: (f64, f64),
    ) -> (Vec<(f64, f64, f64)>, Vec<((f64, f64), (f64, f64))>) {
        let mid_wall = (wall_width / 2) as f64;
    
        (0..self.rings)
        .into_par_iter()
        .map(|ring| {

            let mut ring_arcs = Vec::new();
            let mut ring_lines = Vec::new();
    
            let outermost_ring = ring == self.rings - 1;
    
            let divisions = self.divisions_in_ring(ring);
            let arc_angle = 2.0 * std::f64::consts::PI / divisions as f64;
    
            let inner_wall_radius = self.ring_gap as f64 * ring as f64;
            let outer_wall_radius = inner_wall_radius + self.ring_gap as f64;
    
            for division in 0..divisions {
                let cell = &cells[ring][division];
    
                let angle_beginning = arc_angle * division as f64;
                let angle_ending = angle_beginning + arc_angle;
    
                // Inner rings
                if cell.inner_wall && inner_wall_radius != 0.0 {
                    ring_arcs.push((inner_wall_radius, angle_beginning, angle_ending));
                }
    
                // Radial walls
                if cell.right_wall {
                    let pole = (angle_ending.cos(), angle_ending.sin()); // UNIT CIRCLE POS [-1, 1]
                    let p1 = centre.add(pole.mul(inner_wall_radius - mid_wall)); // CARTESIAN COORDS
                    let p2 = centre.add(pole.mul(outer_wall_radius + mid_wall));
                    ring_lines.push((p1, p2));
                }
    
                // Outermost ring
                if outermost_ring && cell.outer_wall {
                    ring_arcs.push((outer_wall_radius, angle_beginning, angle_ending));
                }
            }
            (ring_arcs, ring_lines)
        })
        // Rayon’s reduce is used to directly combine the results of parallel computations.
        // Instead of collecting nested vectors and flattening them afterward.
        // This ring's arcs and lines will be collected and returned.
        .reduce(
            || (Vec::new(), Vec::new()),
            |(mut arcs, mut lines), (ring_arcs, ring_lines)| {
                arcs.extend(ring_arcs);
                lines.extend(ring_lines);
                (arcs, lines)
            },
        )
    }

    /// A detail value closer to 1 leads to a more granular approximation
    pub fn generate(&mut self, seed: [u32; 4], wall_width: u32, wall_height: u32, granularity: f64, solve: bool) -> Vec<Brick> { // [11, 12, 15, 2];

        let cells = self.get_cells(seed);

        let radius = self.ring_gap * self.rings as u32 + wall_width;
        let centre = (radius as f64, radius as f64);

        let (arcs, lines) = self.arcs_and_walls(cells, wall_width, centre);

        let image_width = 2 * radius as usize;
        let mut bitmap = Bitmap::new(image_width);
        
        for (radius, angle_begin, angle_end) in arcs {
            bitmap.arc(centre, radius, angle_begin, angle_end, BLACK, wall_width);
        }

        for (begin, end) in lines {
            bitmap.line(begin, end, BLACK, wall_width);
        }

        if solve {
            self.draw_solution(&bitmap, centre);
        }

        let bricks: Vec<Brick> = brick_pixels(&mut bitmap, wall_height);

        let factor = (1.0 / granularity).round().max(1.0) as u32;
        bitmap.downscale(factor);
        

        bitmap.save("maze");
        
        bricks
         
    }
    
}
