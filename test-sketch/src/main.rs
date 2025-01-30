use fractal::create_fractal_pixels;
use image::{ImageBuffer, Rgb};

fn main() {
    // Using raw u64 to match what will be passed on-chain
    let size = 1024;
    let raw_center_x: u64 = 7714059436743860000; // (-0.74546 - min_x) / (max_x - min_x) * 2^64
    let raw_center_y: u64 = 9940888892175165000; // (0.11669 - min_y) / (max_y - min_y) * 2^64
    let raw_zoom: u64 = 11458566504308191000; // (log(0.01276) - log(min_zoom)) / (log(max_zoom) - log(min_zoom)) * 2^64
    let pixels = create_fractal_pixels(size, raw_center_x, raw_center_y, raw_zoom);

    let mut imgbuf = ImageBuffer::new(size as u32, size as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgb(pixels[(y as usize) * (size as usize) + (x as usize)]);
    }

    imgbuf.save("../target/mandelbrot.png").unwrap();
}
