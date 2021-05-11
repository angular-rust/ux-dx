#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Context, Object, Texture};

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct SubTexture(Object<ffi::CoglSubTexture, SubTextureClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_sub_texture_get_gtype(),
    }
}

impl SubTexture {
    /// Creates a high-level `SubTexture` representing a sub-region of
    /// any other `Texture`. The sub-region must strictly lye within the
    /// bounds of the `parent_texture`. The returned texture implements the
    /// `MetaTexture` interface because it's not a low level texture
    /// that hardware can understand natively.
    ///
    /// `<note>`Remember: Unless you are using high level drawing APIs such
    /// as `rectangle` or other APIs documented to understand the
    /// `MetaTexture` interface then you need to use the
    /// `MetaTexture` interface to resolve a `SubTexture` into a
    /// low-level texture before drawing.`</note>`
    /// ## `ctx`
    /// A `Context` pointer
    /// ## `parent_texture`
    /// The full texture containing a sub-region you want
    ///  to make a `SubTexture` from.
    /// ## `sub_x`
    /// The top-left x coordinate of the parent region to make
    ///  a texture from.
    /// ## `sub_y`
    /// The top-left y coordinate of the parent region to make
    ///  a texture from.
    /// ## `sub_width`
    /// The width of the parent region to make a texture from.
    /// ## `sub_height`
    /// The height of the parent region to make a texture
    ///  from.
    ///
    /// # Returns
    ///
    /// A newly allocated `SubTexture`
    ///  representing a sub-region of `parent_texture`.
    pub fn new<P: IsA<Texture>>(
        ctx: &Context,
        parent_texture: &P,
        sub_x: i32,
        sub_y: i32,
        sub_width: i32,
        sub_height: i32,
    ) -> SubTexture {
        unsafe {
            from_glib_full(ffi::cogl_sub_texture_new(
                ctx.to_glib_none().0,
                parent_texture.as_ref().to_glib_none().0,
                sub_x,
                sub_y,
                sub_width,
                sub_height,
            ))
        }
    }

    /// Retrieves the parent texture that `self` derives its content
    /// from. This is the texture that was passed to
    /// `SubTexture::new` as the parent_texture argument.
    ///
    /// # Returns
    ///
    /// The parent texture that `self`
    ///  derives its content from.
    pub fn get_parent(&self) -> Option<Texture> {
        unsafe { from_glib_none(ffi::cogl_sub_texture_get_parent(self.to_glib_none().0)) }
    }
}

impl fmt::Display for SubTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubTexture")
    }
}
