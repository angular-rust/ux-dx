use super::Context;
use std::fmt;

pub struct PixelBuffer {
    // Buffer            _parent;
}

// static PixelBuffer *
// _pixel_buffer_new (Context *context,
//                         size_t size,
//                         const void *data,
//                         Error **error)
// {
//   PixelBuffer *pixel_buffer = g_slice_new0 (PixelBuffer);
//   Buffer *buffer = BUFFER (pixel_buffer);

//   /* parent's constructor */
//   _buffer_initialize (buffer,
//                            context,
//                            size,
//                            BUFFER_BIND_TARGET_PIXEL_UNPACK,
//                            BUFFER_USAGE_HINT_TEXTURE,
//                            BUFFER_UPDATE_HINT_STATIC);

//   _pixel_buffer_object_new (pixel_buffer);

//   if (data)
//     {
//       if (!_buffer_set_data (BUFFER (pixel_buffer),
//                                   0,
//                                   data,
//                                   size,
//                                   error))
//         {
//           object_unref (pixel_buffer);
//           return NULL;
//         }
//     }

//   return pixel_buffer;
// }

impl PixelBuffer {
    // pixel_buffer_new:
    // @context: A #Context
    // @size: The number of bytes to allocate for the pixel data.
    // @data: An optional pointer to vertex data to upload immediately
    //
    // Declares a new #PixelBuffer of @size bytes to contain arrays of
    // pixels. Once declared, data can be set using buffer_set_data()
    // or by mapping it into the application's address space using
    // buffer_map().
    //
    // If @data isn't %NULL then @size bytes will be read from @data and
    // immediately copied into the new buffer.
    //
    // Return value: (transfer full): a newly allocated #PixelBuffer
    //
    // Since: 1.10
    // Stability: unstable
    pub fn new(context: &Context, size: usize, data: &[u8]) -> PixelBuffer {
        // Error *ignore_error = NULL;
        // PixelBuffer *buffer =
        //   _pixel_buffer_new (context, size, data, &ignore_error);
        // if (!buffer)
        //   error_free (ignore_error);
        // return buffer;
        unimplemented!()
    }
}

impl fmt::Display for PixelBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PixelBuffer")
    }
}
