use crate::{Context, Object};
use std::fmt;

pub struct PixelBuffer {
    // CoglBuffer            _parent;
}

// static CoglPixelBuffer *
// _cogl_pixel_buffer_new (CoglContext *context,
//                         size_t size,
//                         const void *data,
//                         CoglError **error)
// {
//   CoglPixelBuffer *pixel_buffer = g_slice_new0 (CoglPixelBuffer);
//   CoglBuffer *buffer = COGL_BUFFER (pixel_buffer);

//   /* parent's constructor */
//   _cogl_buffer_initialize (buffer,
//                            context,
//                            size,
//                            COGL_BUFFER_BIND_TARGET_PIXEL_UNPACK,
//                            COGL_BUFFER_USAGE_HINT_TEXTURE,
//                            COGL_BUFFER_UPDATE_HINT_STATIC);

//   _cogl_pixel_buffer_object_new (pixel_buffer);

//   if (data)
//     {
//       if (!_cogl_buffer_set_data (COGL_BUFFER (pixel_buffer),
//                                   0,
//                                   data,
//                                   size,
//                                   error))
//         {
//           cogl_object_unref (pixel_buffer);
//           return NULL;
//         }
//     }

//   return pixel_buffer;
// }

impl PixelBuffer {
    // * cogl_pixel_buffer_new:
    // * @context: A #CoglContext
    // * @size: The number of bytes to allocate for the pixel data.
    // * @data: An optional pointer to vertex data to upload immediately
    // *
    // * Declares a new #CoglPixelBuffer of @size bytes to contain arrays of
    // * pixels. Once declared, data can be set using cogl_buffer_set_data()
    // * or by mapping it into the application's address space using
    // * cogl_buffer_map().
    // *
    // * If @data isn't %NULL then @size bytes will be read from @data and
    // * immediately copied into the new buffer.
    // *
    // * Return value: (transfer full): a newly allocated #CoglPixelBuffer
    // *
    // * Since: 1.10
    // * Stability: unstable
    pub fn new(context: &Context, size: usize, data: &[u8]) -> PixelBuffer {
        // CoglError *ignore_error = NULL;
        // CoglPixelBuffer *buffer =
        //   _cogl_pixel_buffer_new (context, size, data, &ignore_error);
        // if (!buffer)
        //   cogl_error_free (ignore_error);
        // return buffer;
        unimplemented!()
    }
}

impl fmt::Display for PixelBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PixelBuffer")
    }
}
