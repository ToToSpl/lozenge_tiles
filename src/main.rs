use image;

#[derive(Clone, Copy, Debug)]
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

    let min_x = f32::floor(a.x.min(f32::min(b.x, c.x))) as u32;
    let max_x = f32::ceil(a.x.max(f32::max(b.x, c.x))) as u32;
    let min_y = f32::floor(a.y.min(f32::min(b.y, c.y))) as u32;
    let max_y = f32::ceil(a.y.max(f32::max(b.y, c.y))) as u32;

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

#[derive(Clone, Copy, Debug)]
struct GridTriangle {
    x: usize,
    y: usize,
    triangle_height: f32,
    start_point: Point,
}

impl GridTriangle {
    fn new(x: usize, y: usize, triangle_height: f32, start_point: Point) -> GridTriangle {
        GridTriangle {
            x,
            y,
            triangle_height,
            start_point,
        }
    }

    fn draw(
        self,
        buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        x: usize,
        y: usize,
        color: image::Rgb<u8>,
    ) {
        if x >= self.x || y >= self.y {
            panic!("Tried to draw triangle at {:}, {:}", x, y);
        }

        let mut angle = if y % 2 == 0 {
            std::f32::consts::FRAC_PI_6
        } else {
            -std::f32::consts::FRAC_PI_6
        };

        let mut x_coord = self.start_point.x;
        if y % 2 == 1 {
            x_coord -= 1.0 / 3.0 * self.triangle_height;
        }
        x_coord += ((x / 2) as f32) * 2.0 * self.triangle_height;
        if x % 2 == 1 {
            if y % 2 == 0 {
                x_coord += (2.0 / 3.0) * self.triangle_height;
            } else {
                x_coord += (4.0 / 3.0) * self.triangle_height;
            }
            angle += std::f32::consts::PI;
        }

        let y_coord =
            self.start_point.y + (y as f32) * (f32::sqrt(3.0) / 3.0) * self.triangle_height;

        draw_triangle_equilateral(
            buf,
            self.triangle_height,
            Point::new(x_coord as f32, y_coord as f32),
            angle,
            color,
        );
    }
}

fn main() {
    let imgx = 800 as usize;
    let imgy = 800 as usize;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Iterate over the coordinates and pixels of the image
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    let colors = vec![
        image::Rgb([105, 154, 225]),
        image::Rgb([225, 117, 46]),
        image::Rgb([114, 225, 105]),
    ];

    let grid = GridTriangle::new(13, 22, 50.0, Point::new(100.0, 100.0));
    for y in 0..grid.y {
        for x in 0..grid.x {
            grid.draw(&mut imgbuf, x, y, colors[(x + y) % colors.len()]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("triangles.png").unwrap();
}
