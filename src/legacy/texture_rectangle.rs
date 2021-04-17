#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::Bitmap;
use crate::{Context, Object, PixelFormat, Texture};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TextureRectangle(Object<ffi::CoglTextureRectangle, TextureRectangleClass>) @extends Object, @implements Texture;

    match fn {
        get_type => || ffi::cogl_texture_rectangle_get_gtype(),
    }
}

impl TextureRectangle {
    pub fn from_bitmap(bitmap: &Bitmap) -> TextureRectangle {
        unsafe {
            from_glib_full(ffi::cogl_texture_rectangle_new_from_bitmap(
                bitmap.to_glib_none().0,
            ))
        }
    }

    pub fn from_foreign(
        ctx: &Context,
        gl_handle: u32,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> TextureRectangle {
        unsafe {
            from_glib_full(ffi::cogl_texture_rectangle_new_from_foreign(
                ctx.to_glib_none().0,
                gl_handle,
                width,
                height,
                format.to_glib(),
            ))
        }
    }

    pub fn with_size(ctx: &Context, width: i32, height: i32) -> TextureRectangle {
        unsafe {
            from_glib_full(ffi::cogl_texture_rectangle_new_with_size(
                ctx.to_glib_none().0,
                width,
                height,
            ))
        }
    }
}

impl fmt::Display for TextureRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureRectangle")
    }
}
