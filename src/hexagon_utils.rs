use super::hexagon::{
    Hexagon, HEX_EDGE1, HEX_EDGE2, HEX_EDGE3, HEX_FILL1, HEX_FILL2, HEX_FILL3, HEX_INSIDE,
    HEX_OUTSIDE,
};
use super::point::Point;
use super::AppState;

pub fn generate_hexagon(state: &mut AppState, side_len: usize) {
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

pub fn change_hex_grid(state: &mut AppState, coord: &(i32, i32)) -> Result<(), ()> {
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
