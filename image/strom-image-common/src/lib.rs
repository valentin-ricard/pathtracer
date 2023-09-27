pub mod buffered;

pub enum ImageType {
    RGB,
    RGBA,
    GRAYSCALE
}

pub trait ImageSink<PixelType> {
    /// Raw procedure used to set the pixel.
    /// There is no guarantee whatsoever about the order the pixels are considered "finished".
    fn add_pixel(&mut self, coordinates: (usize, usize), value: PixelType);
    /// Called when the computations are finished,
    /// and the image has to be totally saved and closed after this call.
    fn close(self);
}

pub trait ImageSinkSource<PixelType, T> {
    fn initialize(size: (usize, usize), image_type: ImageType) -> Result<T, ()>;
}