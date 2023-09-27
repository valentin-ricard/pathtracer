use strom_color::rgb::scaled::RgbScaleColor;
use strom_progress::{generate_image, PixelResult};

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Print the header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    generate_image(IMAGE_WIDTH, IMAGE_HEIGHT, |row, column| {

        println!("{}", RgbScaleColor::of(
            row / (IMAGE_WIDTH as f32 - 1.0),
            column / (IMAGE_HEIGHT as f32 - 1.0),
            0.25f32
        ).ppm());

        PixelResult::None
    });
}
