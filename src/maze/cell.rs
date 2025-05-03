#[derive(Clone)]
pub struct Cell {
    pub visited: bool,
    pub inner_wall: bool,
    pub outer_wall: bool,
    pub right_wall: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            visited: false,
            inner_wall: true,
            outer_wall: true,
            right_wall: true,
        }
    }
}