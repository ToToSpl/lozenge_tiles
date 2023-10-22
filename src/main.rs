use image;

#[derive(Clone)]
struct Complex {
    rel: f32,
    img: f32,
}

impl Complex {
    fn new(rel: f32, img: f32) -> Complex {
        Self { rel, img }
    }

    fn norm(&self) -> f32 {
        f32::sqrt(self.rel * self.rel + self.img * self.img)
    }

    fn mul(a: &Complex, b: &Complex) -> Complex {
        Complex {
            rel: (a.rel * b.rel) - (a.img * b.img),
            img: (a.rel * b.img) + (a.img * b.rel),
        }
    }

    fn add(a: &Complex, b: &Complex) -> Complex {
        Complex {
            rel: a.rel + b.rel,
            img: a.img + b.img,
        }
    }
}

fn main() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

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
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = Complex::new(-0.4, 0.6);
            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = Complex::add(&Complex::mul(&z, &z), &c);
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
