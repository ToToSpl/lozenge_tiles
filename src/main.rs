use std::collections::{HashMap, HashSet};
mod point;
use point::Point;
mod draw_figs;
use draw_figs::{draw_hexagon, HexagonKind};
mod grid_rounder;
use grid_rounder::GridRounder;

pub struct AppState<'a> {
    pub buf: &'a mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub hexagon_map: &'a mut HashMap<(i32, i32), HexagonKind>,
    pub triangles_drawn: &'a mut HashSet<(i32, i32)>,
    pub rounder: &'a GridRounder,
}

fn main() {
    let imgx = 851_usize;
    let imgy = 851_usize;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Iterate over the coordinates and pixels of the image
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    let triangle_height = 50.0;

    let mut hexagon_map: HashMap<(i32, i32), HexagonKind> = HashMap::new();
    let mut triangles_drawn: HashSet<(i32, i32)> = HashSet::new();
    let center = Point::new(imgx as f32 / 2.0, imgy as f32 / 2.0);
    let rounder = GridRounder::new(
        center,
        triangle_height / 2.0,
        triangle_height / f32::sqrt(3.0),
    );

    let mut state = AppState {
        buf: &mut imgbuf,
        hexagon_map: &mut hexagon_map,
        triangles_drawn: &mut triangles_drawn,
        rounder: &rounder,
    };

    state
        .hexagon_map
        .insert(rounder.get_coord(0.0, 0.0), HexagonKind::Inside123);
    {
        let kinds = [
            HexagonKind::Vertical12,
            HexagonKind::Diagonal23,
            HexagonKind::Diagonal31,
        ];
        let mut points = [Point::new(0.0, 0.0); 3];

        let mut angle = -std::f32::consts::FRAC_PI_2;
        for i in 0..points.len() {
            points[i].x = 2.0 * triangle_height / f32::sqrt(3.0) * f32::cos(angle);
            points[i].y = 2.0 * triangle_height / f32::sqrt(3.0) * f32::sin(angle);
            angle += 2.0 * std::f32::consts::FRAC_PI_3;
        }

        for i in 1..6 {
            for (j, p) in points.iter().enumerate() {
                state
                    .hexagon_map
                    .insert(rounder.get_coord(i as f32 * p.x, i as f32 * p.y), kinds[j]);
            }
        }
    }

    for (coord, kind) in state.hexagon_map.clone() {
        draw_hexagon(
            &mut state,
            Point::new(center.x + coord.0 as f32, center.y + coord.1 as f32),
            1.01 * triangle_height,
            kind,
        );
    }

    imgbuf.save("triangles.png").unwrap();
}
