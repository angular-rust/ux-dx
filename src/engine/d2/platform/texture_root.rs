use bytes::Bytes;

use crate::engine::d2::{
    display::{Graphics, Texture},
    util::Disposable,
};

/// The "root" of a texture atlas. An internal abstraction that makes implementing subTexture() easier
/// across platforms.
pub trait TextureRoot: Disposable {
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn create_texture(&self, width: i32, height: i32) -> Box<dyn Texture>; // BasicTexture<R>

    fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32) -> Bytes;
    fn write_pixels(&self, pixels: Bytes, x: i32, y: i32, source_w: i32, source_h: i32);

    fn graphics(&self) -> Box<dyn Graphics>;
}
