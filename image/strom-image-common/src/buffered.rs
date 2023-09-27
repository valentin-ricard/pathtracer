use crate::ImageSink;

/// A helper struct for ImageSink.
///
/// The objective is to make as easy as possible to support ordered images (currently only PPM)
/// without having a lot of duplicate code.
///
/// It works by having an internal buffer allocated, and when the close() function is called,
/// the entire buffer will be flushed down to the underlying implementation.
pub struct OrderedImageSink<PixelType, WrappedSink: ImageSink<PixelType>> {
    size: (usize, usize),
    buffer: Vec<PixelType>,
    inner: WrappedSink
}

impl <PixelType, WrappedSink: ImageSink<PixelType>> ImageSink<PixelType> for OrderedImageSink<PixelType, WrappedSink> {
    fn add_pixel(&mut self, (x, y): (usize, usize), value: PixelType) {
        let flat_coordinates = (self.size.0 - x) * (self.size.1) + y;
        self.buffer[flat_coordinates] = value;
    }

    fn close(self) {
        let mut this = self;
        for (i, item) in this.buffer.into_iter().enumerate() {
            let row = this.size.0 - (i / this.size.1);
            let column = i % this.size.1;
            this.inner.add_pixel((row, column), item);
        }

        this.inner.close();
    }
}

