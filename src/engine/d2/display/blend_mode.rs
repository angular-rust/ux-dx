/// Blend mode used to composite a sprite.
///  *
/// See [Wikipedia](https://en.wikipedia.org/wiki/Blend_modes) for more info.

#[derive(Clone, Copy, Debug)]
pub enum BlendMode {
    /// Blends the source color on top of the destination, respecting transparency.
    ///  *
    Normal,

    /// Adds the source and destination colors, lightening the final image.
    ///  *
    Add,

    /// Multiplies the source and destination colors, darkening the final image.
    ///  *
    Multiply,

    /// Inverts and multiplies the source and destination colors, lightening the final image.
    ///  *
    Screen,

    /// Masks the overlapping area by applying the source alpha to the destination image.
    ///  *
    /// __WARNING__: In HTML5 canvas, this blend mode is unbounded. It will clear the entire
    /// destination image, not just the bounds within the source image.
    ///  *
    Mask,

    /// Ignores the destination color, and copies the source without handling transparency.
    ///  *
    /// __WARNING__: In HTML5 canvas, this blend mode is unbounded. It will clear the entire
    /// destination image, not just the bounds within the source image.
    ///  *
    Copy,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}
