use indicatif::ProgressBar;

pub fn generate_image<F>(image_width: usize, image_height: usize, process: F)
where
    F: Fn(usize, usize) -> PixelResult,
{
    let total_pixels = image_width * image_height;
    let progress = ProgressBar::new(total_pixels as u64);
    progress.set_draw_delta((total_pixels / 100) as u64);
    for column in (0..image_height).rev() {
        for row in 0..image_width {
            process(column, row);
            progress.inc(1);
        }
    }
}

pub enum PixelResult {
    Color { r: usize, g: usize, b: usize },
    None,
}
