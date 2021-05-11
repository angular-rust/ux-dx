use crate::{Bitmap, Context, Object, PixelFormat, Texture};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Texture2D(Object<ffi::CoglTexture2D, Texture2DClass>) @extends Object, @implements Texture;

    match fn {
        get_type => || ffi::cogl_texture_2d_get_gtype(),
    }
}

impl Texture2D {
    /// Wraps an existing GL_TEXTURE_2D texture object as a `Texture2D`.
    /// This can be used for integrating Cogl with software using OpenGL
    /// directly.
    ///
    /// The texture is still configurable until it has been allocated so
    /// for example you can declare whether the texture is premultiplied
    /// with `Texture::set_premultiplied`.
    ///
    /// `<note>`The results are undefined for passing an invalid `gl_handle`
    /// or if `width` or `height` don't have the correct texture
    /// geometry.`</note>`
    ///
    /// ## `ctx`
    /// A `Context`
    /// ## `gl_handle`
    /// A GL handle for a GL_TEXTURE_2D texture object
    /// ## `width`
    /// Width of the foreign GL texture
    /// ## `height`
    /// Height of the foreign GL texture
    /// ## `format`
    /// The format of the texture
    ///
    /// # Returns
    ///
    /// A newly allocated `Texture2D`
    pub fn gl_new_from_foreign(
        ctx: &Context,
        gl_handle: u32,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> Texture2D {
        unsafe {
            from_glib_full(ffi::cogl_texture_2d_gl_new_from_foreign(
                ctx.to_glib_none().0,
                gl_handle,
                width,
                height,
                format.to_glib(),
            ))
        }
    }

    pub fn from_bitmap(bitmap: &Bitmap) -> Texture2D {
        unsafe {
            from_glib_full(ffi::cogl_texture_2d_new_from_bitmap(
                bitmap.to_glib_none().0,
            ))
        }
    }

    pub fn from_data(
        ctx: &Context,
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> Result<Texture2D, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_2d_new_from_data(
                ctx.to_glib_none().0,
                width,
                height,
                format.to_glib(),
                rowstride,
                data.as_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_file(ctx: &Context, filename: &str) -> Result<Texture2D, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_2d_new_from_file(
                ctx.to_glib_none().0,
                filename.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_size(ctx: &Context, width: i32, height: i32) -> Texture2D {
        unsafe {
            from_glib_full(ffi::cogl_texture_2d_new_with_size(
                ctx.to_glib_none().0,
                width,
                height,
            ))
        }
    }
}

impl fmt::Display for Texture2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2D")
    }
}
