use bytes::Bytes;

use crate::engine::d2::{
    asset::AssetFormat,
    display::{Graphics, Texture},
    util::Value,
};

/// Functions related to the device's renderer.
pub trait RendererSystem {
    /// The type of this renderer.
    fn render_type(&self) -> RendererType;

    /// The maximum width and height of a Texture on this renderer, in pixels. Guaranteed to be at
    /// least 1024.
    fn max_texture_size(&self) -> i32;

    /// Whether the renderer currently has a GPU context. In some renderers (Stage3D and WebGL) the
    /// GPU and all its resources may be destroyed at any time by the system. On renderers that don't
    /// need to worry about reclaiming GPU resources (Canvas) this is always true.
    ///  *
    /// When this becomes false, all Textures and Graphics objects are destroyed and become invalid.
    /// When it returns to true, apps should reload its textures.
    fn has_gpu(&self) -> Value<bool>;

    /// Creates a new blank texture, initialized to transparent black.
    ///  *
    /// @param width The width of the texture, in pixels.
    /// @param height The height of the texture, in pixels.
    ///  *
    /// @returns The new texture, or None if the GPU context is currently unavailable.
    fn create_texture(&self, width: i32, height: i32) -> Box<dyn Texture>;

    // /// Creates a new texture from native image data. Normally you should use
    // /// `System::loadAssetPack()` to load textures, but this can be useful for working with external
    // /// code that deals with native images.
    // ///  *
    // /// @param image The platform-specific image data. In Flash, this is a BitmapData. In HTML, this
    // /// is an ImageElement, CanvasElement, or VideoElement.
    // ///  *
    // /// @returns The new texture, or None if the GPU context is currently unavailable.
    // fn createTextureFromImage(&self, image: NativeImage) -> Box<dyn Texture>;

    // fn createBuffer (&self, size: i32) ->Buffer;
    // fn createShader (&self, glsl :String) ->Shader;

    fn graphics(&self) -> Box<dyn Graphics>;

    /// The compressed texture formats supported by this renderer.
    fn compressed_texture_formats(&self) -> Vec<AssetFormat>;

    fn create_compressed_texture(&self, format: AssetFormat, data: Bytes) -> Box<dyn Texture>;

    /// Notifies the renderer that things are about to be drawn.
    fn will_render(&self);

    /// Notifies the renderer that drawing the frame is complete.
    fn did_render(&self);
}

pub enum RendererType {
    Stage3D,
    WebGL,
    Canvas,
}
