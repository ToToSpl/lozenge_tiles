// structure to clip coordinates to set locations

use super::Point;

#[derive(Copy, Clone, Debug)]
pub struct GridRounder {
    pub center: Point,
    pub grid_width: f32,
    pub grid_height: f32,
}

impl GridRounder {
    pub fn new(center: Point, grid_width: f32, grid_height: f32) -> GridRounder {
        GridRounder {
            center,
            grid_width,
            grid_height,
        }
    }

    pub fn get_coord(self, x: f32, y: f32) -> (i32, i32) {
        let x = (x / self.grid_width).round() * self.grid_width;
        let y = (y / self.grid_height).round() * self.grid_height;
        (x as i32, y as i32)
    }
}
