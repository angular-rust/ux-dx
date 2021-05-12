use crate::{Context, Object};
use std::fmt;

pub struct AttributeBuffer {
    // CoglBuffer _parent;
}

impl AttributeBuffer {
    // * cogl_attribute_buffer_new:
    // * @context: A #CoglContext
    // * @bytes: The number of bytes to allocate for vertex attribute data.
    // * @data: (array length=bytes): An optional pointer to vertex data to
    // *        upload immediately.
    // *
    // * Describes a new #CoglAttributeBuffer of @size bytes to contain
    // * arrays of vertex attribute data and also uploads @size bytes read
    // * from @data to the new buffer.
    // *
    // * You should never pass a %NULL data pointer.
    // *
    // * <note>This function does not report out-of-memory errors back to
    // * the caller by returning %NULL and so you can assume this function
    // * always succeeds.</note>
    // *
    // * <note>In the unlikely case that there is an out of memory problem
    // * then Cogl will abort the application with a message. If your
    // * application needs to gracefully handle out-of-memory errors then
    // * you can use cogl_attribute_buffer_new_with_size() and then
    // * explicitly catch errors with cogl_buffer_set_data() or
    // * cogl_buffer_map().</note>
    // *
    // * Return value: (transfer full): A newly allocated #CoglAttributeBuffer (never %NULL)
    // *
    // * Since: 1.4
    // * Stability: Unstable
    pub fn new(context: &Context, data: &[u8]) -> AttributeBuffer {
        // CoglAttributeBuffer *buffer;

        // buffer = cogl_attribute_buffer_new_with_size (context, bytes);

        // // Note: to keep the common cases simple this API doesn't throw
        // // CoglErrors, so developers can assume this function never returns
        // // NULL and we will simply abort on error.
        // //
        // // Developers wanting to catch errors can use
        // // cogl_attribute_buffer_new_with_size() and catch errors when later
        // // calling cogl_buffer_set_data() or cogl_buffer_map().

        // // XXX: NB: for Cogl 2.0 we don't allow NULL data here but we can't
        // // break the api for 1.x and so we keep the check for now. */
        // if (data)
        //     _cogl_buffer_set_data (COGL_BUFFER (buffer),
        //                         0,
        //                         data,
        //                         bytes,
        //                         NULL);

        // return buffer;
        unimplemented!()
    }

    // * cogl_attribute_buffer_new_with_size:
    // * @context: A #CoglContext
    // * @bytes: The number of bytes to allocate for vertex attribute data.
    // *
    // * Describes a new #CoglAttributeBuffer of @size bytes to contain
    // * arrays of vertex attribute data. Afterwards data can be set using
    // * cogl_buffer_set_data() or by mapping it into the application's
    // * address space using cogl_buffer_map().
    // *
    // * The underlying storage of this buffer isn't allocated by this
    // * function so that you have an opportunity to use the
    // * cogl_buffer_set_update_hint() and cogl_buffer_set_usage_hint()
    // * functions which may influence how the storage is allocated. The
    // * storage will be allocated once you upload data to the buffer.
    // *
    // * Note: You can assume this function always succeeds and won't return
    // * %NULL
    // *
    // * Return value: (transfer full): A newly allocated #CoglAttributeBuffer. Never %NULL.
    // *
    // * Stability: Unstable
    pub fn with_size(context: &Context, bytes: usize) -> AttributeBuffer {
        // CoglAttributeBuffer *buffer = g_slice_new (CoglAttributeBuffer);

        // // parent's constructor
        // _cogl_buffer_initialize (COGL_BUFFER (buffer),
        //                         context,
        //                         bytes,
        //                         COGL_BUFFER_BIND_TARGET_ATTRIBUTE_BUFFER,
        //                         COGL_BUFFER_USAGE_HINT_ATTRIBUTE_BUFFER,
        //                         COGL_BUFFER_UPDATE_HINT_STATIC);

        // return _cogl_attribute_buffer_object_new (buffer);
        unimplemented!()
    }
}

impl fmt::Display for AttributeBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AttributeBuffer")
    }
}
