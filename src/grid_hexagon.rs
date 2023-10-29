use crate::AppState;

use super::grid_triangle::draw_triangle_equilateral;

use super::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexagonKind {
    Fill1,
    Fill2,
    Fill3,
    Vertical12,
    Diagonal23,
    Diagonal31,
    Inside123,
    Outside123,
    Forbidden,
}

enum HexagonColors {
    OneColor(image::Rgb<u8>),
    TwoColor(([image::Rgb<u8>; 2], u8)),
    ThreeColor(([image::Rgb<u8>; 3], u8)),
}

pub fn draw_hexagon(
    state: &mut AppState,
    center: Point,
    sub_triangle_height: f32,
    kind: HexagonKind,
) {
    if kind == HexagonKind::Forbidden {
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

    let colors = match kind {
        HexagonKind::Fill1 => HexagonColors::OneColor(COLOR1),
        HexagonKind::Fill2 => HexagonColors::OneColor(COLOR2),
        HexagonKind::Fill3 => HexagonColors::OneColor(COLOR3),
        HexagonKind::Vertical12 => HexagonColors::TwoColor(([COLOR1, COLOR2], 5)),
        HexagonKind::Diagonal23 => HexagonColors::TwoColor(([COLOR2, COLOR3], 1)),
        HexagonKind::Diagonal31 => HexagonColors::TwoColor(([COLOR3, COLOR1], 3)),
        HexagonKind::Inside123 => HexagonColors::ThreeColor(([COLOR1, COLOR2, COLOR3], 0)),
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
    }
}
