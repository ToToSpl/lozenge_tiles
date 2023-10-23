use image;

#[derive(Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn cross_mag(a: &Point, b: &Point) -> f32 {
        a.x * b.y - a.y * b.x
    }
}

fn main() {
    let imgx = 800;
    let imgy = 800;

    let a = Point::new(100.0, 700.0);
    let b = Point::new(400.0, 100.0);
    let c = Point::new(700.0, 700.0);

    let vec_ab = Point::new(a.x - b.x, a.y - b.y);
    let vec_bc = Point::new(b.x - c.x, b.y - c.y);
    let vec_ca = Point::new(c.x - a.x, c.y - a.y);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let v_a = Point::new(x as f32 - a.x, y as f32 - a.y);
            let v_b = Point::new(x as f32 - b.x, y as f32 - b.y);
            let v_c = Point::new(x as f32 - c.x, y as f32 - c.y);

            let test_ab = Point::cross_mag(&v_a, &vec_ab) >= 0.0;
            let test_bc = Point::cross_mag(&v_b, &vec_bc) >= 0.0;
            let test_ca = Point::cross_mag(&v_c, &vec_ca) >= 0.0;

            if test_ab && test_bc && test_ca {
                let pixel = imgbuf.get_pixel_mut(x, y);
                let image::Rgb(data) = *pixel;
                *pixel = image::Rgb([data[0], 255 as u8, data[2]]);
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
