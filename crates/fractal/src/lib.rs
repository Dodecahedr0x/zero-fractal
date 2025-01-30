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

pub fn create_fractal_pixels(size: u64, x: u64, y: u64, zoom: u64) -> Vec<[u8; 3]> {
    let imgx = size as usize;
    let imgy = size as usize;

    let max_iter = 255;
    let min_x = -2.0;
    let max_x = 1.0;
    let min_y = -1.5;
    let max_y = 1.5;
    let min_zoom: f64 = 0.00001;
    let max_zoom: f64 = 1.0;

    let raw_center_x: u64 = x;
    let raw_center_y: u64 = y;
    let raw_zoom: u64 = zoom;

    let center_x = (raw_center_x as f64) / 2.0_f64.powi(64) * (max_x - min_x) + min_x;
    let center_y = (raw_center_y as f64) / 2.0_f64.powi(64) * (max_y - min_y) + min_y;
    let zoom = 10_f64.powf(
        (raw_zoom as f64) / 2_f64.powi(64) * (max_zoom.log10() - min_zoom.log10())
            + min_zoom.log10(),
    );

    let mut img = Vec::with_capacity(imgx * imgy);

    for y in 0..imgy {
        for x in 0..imgx {
            let cx = (x as f64 / imgx as f64 - 0.5) * (max_x - min_x) * zoom + center_x;
            let cy = (y as f64 / imgy as f64 - 0.5) * (max_y - min_y) * zoom + center_y;
            let c = (cx, cy);

            let m = mandelbrot(c, max_iter);

            img.push(get_color(m, max_iter));
        }
    }

    img
}
