use crate::{Context, Object};
use std::fmt;

pub struct IndexBuffer {
    // Buffer _parent;
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
        // IndexBuffer *indices = g_slice_new (IndexBuffer);

        // /* parent's constructor */
        // _buffer_initialize (COGL_BUFFER (indices),
        //                         context,
        //                         bytes,
        //                         COGL_BUFFER_BIND_TARGET_INDEX_BUFFER,
        //                         COGL_BUFFER_USAGE_HINT_INDEX_BUFFER,
        //                         COGL_BUFFER_UPDATE_HINT_STATIC);

        // return _index_buffer_object_new (indices);
        unimplemented!()
    }
}

impl fmt::Display for IndexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndexBuffer")
    }
}
