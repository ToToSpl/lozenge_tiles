use super::Point;
use crate::AppState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexagonKind {
    Fill1,
    Fill2,
    Fill3,
    Vertical12,
    Diagonal23,
    Diagonal31,
    Edge1,
    Edge2,
    Edge3,
    Inside123,
    Outside123,
    Forbidden,
}

enum HexagonColors {
    OneColor(image::Rgb<u8>),
    TwoColor(([image::Rgb<u8>; 2], u8)),
    ThreeColor(([image::Rgb<u8>; 3], u8)),
    ThreeColorAssym(([image::Rgb<u8>; 3], u8)),
}

#[derive(Clone, Copy, Debug)]
pub struct Hexagon {
    pub kind: HexagonKind,
}

impl Hexagon {
    pub fn new(kind: HexagonKind) -> Hexagon {
        Hexagon { kind }
    }

    pub fn draw(self, state: &mut AppState, center: Point, sub_triangle_height: f32) {
        if self.kind == HexagonKind::Forbidden {
            return;
        }

        const COLOR1: image::Rgb<u8> = image::Rgb([105, 154, 225]);
        const COLOR2: image::Rgb<u8> = image::Rgb([225, 117, 46]);
        const COLOR3: image::Rgb<u8> = image::Rgb([114, 225, 105]);

        const ANGLES: [f32; 2] = [-std::f32::consts::FRAC_PI_6, std::f32::consts::FRAC_PI_6]; // right, left

        let mut points = [Point::new(0.0, 0.0); 6];
        {
            let mut angle = std::f32::consts::PI;
            for i in 0..points.len() {
                points[i].x = center.x + (2.0 / 3.0 * sub_triangle_height) * f32::cos(angle);
                points[i].y = center.y + (2.0 / 3.0 * sub_triangle_height) * f32::sin(angle);
                angle += std::f32::consts::FRAC_PI_3;
            }
        }

        let colors = match self.kind {
            HexagonKind::Fill1 => HexagonColors::OneColor(COLOR1),
            HexagonKind::Fill2 => HexagonColors::OneColor(COLOR2),
            HexagonKind::Fill3 => HexagonColors::OneColor(COLOR3),
            HexagonKind::Vertical12 => HexagonColors::TwoColor(([COLOR1, COLOR2], 5)),
            HexagonKind::Diagonal23 => HexagonColors::TwoColor(([COLOR2, COLOR3], 1)),
            HexagonKind::Diagonal31 => HexagonColors::TwoColor(([COLOR3, COLOR1], 3)),
            HexagonKind::Inside123 => HexagonColors::ThreeColor(([COLOR1, COLOR2, COLOR3], 0)),
            HexagonKind::Edge1 => HexagonColors::ThreeColorAssym(([COLOR1, COLOR3, COLOR2], 5)),
            HexagonKind::Edge2 => HexagonColors::ThreeColorAssym(([COLOR2, COLOR1, COLOR3], 1)),
            HexagonKind::Edge3 => HexagonColors::ThreeColorAssym(([COLOR3, COLOR2, COLOR1], 3)),
            HexagonKind::Outside123 => HexagonColors::ThreeColor(([COLOR1, COLOR2, COLOR3], 3)),
            HexagonKind::Forbidden => panic!("Should not happen!"),
        };

        match colors {
            HexagonColors::OneColor(col) => {
                for (i, p) in points.iter().enumerate() {
                    draw_triangle_equilateral(state, sub_triangle_height, *p, ANGLES[i % 2], col)
                }
            }
            HexagonColors::TwoColor((cols, start)) => {
                let mut i_p = start;
                for i in 0..points.len() {
                    draw_triangle_equilateral(
                        state,
                        sub_triangle_height,
                        points[i_p as usize],
                        ANGLES[(i_p % 2) as usize],
                        cols[i / 3],
                    );

                    i_p = (i_p + 1) % points.len() as u8;
                }
            }
            HexagonColors::ThreeColor((cols, start)) => {
                let mut i_p = start;
                for i in 0..points.len() {
                    draw_triangle_equilateral(
                        state,
                        sub_triangle_height,
                        points[i_p as usize],
                        ANGLES[(i_p % 2) as usize],
                        cols[i / 2],
                    );

                    i_p = (i_p + 1) % points.len() as u8;
                }
            }
            HexagonColors::ThreeColorAssym((cols, start)) => {
                let mut i_p = start;
                for i in 0..points.len() {
                    let col = if i <= 3 { cols[0] } else { cols[i - 3] };
                    draw_triangle_equilateral(
                        state,
                        sub_triangle_height,
                        points[i_p as usize],
                        ANGLES[(i_p % 2) as usize],
                        col,
                    );

                    i_p = (i_p + 1) % points.len() as u8;
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
    }
}
