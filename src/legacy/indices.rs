use crate::{IndexBuffer, IndicesType, Object};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Indices(Object<ffi::CoglIndices, IndicesClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_indices_get_gtype(),
    }
}

impl Indices {
    //pub fn new(context: &Context, type_: IndicesType, indices_data: /*Unimplemented*/Option<Fundamental: Pointer>, n_indices: i32) -> Indices {
    //    unsafe { TODO: call cogl_sys:cogl_indices_new() }
    //}

    pub fn new_for_buffer(type_: IndicesType, buffer: &IndexBuffer, offset: usize) -> Indices {
        unsafe {
            from_glib_full(ffi::cogl_indices_new_for_buffer(
                type_.to_glib(),
                buffer.to_glib_none().0,
                offset,
            ))
        }
    }

    pub fn get_buffer(&self) -> Option<IndexBuffer> {
        unsafe { from_glib_none(ffi::cogl_indices_get_buffer(self.to_glib_none().0)) }
    }

    pub fn get_offset(&self) -> usize {
        unsafe { ffi::cogl_indices_get_offset(self.to_glib_none().0) }
    }

    pub fn get_type(&self) -> IndicesType {
        unsafe { from_glib(ffi::cogl_indices_get_type(self.to_glib_none().0)) }
    }

    pub fn set_offset(&self, offset: usize) {
        unsafe {
            ffi::cogl_indices_set_offset(self.to_glib_none().0, offset);
        }
    }
}

impl fmt::Display for Indices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Indices")
    }
}
