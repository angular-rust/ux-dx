use bytes::Bytes;
use std::{fmt, rc::Rc};

use crate::engine::d2::asset::Asset;

use super::{Graphics, SubTexture};
/// A loaded texture image.
pub trait Texture: Asset + fmt::Debug {
    /// The width of this texture, in pixels.
    fn width(&self) -> i32;

    /// The height of this texture, in pixels.
    fn height(&self) -> i32;

    /// The Graphics that draws to this texture.
    ///
    fn graphics(&self) -> Box<dyn Graphics>;

    /// Reads pixels out from the given region. This is potentially a very SLOW operation, avoid
    /// overusing it.
    ///  *
    /// @returns A byte buffer in RGBA order.
    fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32) -> Bytes;

    /// Writes pixels at a given position. sourceW/H is the width and height of the given byte
    /// buffer. This is potentially a very SLOW operation, avoid overusing it.
    /// 
    /// @param pixels A byte buffer in RGBA order.
    fn write_pixels(&self, pixels: Bytes, x: i32, y: i32, source_w: i32, source_h: i32);

    /// Creates a SubTexture that displays a region of this texture.
    ///  *
    /// The returned sub-texture is only a "view", so any changes to the parent texture will affect
    /// its regions. Repeatedly nested sub-textures are allowed.
    ///  *
    /// NOTE: The `graphics` instance of the sub-texture is the same as its parent. This means you
    /// may need to `translate()` first when working with sub-textures, and take care not to conflict
    /// with other textures' `graphics`.
    ///  *
    /// @param x The X offset of the region.
    /// @param y The Y offset of the region.
    /// @param width The width of the region.
    /// @param height The height of the region.
    fn sub_texture(&self, x: i32, y: i32, width: i32, height: i32) -> Rc<dyn SubTexture>;

    /// Splits this texture into multiple tiles using `sub_texture()`.
    ///  *
    /// @param tilesWide The width, in number of tiles.
    /// @param tilesHigh The height, in number of tiles.
    //  tilesHigh: i32 = 1
    fn split(&self, tiles_wide: i32, tiles_high: i32) -> Vec<Rc<dyn SubTexture>>;
}
