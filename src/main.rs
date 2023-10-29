mod point;
use point::Point;
mod grid_triangle;
use grid_triangle::GridTriangle;
mod grid_hexagon;
mod grid_rounder;
use grid_rounder::GridRounder;

fn main() {
    let imgx = 450_usize;
    let imgy = 450_usize;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Iterate over the coordinates and pixels of the image
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    let colors = [
        image::Rgb([105, 154, 225]),
        image::Rgb([225, 117, 46]),
        image::Rgb([114, 225, 105]),
    ];

    let triangle_height = 50.0;

    let grid = GridTriangle::new(6, 11, triangle_height, Point::new(109.0, 81.0));
    for y in 0..grid.height {
        for x in 0..grid.width {
            grid.draw(&mut imgbuf, x, y, colors[(x + y) % colors.len()]);
        }
    }

    let center = Point::new(imgx as f32 / 2.0, imgy as f32 / 2.0);
    let rounder = GridRounder::new(
        center,
        triangle_height / 2.0,
        triangle_height / f32::sqrt(3.0),
    );

    for y in 0..imgy {
        for x in 0..imgx {
            let coords = rounder.get_coord((x as f32) - center.x, (y as f32) - center.y);
            let coord_x = (coords.0 + center.x as i32) as u32;
            let coord_y = (coords.1 + center.y as i32) as u32;
            if coord_x >= imgx as u32 || coord_y >= imgy as u32 {
                continue;
            }

            let pixel = imgbuf.get_pixel_mut(coord_x, coord_y);
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    imgbuf.save("triangles.png").unwrap();
}
