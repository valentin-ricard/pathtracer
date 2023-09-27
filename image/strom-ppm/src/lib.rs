use std::fs::File;
use strom_color::rgb::normalized::RgbNormalizedColor;
use strom_image_common::buffered::OrderedImageSink;
use strom_image_common::ImageSink;

pub struct PpmImageSink {
    file_handle: File
}
impl ImageSink<RgbNormalizedColor> for PpmImageSink {
    fn add_pixel(&mut self, coordinates: (usize, usize), value: RgbNormalizedColor) {
        // We are assured that the coordinates are in order, so we can just work our way with it ^^
        
    }

    fn close(self) {
        //Explicitly close the file by dropping the handle to it.
        drop(self.file_handle);
    }
}