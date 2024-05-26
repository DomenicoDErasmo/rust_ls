/// Holds the RGB values of some color.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Color {
    /// The RGB to output
    pub rgb: [u8; 3],
}

impl Color {
    /// Makes the color blue.
    #[inline]
    #[must_use]
    pub const fn blue() -> Self {
        Self {
            rgb: [u8::MIN, u8::MIN, u8::MAX],
        }
    }
    /// Makes the color white.
    #[inline]
    #[must_use]
    pub const fn white() -> Self {
        Self {
            rgb: [u8::MAX, u8::MAX, u8::MAX],
        }
    }
}
