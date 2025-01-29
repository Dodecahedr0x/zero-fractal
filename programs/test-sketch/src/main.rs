use image::{ImageBuffer, Rgb};

fn mandelbrot(c: (f64, f64), max_iter: u32) -> u32 {
    let mut z = (0.0, 0.0);
    let mut n = 0;

    while n < max_iter {
        let (zr, zi) = z;
        z = (zr * zr - zi * zi + c.0, 2.0 * zr * zi + c.1);

        if (z.0 * z.0 + z.1 * z.1).sqrt() > 2.0 {
            break;
        }

        n += 1;
    }

    n
}

/// Nice coloring.
/// Taken from: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
fn get_color(iter: u32, max_iter: u32) -> [u8; 3] {
    if iter < max_iter && iter > 0 {
        let i = iter % 16;
        match i {
            1 => [25, 7, 26],
            2 => [9, 1, 47],
            3 => [4, 4, 73],
            4 => [0, 7, 100],
            5 => [12, 44, 138],
            6 => [24, 82, 177],
            7 => [57, 125, 209],
            8 => [134, 181, 229],
            9 => [211, 236, 248],
            10 => [241, 233, 191],
            11 => [248, 201, 95],
            12 => [255, 170, 0],
            13 => [204, 128, 0],
            14 => [153, 87, 0],
            15 => [106, 52, 3],
            _ => [66, 30, 15],
        }
    } else {
        [0, 0, 0]
    }
}

fn main() {
    let imgx = 18;
    let imgy = 18;
    let max_iter = 255;
    let min_x = -2.0;
    let max_x = 1.0;
    let min_y = -1.5;
    let max_y = 1.5;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = x as f64 / imgx as f64 * (max_x - min_x) + min_x;
        let cy = y as f64 / imgy as f64 * (max_y - min_y) + min_y;
        let c = (cx, cy);

        let m = mandelbrot(c, max_iter);

        *pixel = Rgb(get_color(m, max_iter));
    }

    imgbuf.save("../../target/mandelbrot.png").unwrap();
}
