#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Context, TexturePixmapX11ReportLevel};
use std::{fmt, ptr};

pub struct TexturePixmapX11 {}

impl TexturePixmapX11 {
    /// Creates a texture that contains the contents of `pixmap`. If
    /// `automatic_updates` is `true` then  will attempt to listen for
    /// damage events on the pixmap and automatically update the texture
    /// when it changes.
    /// ## `context`
    /// A `Context`
    /// ## `pixmap`
    /// A X11 pixmap ID
    /// ## `automatic_updates`
    /// Whether to automatically copy the contents of
    /// the pixmap to the texture.
    ///
    /// # Returns
    ///
    /// a new `TexturePixmapX11` instance
    pub fn new(context: &Context, pixmap: u32, automatic_updates: bool) -> TexturePixmapX11 {
        // let ret = ffi::texture_pixmap_x11_new(
        unimplemented!()
    }

    /// Creates one of a pair of textures to contain the contents of `pixmap`,
    /// which has stereo content. (Different images for the right and left eyes.)
    /// The left image is drawn using this texture; the right image is drawn
    /// using a texture created by calling
    /// `TexturePixmapX11::new_right` and passing in this texture as an
    /// argument.
    ///
    /// In general, you should not use this fn unless you have
    /// queried the `GLX_STEREO_TREE_EXT` attribute of the corresponding
    /// window using glXQueryDrawable() and determined that the window is
    /// stereo. Note that this attribute can change over time and
    /// notification is also provided through events defined in the
    /// EXT_stereo_tree GLX extension. As long as the system has support for
    /// stereo content, drawing using the left and right pixmaps will not
    /// produce an error even if the window doesn't have stereo
    /// content any more, but drawing with the right pixmap will produce
    /// undefined output, so you need to listen for these events and
    /// re-render to avoid race conditions. (Recreating a non-stereo
    /// pixmap is not necessary, but may save resources.)
    /// ## `context`
    /// A `Context`
    /// ## `pixmap`
    /// A X11 pixmap ID
    /// ## `automatic_updates`
    /// Whether to automatically copy the contents of
    /// the pixmap to the texture.
    ///
    /// # Returns
    ///
    /// a new `TexturePixmapX11` instance
    pub fn new_left(context: &Context, pixmap: u32, automatic_updates: bool) -> TexturePixmapX11 {
        // ffi::texture_pixmap_x11_new_left
        unimplemented!()
    }

    /// Checks whether the given `self` is using the
    /// GLX_EXT_texture_from_pixmap or similar extension to copy the
    /// contents of the pixmap to the texture. This extension is usually
    /// implemented as zero-copy operation so it implies the updates are
    /// working efficiently.
    ///
    /// # Returns
    ///
    /// `true` if the texture is using an efficient extension
    ///  and `false` otherwise
    pub fn is_using_tfp_extension(&self) -> bool {
        // ffi::texture_pixmap_x11_is_using_tfp_extension
        unimplemented!()
    }

    /// Creates a texture object that corresponds to the right-eye image
    /// of a pixmap with stereo content. `self` must have been
    /// created using `TexturePixmapX11::new_left`.
    ///
    /// # Returns
    ///
    /// a new `TexturePixmapX11` instance
    pub fn new_right(&self) -> Option<TexturePixmapX11> {
        // ffi::texture_pixmap_x11_new_right
        unimplemented!()
    }

    /// Sets the damage object that will be used to track automatic updates
    /// to the `self`. Damage tracking can be disabled by passing 0 for
    /// `damage`. Otherwise this damage will replace the one used if `true`
    /// was passed for automatic_updates to `TexturePixmapX11::new`.
    ///
    /// Note that  will subtract from the damage region as it processes
    /// damage events.
    /// ## `damage`
    /// A X11 Damage object or 0
    /// ## `report_level`
    /// The report level which describes how to interpret
    ///  the damage events. This should match the level that the damage
    ///  object was created with.
    pub fn set_damage_object(&self, damage: u32, report_level: TexturePixmapX11ReportLevel) {
        // ffi::texture_pixmap_x11_set_damage_object
        unimplemented!()
    }

    /// Forces an update of the given `self` so that it is refreshed with
    /// the contents of the pixmap that was given to
    /// `TexturePixmapX11::new`.
    /// ## `x`
    /// x coordinate of the area to update
    /// ## `y`
    /// y coordinate of the area to update
    /// ## `width`
    /// width of the area to update
    /// ## `height`
    /// height of the area to update
    pub fn update_area(&self, x: i32, y: i32, width: i32, height: i32) {
        // ffi::texture_pixmap_x11_update_area
        unimplemented!()
    }

    pub fn error_quark() -> u32 {
        // unsafe { ffi::texture_pixmap_x11_error_quark() }
        unimplemented!()
    }
}

impl fmt::Display for TexturePixmapX11 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TexturePixmapX11")
    }
}
