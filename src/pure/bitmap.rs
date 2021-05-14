use super::{Context, PixelBuffer, PixelFormat};
use bytes::Bytes;
use std::fmt;

// * @short_description: Functions for loading images
// *
// *  allows loading image data into memory as Bitmaps without
// * loading them immediately into GPU textures.
#[derive(Debug)]
pub struct Bitmap {
    // Object _parent;

// /* Pointer back to the context that this bitmap was created with */
// Context *context;

// PixelFormat format;
// int width;
// int height;
// int rowstride;

// uint8_t *data;

// Bool mapped;
// Bool bound;

// /* If this is non-null then 'data' is ignored and instead it is
//    fetched from this shared bitmap. */
// Bitmap *shared_bmp;

// /* If this is non-null then 'data' is treated as an offset into the
//    buffer and map will divert to mapping the buffer */
// Buffer *buffer;
}

impl Bitmap {
    /// Creates a bitmap using some existing data. The data is not copied
    /// so the application must keep the buffer alive for the lifetime of
    /// the `Bitmap`. This can be used for example with
    /// `Framebuffer::read_pixels_into_bitmap` to read data directly
    /// into an application buffer with the specified rowstride.
    /// ## `context`
    /// A `Context`
    /// ## `width`
    /// The width of the bitmap.
    /// ## `height`
    /// The height of the bitmap.
    /// ## `format`
    /// The format of the pixel data.
    /// ## `rowstride`
    /// The rowstride of the bitmap (the number of bytes from
    ///  the start of one row of the bitmap to the next).
    /// ## `data`
    /// A pointer to the data. The bitmap will take ownership of this data.
    ///
    /// # Returns
    ///
    /// A new `Bitmap`.
    pub fn new_for_data(
        context: &Context,
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> Bitmap {
        // Bitmap *bmp;

        // g_return_val_if_fail (is_context (context), NULL);

        // /* Rowstride from width if not given */
        // if (rowstride == 0)
        //     rowstride = width * _pixel_format_get_bytes_per_pixel (format);

        // bmp = g_slice_new (Bitmap);
        // bmp->context = context;
        // bmp->format = format;
        // bmp->width = width;
        // bmp->height = height;
        // bmp->rowstride = rowstride;
        // bmp->data = data;
        // bmp->mapped = false;
        // bmp->bound = false;
        // bmp->shared_bmp = NULL;
        // bmp->buffer = NULL;

        // return _bitmap_object_new (bmp);
        unimplemented!()
    }

    // * bitmap_new_from_buffer:
    // * @buffer: A #Buffer containing image data
    // * @format: The #PixelFormat defining the format of the image data
    // *          in the given @buffer.
    // * @width: The width of the image data in the given @buffer.
    // * @height: The height of the image data in the given @buffer.
    // * @rowstride: The rowstride in bytes of the image data in the given @buffer.
    // * @offset: The offset into the given @buffer to the first pixel that
    // *          should be considered part of the #Bitmap.
    // *
    // * Wraps some image data that has been uploaded into a #Buffer as
    // * a #Bitmap. The data is not copied in this process.
    // *
    // * Return value: (transfer full): a #Bitmap encapsulating the given @buffer.
    // *
    // * Since: 1.8
    // * Stability: unstable
    pub fn from_buffer(
        buffer: Bytes,
        format: PixelFormat,
        width: i32,
        height: i32,
        rowstride: i32,
        offset: i32,
    ) -> Bitmap {
        // Bitmap *bmp;

        // _COGL_RETURN_VAL_IF_FAIL (is_buffer (buffer), NULL);

        // bmp = bitmap_new_for_data (buffer->context,
        //                                 width, height,
        //                                 format,
        //                                 rowstride,
        //                                 NULL /* data */);

        // bmp->buffer = object_ref (buffer);
        // bmp->data = GINT_TO_POINTER (offset);

        // return bmp;
        unimplemented!()
    }

    // * bitmap_new_from_file:
    // * @filename: the file to load.
    // * @error: a #Error or %NULL.
    // *
    // * Loads an image file from disk. This function can be safely called from
    // * within a thread.
    // *
    // * Return value: (transfer full): a #Bitmap to the new loaded
    // *               image data, or %NULL if loading the image failed.
    pub fn from_file(filename: &str) -> Bitmap {
        // _COGL_GET_CONTEXT (ctx, NULL);

        // _COGL_RETURN_VAL_IF_FAIL (filename != NULL, NULL);
        // _COGL_RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

        // return _bitmap_from_file (ctx, filename, error);
        unimplemented!()
    }

    // * bitmap_new_with_size:
    // * @context: A #Context
    // * @width: width of the bitmap in pixels
    // * @height: height of the bitmap in pixels
    // * @format: the format of the pixels the array will store
    // *
    // * Creates a new #Bitmap with the given width, height and format.
    // * The initial contents of the bitmap are undefined.
    // *
    // * The data for the bitmap will be stored in a newly created
    // * #PixelBuffer. You can get a pointer to the pixel buffer using
    // * bitmap_get_buffer(). The #Buffer API can then be
    // * used to fill the bitmap with data.
    // *
    // * <note> will try its best to provide a hardware array you can
    // * map, write into and effectively do a zero copy upload when creating
    // * a texture from it with texture_new_from_bitmap(). For various
    // * reasons, such arrays are likely to have a stride larger than width
    // * * bytes_per_pixel. The user must take the stride into account when
    // * writing into it. The stride can be retrieved with
    // * bitmap_get_rowstride().</note>
    // *
    // * Return value: (transfer full): a #PixelBuffer representing the
    // *               newly created array or %NULL on failure
    // *
    // * Since: 1.10
    // * Stability: Unstable
    pub fn with_size(context: &Context, width: u32, height: u32, format: PixelFormat) -> Bitmap {
        // PixelBuffer *pixel_buffer;
        // Bitmap *bitmap;
        // unsigned int rowstride;

        // /* creating a buffer to store "any" format does not make sense */
        // _COGL_RETURN_VAL_IF_FAIL (format != COGL_PIXEL_FORMAT_ANY, NULL);

        // /* for now we fallback to pixel_buffer_new, later, we could ask
        // * libdrm a tiled buffer for instance */
        // rowstride = width * _pixel_format_get_bytes_per_pixel (format);

        // pixel_buffer =
        //     pixel_buffer_new (context,
        //                         height * rowstride,
        //                         NULL); /* data */
        // _COGL_RETURN_VAL_IF_FAIL (pixel_buffer != NULL, NULL);

        // bitmap = bitmap_new_from_buffer (COGL_BUFFER (pixel_buffer),
        //                                         format,
        //                                         width, height,
        //                                         rowstride,
        //                                         0 /* offset */);

        // object_unref (pixel_buffer);

        // return bitmap;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the `PixelBuffer` that this
    ///  buffer uses for storage. Note that if the bitmap was created with
    ///  `Bitmap::new_from_file` then it will not actually be using a
    ///  pixel buffer and this function will return `None`.
    pub fn get_buffer(&self) -> Option<PixelBuffer> {
        // while (bitmap->shared_bmp)
        //     bitmap = bitmap->shared_bmp;

        // return COGL_PIXEL_BUFFER (bitmap->buffer);
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the `PixelFormat` that the data for the bitmap is in.
    pub fn get_format(&self) -> PixelFormat {
        // bitmap->format = format;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the height of the bitmap
    pub fn get_height(&self) -> i32 {
        // return bitmap->height;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the rowstride of the bitmap. This is the number of
    ///  bytes between the address of start of one row to the address of the
    ///  next row in the image.
    pub fn get_rowstride(&self) -> i32 {
        // return bitmap->rowstride;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the width of the bitmap
    pub fn get_width(&self) -> i32 {
        // return bitmap->width;
        unimplemented!()
    }

    /// Parses an image file enough to extract the width and height
    /// of the bitmap.
    /// ## `filename`
    /// the file to check
    /// ## `width`
    /// return location for the bitmap width, or `None`
    /// ## `height`
    /// return location for the bitmap height, or `None`
    ///
    /// # Returns
    ///
    /// `true` if the image was successfully parsed
    pub fn get_size_from_file(filename: &str) -> (bool, i32, i32) {
        // return _bitmap_get_size_from_file (filename, width, height);
        unimplemented!()
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitmap")
    }
}
