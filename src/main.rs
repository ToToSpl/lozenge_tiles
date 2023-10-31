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

fn generate_hexagon(state: &mut AppState, side_len: usize, triangle_height: f32) {
    state
        .hexagon_map
        .insert(state.rounder.get_coord(0.0, 0.0), HexagonKind::Inside123);

    let kinds = [
        HexagonKind::Vertical12,
        HexagonKind::Diagonal23,
        HexagonKind::Diagonal31,
    ];
    let mut points = [Point::new(0.0, 0.0); 3];

    let mut angle = -std::f32::consts::FRAC_PI_2;
    for point in &mut points {
        point.x = 2.0 * triangle_height / f32::sqrt(3.0) * f32::cos(angle);
        point.y = 2.0 * triangle_height / f32::sqrt(3.0) * f32::sin(angle);
        angle += 2.0 * std::f32::consts::FRAC_PI_3;
    }

    for i in 1..side_len {
        for (p, k) in std::iter::zip(points, kinds) {
            state
                .hexagon_map
                .insert(state.rounder.get_coord(i as f32 * p.x, i as f32 * p.y), k);
        }
    }

    let mut dir_vecs = [Point::new(0.0, 0.0); 6];

    let mut angle = -std::f32::consts::FRAC_PI_2;
    for vec in &mut dir_vecs {
        vec.x = (side_len as f32 - 0.1) * 2.0 * triangle_height / f32::sqrt(3.0) * f32::cos(angle);
        vec.y = (side_len as f32 - 0.1) * 2.0 * triangle_height / f32::sqrt(3.0) * f32::sin(angle);
        angle += std::f32::consts::FRAC_PI_3;
    }

    let mut boundary_vecs = [Point::new(0.0, 0.0); 6];

    for i in 0..dir_vecs.len() - 1 {
        boundary_vecs[i].x = dir_vecs[i + 1].x - dir_vecs[i].x;
        boundary_vecs[i].y = dir_vecs[i + 1].y - dir_vecs[i].y;
    }
    boundary_vecs[dir_vecs.len() - 1].x = dir_vecs[0].x - dir_vecs[dir_vecs.len() - 1].x;
    boundary_vecs[dir_vecs.len() - 1].y = dir_vecs[0].y - dir_vecs[dir_vecs.len() - 1].y;

    let mut stack = Vec::new();
    let mut angle = -std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_3;
    for _ in 0..3 {
        let x = 2.0 * triangle_height / f32::sqrt(3.0) * f32::cos(angle);
        let y = 2.0 * triangle_height / f32::sqrt(3.0) * f32::sin(angle);
        stack.push(state.rounder.get_coord(x, y));
        angle += 2.0 * std::f32::consts::FRAC_PI_3;
    }

    'outer: while let Some(curr) = stack.pop() {
        if state.hexagon_map.contains_key(&curr) {
            continue;
        }
        for (vec, start) in std::iter::zip(boundary_vecs, dir_vecs) {
            let curr_vec = Point::new(curr.0 as f32 - start.x, curr.1 as f32 - start.y);
            if Point::cross_mag(&curr_vec, &vec) > 0.0 {
                continue 'outer;
            }
        }

        let mut angle = -std::f32::consts::FRAC_PI_2;
        for _ in 0..3 {
            let x = curr.0 as f32 + 2.0 * triangle_height / f32::sqrt(3.0) * f32::cos(angle);
            let y = curr.1 as f32 + 2.0 * triangle_height / f32::sqrt(3.0) * f32::sin(angle);
            stack.push(state.rounder.get_coord(x, y));
            angle += 2.0 * std::f32::consts::FRAC_PI_3;
        }

        let mut checks = [false; 3];
        for i in 0..3 {
            let curr_vec = Point::new(curr.0 as f32, curr.1 as f32);
            checks[i] = Point::cross_mag(&curr_vec, &dir_vecs[2 * i + 1]) > 0.0;
        }
        if checks[0] && !checks[1] {
            state.hexagon_map.insert(curr, HexagonKind::Fill1);
        } else if checks[1] && !checks[2] {
            state.hexagon_map.insert(curr, HexagonKind::Fill2);
        } else if checks[2] && !checks[0] {
            state.hexagon_map.insert(curr, HexagonKind::Fill3);
        }
    }
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

    generate_hexagon(&mut state, 6, triangle_height);

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
