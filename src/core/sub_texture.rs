#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]
use super::{Context, Texture};
use crate::prelude::*;
use std::fmt;

// SECTION:sub-texture
// @short_description: Functions for creating and manipulating
//                     sub-textures.
//
// These functions allow high-level textures to be created that
// represent a sub-region of another texture. For example these
// can be used to implement custom texture atlasing schemes.

pub struct SubTexture {
    // Texture _parent;

// This is the texture that was passed in to
//     _sub_texture_new. If this is also a sub texture then we will
//     use the full texture from that to render instead of making a
//     chain. However we want to preserve the next texture in case the
//     user is expecting us to keep a reference and also so that we can
//     later add a sub_texture_get_parent_texture() function. */
// Texture *next_texture;
// This is the texture that will actually be used to draw. It will
//     point to the end of the chain if a sub texture of a sub texture
//     is created */
// Texture *full_texture;

// The offset of the region represented by this sub-texture. This is
// the offset in full_texture which won't necessarily be the same as
// the offset passed to _sub_texture_new if next_texture is
// actually already a sub texture */
// int sub_x;
// int sub_y;
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
    pub fn new<P: Is<Texture>>(
        ctx: &Context,
        parent_texture: &P,
        sub_x: i32,
        sub_y: i32,
        sub_width: i32,
        sub_height: i32,
    ) -> SubTexture {
        // Texture    *full_texture;
        // SubTexture *sub_tex;
        // Texture    *tex;
        // unsigned int    next_width, next_height;

        // next_width = texture_get_width (next_texture);
        // next_height = texture_get_height (next_texture);

        // The region must specify a non-zero subset of the full texture */
        // _RETURN_VAL_IF_FAIL (sub_x >= 0 && sub_y >= 0, NULL);
        // _RETURN_VAL_IF_FAIL (sub_width > 0 && sub_height > 0, NULL);
        // _RETURN_VAL_IF_FAIL (sub_x + sub_width <= next_width, NULL);
        // _RETURN_VAL_IF_FAIL (sub_y + sub_height <= next_height, NULL);

        // sub_tex = g_new (SubTexture, 1);

        // tex = TEXTURE (sub_tex);

        // _texture_init (tex, ctx, sub_width, sub_height,
        //                     _texture_get_format (next_texture),
        //                     NULL, /* no loader */
        //                     &sub_texture_vtable);

        // If the next texture is also a sub texture we can avoid one level
        //     of indirection by referencing the full texture of that texture
        //     instead. */
        // if (is_sub_texture (next_texture))
        //     {
        //     SubTexture *other_sub_tex = SUB_TEXTURE (next_texture);
        //     full_texture = other_sub_tex->full_texture;
        //     sub_x += other_sub_tex->sub_x;
        //     sub_y += other_sub_tex->sub_y;
        //     }
        // else
        //     full_texture = next_texture;

        // sub_tex->next_texture = object_ref (next_texture);
        // sub_tex->full_texture = object_ref (full_texture);

        // sub_tex->sub_x = sub_x;
        // sub_tex->sub_y = sub_y;

        // return _sub_texture_object_new (sub_tex);
        unimplemented!()
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
        // return sub_texture->next_texture;
        unimplemented!()
    }
}

impl fmt::Display for SubTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubTexture")
    }
}
