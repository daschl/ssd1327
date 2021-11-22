//! Display size.

/// Workaround trait, since `Default` is only implemented to arrays up to 32 of size
pub trait NewZeroed {
    /// Creates a new value with its memory set to zero
    fn new_zeroed() -> Self;
}

impl<const N: usize> NewZeroed for [u8; N] {
    fn new_zeroed() -> Self {
        [0u8; N]
    }
}

/// Display Size and Configuration
///
/// This trait allows implementing various displays
/// with different resolutions and other properties.
pub trait DisplaySize {
    /// Width in pixels
    const WIDTH: u8;
    /// Height in pixels
    const HEIGHT: u8;
    /// Size of framebuffer. Because the display is 4-bit grayscale, this is width * height * 4 / 8
    type Buffer: AsMut<[u8]> + NewZeroed;
}

/// Size information for the 128x128 display
pub struct DisplaySize128x128;
impl DisplaySize for DisplaySize128x128 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 128;
    type Buffer = [u8; Self::WIDTH as usize * Self::HEIGHT as usize * 4 / 8];
}
