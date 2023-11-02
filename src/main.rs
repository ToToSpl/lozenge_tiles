use rand::Rng;
use std::collections::{HashMap, HashSet};
use tqdm::tqdm;
mod point;
use point::Point;
mod hexagon;
use hexagon::Hexagon;
mod hexagon_utils;
use hexagon_utils::{change_hex_grid, generate_hexagon};
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
