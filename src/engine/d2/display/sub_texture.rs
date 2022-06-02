use super::Texture;

/// A sub-region of a texture atlas, created by `Texture.subTexture`.
pub trait SubTexture: Texture {
    /// The original texture that this sub-texture is a part of.
    fn parent(&self) -> Option<Box<dyn Texture>>;

    /// The X offset into the parent texture, in pixels.
    fn x(&self) -> i32;

    /// The Y offset into the parent texture, in pixels.
    fn y(&self) -> i32;
}
