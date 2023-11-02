use super::Point;
use crate::AppState;

pub const HEX_INSIDE: [HexagonColor; 6] = [
    HexagonColor::Color1,
    HexagonColor::Color1,
    HexagonColor::Color2,
    HexagonColor::Color2,
    HexagonColor::Color3,
    HexagonColor::Color3,
];

pub const HEX_OUTSIDE: [HexagonColor; 6] = [
    HexagonColor::Color2,
    HexagonColor::Color3,
    HexagonColor::Color3,
    HexagonColor::Color1,
    HexagonColor::Color1,
    HexagonColor::Color2,
];

pub const HEX_FILL1: [HexagonColor; 6] = [HexagonColor::Color1; 6];
pub const HEX_FILL2: [HexagonColor; 6] = [HexagonColor::Color2; 6];
pub const HEX_FILL3: [HexagonColor; 6] = [HexagonColor::Color3; 6];

pub const HEX_EDGE1: [HexagonColor; 6] = [
    HexagonColor::Color1,
    HexagonColor::Color1,
    HexagonColor::Color1,
    HexagonColor::Color3,
    HexagonColor::Color3,
    HexagonColor::Color3,
];

pub const HEX_EDGE2: [HexagonColor; 6] = [
    HexagonColor::Color1,
    HexagonColor::Color1,
    HexagonColor::Color2,
    HexagonColor::Color2,
    HexagonColor::Color2,
    HexagonColor::Color1,
];

pub const HEX_EDGE3: [HexagonColor; 6] = [
    HexagonColor::Color3,
    HexagonColor::Color2,
    HexagonColor::Color2,
    HexagonColor::Color2,
    HexagonColor::Color3,
    HexagonColor::Color3,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexagonColor {
    Color1,
    Color2,
    Color3,
}

#[derive(Clone, Copy, Debug)]
pub struct Hexagon {
    pub tiles: [HexagonColor; 6],
}

impl Hexagon {
    pub fn new(tiles: [HexagonColor; 6]) -> Self {
        Self { tiles }
    }

    pub fn change_state(&mut self, side: u8, swap_type: &[HexagonColor; 6]) {
        let (first, second) = match side {
            0 => (5, 0),
            _ => (side, side - 1),
        };
        let (first_next, second_next) = (((first + 3) % 6), ((second + 3) % 6));
        self.tiles[second as usize] = swap_type[first_next as usize];
        self.tiles[first as usize] = swap_type[second_next as usize];
    }

    pub fn is_inside(&self) -> bool {
        return self.tiles == HEX_INSIDE;
    }

    pub fn is_outside(&self) -> bool {
        return self.tiles == HEX_OUTSIDE;
    }

    pub fn draw(&self, state: &mut AppState, center: Point) {
        const ANGLES: [f32; 2] = [-std::f32::consts::FRAC_PI_6, std::f32::consts::FRAC_PI_6]; // right, left

        let mut points = [Point::new(0.0, 0.0); 6];
        let mut angle = -std::f32::consts::FRAC_PI_3;
        for point in &mut points {
            point.x = center.x + (2.0 / 3.0 * state.triangle_height) * f32::cos(angle);
            point.y = center.y + (2.0 / 3.0 * state.triangle_height) * f32::sin(angle);
            angle += std::f32::consts::FRAC_PI_3;
        }

        for (i, (p, c)) in std::iter::zip(points, self.tiles).enumerate() {
            let c_rgb = match c {
                HexagonColor::Color1 => state.colors[0],
                HexagonColor::Color2 => state.colors[1],
                HexagonColor::Color3 => state.colors[2],
            };
            draw_triangle_equilateral(state, 1.01 * state.triangle_height, p, ANGLES[i % 2], c_rgb);
        }
    }
}

fn draw_triangle_equilateral(
    state: &mut AppState,
    height: f32,
    center: Point,
    angle: f32,
    color: image::Rgb<u8>,
) {
    // check if triangle already drawn
    {
        let coord = state.rounder.get_coord(center.x, center.y);
        if state.triangles_drawn.contains(&coord) {
            return;
        } else {
            state.triangles_drawn.insert(coord);
        }
    }
    let mag = (2.0 / 3.0) * height;
    let c = Point::new(
        center.x + mag * f32::sin(0.0 + angle),
        center.y + mag * f32::cos(0.0 + angle),
    );
    let b = Point::new(
        center.x + mag * f32::sin(2.0 * std::f32::consts::FRAC_PI_3 + angle),
        center.y + mag * f32::cos(2.0 * std::f32::consts::FRAC_PI_3 + angle),
    );
    let a = Point::new(
        center.x + mag * f32::sin(4.0 * std::f32::consts::FRAC_PI_3 + angle),
        center.y + mag * f32::cos(4.0 * std::f32::consts::FRAC_PI_3 + angle),
    );

    let vec_ab = Point::new(a.x - b.x, a.y - b.y);
    let vec_bc = Point::new(b.x - c.x, b.y - c.y);
    let vec_ca = Point::new(c.x - a.x, c.y - a.y);

    let min_x = f32::floor(a.x.min(f32::min(b.x, c.x))) as u32;
    let max_x = u32::min(
        f32::ceil(a.x.max(f32::max(b.x, c.x))) as u32,
        state.buf.width(),
    );
    let min_y = f32::floor(a.y.min(f32::min(b.y, c.y))) as u32;
    let max_y = u32::min(
        f32::ceil(a.y.max(f32::max(b.y, c.y))) as u32,
        state.buf.height(),
    );

    for x in min_x..max_x {
        for y in min_y..max_y {
            let v_a = Point::new(x as f32 - a.x, y as f32 - a.y);
            let v_b = Point::new(x as f32 - b.x, y as f32 - b.y);
            let v_c = Point::new(x as f32 - c.x, y as f32 - c.y);

            let test_ab = Point::cross_mag(&v_a, &vec_ab) >= 0.0;
            let test_bc = Point::cross_mag(&v_b, &vec_bc) >= 0.0;
            let test_ca = Point::cross_mag(&v_c, &vec_ca) >= 0.0;

            if test_ab && test_bc && test_ca {
                let pixel = state.buf.get_pixel_mut(x, y);
                *pixel = color;
            }
        }
    }
}
