use crate::{Bitmap, Context, Object, PixelFormat};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct AtlasTexture(Object<ffi::CoglAtlasTexture, AtlasTextureClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_atlas_texture_get_gtype(),
    }
}

impl AtlasTexture {
    pub fn from_bitmap(bmp: &Bitmap) -> AtlasTexture {
        unsafe {
            from_glib_full(ffi::cogl_atlas_texture_new_from_bitmap(
                bmp.to_glib_none().0,
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
    ) -> Result<AtlasTexture, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_atlas_texture_new_from_data(
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

    pub fn from_file(ctx: &Context, filename: &str) -> Result<AtlasTexture, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_atlas_texture_new_from_file(
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

    pub fn with_size(ctx: &Context, width: i32, height: i32) -> AtlasTexture {
        unsafe {
            from_glib_full(ffi::cogl_atlas_texture_new_with_size(
                ctx.to_glib_none().0,
                width,
                height,
            ))
        }
    }
}

impl fmt::Display for AtlasTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AtlasTexture")
    }
}
