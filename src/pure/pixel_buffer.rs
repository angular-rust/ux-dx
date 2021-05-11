use crate::Object;

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct PixelBuffer(Object<ffi::CoglPixelBuffer, PixelBufferClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_pixel_buffer_get_gtype(),
    }
}

impl PixelBuffer {
    //pub fn new(context: &Context, size: usize, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> PixelBuffer {
    //    unsafe { TODO: call cogl_sys:cogl_pixel_buffer_new() }
    //}
}

impl fmt::Display for PixelBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PixelBuffer")
    }
}
