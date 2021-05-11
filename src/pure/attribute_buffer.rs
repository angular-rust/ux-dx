use crate::{Context, Object};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct AttributeBuffer(Object<ffi::CoglAttributeBuffer, AttributeBufferClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_attribute_buffer_get_gtype(),
    }
}

impl AttributeBuffer {
    //pub fn new(context: &Context, data: /*Unimplemented*/&[&Fundamental: Pointer]) -> AttributeBuffer {
    //    unsafe { TODO: call cogl_sys:cogl_attribute_buffer_new() }
    //}

    pub fn with_size(context: &Context, bytes: usize) -> AttributeBuffer {
        unsafe {
            from_glib_full(ffi::cogl_attribute_buffer_new_with_size(
                context.to_glib_none().0,
                bytes,
            ))
        }
    }
}

impl fmt::Display for AttributeBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AttributeBuffer")
    }
}
