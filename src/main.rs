mod point;
use point::Point;
mod grid_triangle;
use grid_triangle::GridTriangle;
mod grid_hexagon;

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

    let grid = GridTriangle::new(6, 11, 50.0, Point::new(-100.0, -100.0));
    for y in 0..grid.height {
        for x in 0..grid.width {
            grid.draw(&mut imgbuf, x, y, colors[(x + y) % colors.len()]);
        }
    }

    imgbuf.save("triangles.png").unwrap();
}
