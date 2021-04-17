#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Bitmap, Context, Object, PixelFormat, Texture};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Texture3D(Object<ffi::CoglTexture3D, Texture3DClass>) @extends Object, @implements Texture;

    match fn {
        get_type => || ffi::cogl_texture_3d_get_gtype(),
    }
}

impl Texture3D {
    pub fn from_bitmap(bitmap: &Bitmap, height: i32, depth: i32) -> Texture3D {
        unsafe {
            from_glib_full(ffi::cogl_texture_3d_new_from_bitmap(
                bitmap.to_glib_none().0,
                height,
                depth,
            ))
        }
    }

    pub fn from_data(
        context: &Context,
        width: i32,
        height: i32,
        depth: i32,
        format: PixelFormat,
        rowstride: i32,
        image_stride: i32,
        data: &[u8],
    ) -> Result<Texture3D, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_3d_new_from_data(
                context.to_glib_none().0,
                width,
                height,
                depth,
                format.to_glib(),
                rowstride,
                image_stride,
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

    pub fn with_size(context: &Context, width: i32, height: i32, depth: i32) -> Texture3D {
        unsafe {
            from_glib_full(ffi::cogl_texture_3d_new_with_size(
                context.to_glib_none().0,
                width,
                height,
                depth,
            ))
        }
    }
}

impl fmt::Display for Texture3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture3D")
    }
}
