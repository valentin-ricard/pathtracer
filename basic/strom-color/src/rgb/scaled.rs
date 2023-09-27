use strom_macro::{operator_impl, scalar_ops};
use crate::rgb::normalized::RgbNormalizedColor;

/// [RgbScaleColor] is a color component.
///
/// The values **must** be contained within 0 and 1.
pub struct RgbScaleColor {
    /// The red component of the color
    pub red: f32,
    /// The green component of the color
    pub green: f32,
    /// The blue component of the color
    pub blue: f32
}

impl RgbScaleColor {
    pub fn of(red: f32, green: f32, blue: f32) -> Self {
        RgbScaleColor {
            red,
            green,
            blue
        }
    }

    pub fn to_ppm(&self) -> String {
        RgbNormalizedColor::normalize(&self)
            .ppm()
    }
}

operator_impl!(RgbScaleColor, RgbScaleColor::of, red, green, blue);

scalar_ops!(RgbScaleColor, RgbScaleColor::of, f32, f32, red, green, blue);