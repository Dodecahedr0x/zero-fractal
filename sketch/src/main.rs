#![no_main]
sp1_zkvm::entrypoint!(main);

use fractal::create_fractal_pixels;

pub fn main() {
    // Read an input to the program.
    // TODO: take center and zoom for the image as input
    let [size, center_x, center_y, zoom] = sp1_zkvm::io::read::<[u64; 4]>();

    let imgbuf = create_fractal_pixels(size, center_x, center_y, zoom);

    // Convert the image buffer to a flat byte array
    let bytes: Vec<u8> = imgbuf.iter().flat_map(|&rgb| rgb).collect();

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
