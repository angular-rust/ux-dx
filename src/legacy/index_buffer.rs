use crate::{Context, Object};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct IndexBuffer(Object<ffi::CoglIndexBuffer, IndexBufferClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_index_buffer_get_gtype(),
    }
}

impl IndexBuffer {
    /// Declares a new `IndexBuffer` of `size` bytes to contain vertex
    /// indices. Once declared, data can be set using
    /// `buffer_set_data` or by mapping it into the application's
    /// address space using `buffer_map`.
    /// ## `context`
    /// A `Context`
    /// ## `bytes`
    /// The number of bytes to allocate for vertex attribute data.
    ///
    /// # Returns
    ///
    /// A newly allocated `IndexBuffer`
    pub fn new(context: &Context, bytes: usize) -> IndexBuffer {
        unsafe { from_glib_full(ffi::cogl_index_buffer_new(context.to_glib_none().0, bytes)) }
    }
}

impl fmt::Display for IndexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndexBuffer")
    }
}
