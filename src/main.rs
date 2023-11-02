use rand::Rng;
use std::collections::{HashMap, HashSet};
use tqdm::tqdm;
mod point;
use point::Point;
mod hexagon;
use hexagon::{
    Hexagon, HEX_EDGE1, HEX_EDGE2, HEX_EDGE3, HEX_FILL1, HEX_FILL2, HEX_FILL3, HEX_INSIDE,
    HEX_OUTSIDE,
};
mod grid_rounder;
use grid_rounder::GridRounder;

const ITER_AMOUNT: usize = 1_000_000;
const IMAGE_WIDTH: usize = 8501;
const IMAGE_HEIGHT: usize = 8501;
const HEXAGON_LEN: usize = 60;
const TRIANGLE_HEIGHT: f32 = 50.0;
const COLOR1: [u8; 3] = [225, 117, 46];
const COLOR2: [u8; 3] = [114, 225, 105];
const COLOR3: [u8; 3] = [105, 154, 225];

pub struct AppState<'a> {
    pub buf: &'a mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub hexagon_map: &'a mut HashMap<(i32, i32), Hexagon>,
    pub triangles_drawn: &'a mut HashSet<(i32, i32)>,
    pub modifiable_hexagons: &'a mut HashSet<(i32, i32)>,
    pub rounder: &'a GridRounder,
    pub colors: [image::Rgb<u8>; 3],
    pub triangle_height: f32,
}

fn get_random_from_set(set: &mut HashSet<(i32, i32)>) -> (i32, i32) {
    let len = set.len();
    if len == 0 {
        panic!("Set cannot be zero len!");
    }
    let rand_num = rand::thread_rng().gen_range(0..len);
    match set.iter().skip(rand_num).next() {
        Some(elem) => *elem,
        None => panic!("Should contain element in this range!"),
    }
}

fn generate_hexagon(state: &mut AppState, side_len: usize) {
    state
        .hexagon_map
        .insert(state.rounder.get_coord(0.0, 0.0), Hexagon::new(HEX_INSIDE));

    let kinds = [HEX_EDGE1, HEX_EDGE2, HEX_EDGE3];
    let mut points = [Point::new(0.0, 0.0); 3];

    let mut angle = -std::f32::consts::FRAC_PI_2;
    for point in &mut points {
        point.x = 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::cos(angle);
        point.y = 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::sin(angle);
        angle += 2.0 * std::f32::consts::FRAC_PI_3;
    }

    for i in 1..side_len {
        for (p, k) in std::iter::zip(points, kinds) {
            state.hexagon_map.insert(
                state.rounder.get_coord(i as f32 * p.x, i as f32 * p.y),
                Hexagon::new(k),
            );
        }
    }

    let mut dir_vecs = [Point::new(0.0, 0.0); 6];

    let mut angle = -std::f32::consts::FRAC_PI_2;
    for vec in &mut dir_vecs {
        vec.x = (side_len as f32 - 0.1) * 2.0 * state.triangle_height / f32::sqrt(3.0)
            * f32::cos(angle);
        vec.y = (side_len as f32 - 0.1) * 2.0 * state.triangle_height / f32::sqrt(3.0)
            * f32::sin(angle);
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
        let x = 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::cos(angle);
        let y = 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::sin(angle);
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
            let x = curr.0 as f32 + 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::cos(angle);
            let y = curr.1 as f32 + 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::sin(angle);
            stack.push(state.rounder.get_coord(x, y));
            angle += 2.0 * std::f32::consts::FRAC_PI_3;
        }

        let mut checks = [false; 3];
        for i in 0..3 {
            let curr_vec = Point::new(curr.0 as f32, curr.1 as f32);
            checks[i] = Point::cross_mag(&curr_vec, &dir_vecs[2 * i + 1]) > 0.0;
        }
        if checks[0] && !checks[1] {
            state.hexagon_map.insert(curr, Hexagon::new(HEX_FILL3));
        } else if checks[1] && !checks[2] {
            state.hexagon_map.insert(curr, Hexagon::new(HEX_FILL1));
        } else if checks[2] && !checks[0] {
            state.hexagon_map.insert(curr, Hexagon::new(HEX_FILL2));
        }
    }
    state.modifiable_hexagons.insert((0, 0));
}

fn change_hex_grid(state: &mut AppState, coord: &(i32, i32)) -> Result<(), ()> {
    let hex = if let Some(hex) = state.hexagon_map.get_mut(coord) {
        hex
    } else {
        return Result::Err(());
    };

    let into = if hex.is_inside() {
        HEX_OUTSIDE
    } else {
        HEX_INSIDE
    };

    hex.tiles = into;

    let mut angle = -std::f32::consts::FRAC_PI_2;
    for i in 0..6 {
        let x = coord.0 as f32 + 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::cos(angle);
        let y = coord.1 as f32 + 2.0 * state.triangle_height / f32::sqrt(3.0) * f32::sin(angle);
        let c = state.rounder.get_coord(x, y);
        if let Some(hex_neigh) = state.hexagon_map.get_mut(&c) {
            hex_neigh.change_state((i + 3) % 6, &into);
            if hex_neigh.is_inside() || hex_neigh.is_outside() {
                state.modifiable_hexagons.insert(c);
            } else {
                state.modifiable_hexagons.remove(&c);
            }
        }
        angle += std::f32::consts::FRAC_PI_3;
    }
    Ok(())
}

fn main() {
    let imgx = IMAGE_WIDTH;
    let imgy = IMAGE_HEIGHT;
    let triangle_height = TRIANGLE_HEIGHT;

    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    let mut hexagon_map: HashMap<(i32, i32), Hexagon> = HashMap::new();
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
        modifiable_hexagons: &mut HashSet::new(),
        rounder: &rounder,
        colors: [image::Rgb(COLOR1), image::Rgb(COLOR2), image::Rgb(COLOR3)],
        triangle_height,
    };

    generate_hexagon(&mut state, HEXAGON_LEN);
    println!("INFO: Begin monte carlo roll");
    for _ in tqdm(0..ITER_AMOUNT) {
        if state.modifiable_hexagons.len() == 0 {
            println!("INFO: Reached locked state. Exiting prematurly");
            break;
        }
        let rand_coord = get_random_from_set(state.modifiable_hexagons);
        let _ = change_hex_grid(&mut state, &rand_coord);
    }

    println!("INFO: Drawing into image...");
    for (coord, hex) in state.hexagon_map.clone() {
        hex.draw(
            &mut state,
            Point::new(center.x + coord.0 as f32, center.y + coord.1 as f32),
        );
    }

    println!("INFO: Saving image...");
    state.buf.save("triangles.png").unwrap();
}
