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

fn draw_triangle_equilateral(
    buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    height: f32,
    center: Point,
    angle: f32,
    color: image::Rgb<u8>,
) {
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

    let min_x = a.x.min(f32::min(b.x, c.x)) as u32;
    let max_x = a.x.max(f32::max(b.x, c.x)) as u32;
    let min_y = a.y.min(f32::min(b.y, c.y)) as u32;
    let max_y = a.y.max(f32::max(b.y, c.y)) as u32;

    // println!("{:}, {:}\n{:}, {:}", min_x, max_x, min_y, max_y);

    for x in min_x..max_x {
        for y in min_y..max_y {
            let v_a = Point::new(x as f32 - a.x, y as f32 - a.y);
            let v_b = Point::new(x as f32 - b.x, y as f32 - b.y);
            let v_c = Point::new(x as f32 - c.x, y as f32 - c.y);

            let test_ab = Point::cross_mag(&v_a, &vec_ab) >= 0.0;
            let test_bc = Point::cross_mag(&v_b, &vec_bc) >= 0.0;
            let test_ca = Point::cross_mag(&v_c, &vec_ca) >= 0.0;

            if test_ab && test_bc && test_ca {
                let pixel = buf.get_pixel_mut(x, y);
                *pixel = color;
            }
        }
    }
}

fn main() {
    let imgx = 800 as usize;
    let imgy = 800 as usize;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    let colors = vec![
        image::Rgb([148, 0, 211]),
        image::Rgb([75, 0, 130]),
        image::Rgb([0, 0, 255]),
        image::Rgb([0, 255, 0]),
        image::Rgb([255, 255, 0]),
        image::Rgb([255, 127, 0]),
        image::Rgb([255, 0, 0]),
    ];
    let mut i = 0;
    let height = 50 as usize;

    // A redundant loop to demonstrate reading image data
    for x in (height..imgx).step_by(height) {
        for y in (height..imgy).step_by(height) {
            draw_triangle_equilateral(
                &mut imgbuf,
                height as f32,
                Point::new(x as f32, y as f32),
                i as f32 * std::f32::consts::FRAC_PI_8,
                colors[i % colors.len()],
            );

            i += 1;
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("triangles.png").unwrap();
}
