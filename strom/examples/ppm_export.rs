use strom_progress::{generate_image, PixelResult};

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Print the header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    generate_image(IMAGE_WIDTH, IMAGE_HEIGHT, |row, column| {
        let r = row as f32 / (IMAGE_WIDTH as f32 - 1f32);
        let g = column as f32 / (IMAGE_HEIGHT as f32 - 1f32);
        let b = 0.25f32;

        let ir = (255.999 * r) as usize;
        let ig = (255.999 * g) as usize;
        let ib = (255.999 * b) as usize;



        println!("{} {} {}", ir, ig, ib);

        PixelResult::None
    });
}
