#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Bitmap, Context, Object, PixelFormat, Texture};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Texture2DSliced(Object<ffi::CoglTexture2DSliced, Texture2DSlicedClass>) @extends Object, @implements Texture;

    match fn {
        get_type => || ffi::cogl_texture_2d_sliced_get_gtype(),
    }
}

impl Texture2DSliced {
    pub fn from_bitmap(bmp: &Bitmap, max_waste: i32) -> Texture2DSliced {
        unsafe {
            from_glib_full(ffi::cogl_texture_2d_sliced_new_from_bitmap(
                bmp.to_glib_none().0,
                max_waste,
            ))
        }
    }

    pub fn from_data(
        ctx: &Context,
        width: i32,
        height: i32,
        max_waste: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> Result<Texture2DSliced, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_2d_sliced_new_from_data(
                ctx.to_glib_none().0,
                width,
                height,
                max_waste,
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

    pub fn from_file(
        ctx: &Context,
        filename: &str,
        max_waste: i32,
    ) -> Result<Texture2DSliced, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_2d_sliced_new_from_file(
                ctx.to_glib_none().0,
                filename.to_glib_none().0,
                max_waste,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_size(ctx: &Context, width: i32, height: i32, max_waste: i32) -> Texture2DSliced {
        unsafe {
            from_glib_full(ffi::cogl_texture_2d_sliced_new_with_size(
                ctx.to_glib_none().0,
                width,
                height,
                max_waste,
            ))
        }
    }
}

impl fmt::Display for Texture2DSliced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2DSliced")
    }
}
