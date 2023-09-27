use crate::rgb::scaled::RgbScaleColor;

/// [RgbNormalizedColor] is a color component.
///
/// The values are integer constrained between 0 and 255
/// for each component of the color (Red, Green and Blue).
pub struct RgbNormalizedColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl RgbNormalizedColor {
    pub fn normalize(color: &RgbScaleColor) -> Self {
        RgbNormalizedColor {
            red: (255.999 * color.red) as u8,
            green: (255.999 * color.green) as u8,
            blue: (255.999 * color.blue) as u8,
        }
    }

    pub fn ppm(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}