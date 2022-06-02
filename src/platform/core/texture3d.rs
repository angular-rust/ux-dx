#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Bitmap, Context, PixelFormat, Texture, TextureLoader};
use std::{fmt, ptr};

// @extends Object, @implements Texture;
pub struct Texture3D {
    // Texture _parent;

    // The internal format of the texture represented as a
    //     PixelFormat */
    // PixelFormat internal_format;
    // int depth;
    // Bool auto_mipmap;
    // Bool mipmaps_dirty;

    // TODO: factor out these OpenGL specific members into some form
    // of driver private state. */

    // The internal format of the GL texture represented as a GL enum */
    // GLenum gl_format;
    // The texture object number */
    // GLuint gl_texture;
    // GLenum gl_legacy_texobj_min_filter;
    // GLenum gl_legacy_texobj_mag_filter;
    // GLint gl_legacy_texobj_wrap_mode_s;
    // GLint gl_legacy_texobj_wrap_mode_t;
    // GLint gl_legacy_texobj_wrap_mode_p;
    // TexturePixel first_pixel;
}

impl Texture3D {
    // texture_3d_new_from_bitmap:
    // @bitmap: A #Bitmap object::
    // @height: height of the texture in pixels.
    // @depth: depth of the texture in pixels.
    //
    // Creates a low-level 3D texture and initializes it with the images
    // in @bitmap. The images are assumed to be packed together after one
    // another in the increasing y axis. The height of individual image is
    // given as @height and the number of images is given in @depth. The
    // actual height of the bitmap can be larger than @height × @depth. In
    // this case it assumes there is padding between the images.
    //
    // The storage for the texture is not allocated before this function
    // returns. You can call texture_allocate() to explicitly
    // allocate the underlying storage or preferably let
    // automatically allocate storage lazily when it may know more about
    // how the texture is going to be used and can optimize how it is
    // allocated.
    //
    // The texture is still configurable until it has been allocated so
    // for example you can influence the internal format of the texture
    // using texture_set_components() and
    // texture_set_premultiplied().
    //
    // This texture will fail to allocate later if
    // %FEATURE_ID_TEXTURE_3D is not advertised. Allocation can also
    // fail if the requested dimensions are not supported by the
    // GPU.
    //
    // Return value: (transfer full): a newly created #Texture3D
    // Since: 2.0
    // Stability: unstable
    pub fn from_bitmap(bitmap: &Bitmap, height: u32, depth: u32) -> Texture3D {
        // _RETURN_VAL_IF_FAIL (bmp, NULL);

        let loader = TextureLoader::Bitmap {
            bitmap: bitmap.clone(),
            can_convert_in_place: false, // TODO add api for this
            depth,
            height,
        };

        Self::create_base(
            &Context::global(),
            bitmap.width(),
            height,
            depth,
            bitmap.format(),
            &loader,
        )
    }

    // texture_3d_new_from_data:
    // @context: a #Context
    // @width: width of the texture in pixels.
    // @height: height of the texture in pixels.
    // @depth: depth of the texture in pixels.
    // @format: the #PixelFormat the buffer is stored in in RAM
    // @rowstride: the memory offset in bytes between the starts of
    //    scanlines in @data or 0 to infer it from the width and format
    // @image_stride: the number of bytes from one image to the next. This
    //    can be used to add padding between the images in a similar way
    //    that the rowstride can be used to add padding between
    //    rows. Alternatively 0 can be passed to infer the @image_stride
    //    from the @height.
    // @data: pointer the memory region where the source buffer resides
    // @error: A Error return location.
    //
    // Creates a low-level 3D texture and initializes it with @data. The
    // data is assumed to be packed array of @depth images. There can be
    // padding between the images using @image_stride.
    //
    // This api will always immediately allocate GPU memory for the
    // texture and upload the given data so that the @data pointer does
    // not need to remain valid once this fn returns. This means it
    // is not possible to configure the texture before it is allocated. If
    // you do need to configure the texture before allocation (to specify
    // constraints on the internal format for example) then you can
    // instead create a #Bitmap for your data and use
    // texture_3d_new_from_bitmap().
    //
    // Return value: (transfer full): the newly created #Texture3D or
    //               %NULL if there was an error and an exception will be
    //               returned through @error.
    // Since: 1.10
    // Stability: Unstable
    pub fn from_data(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        format: PixelFormat,
        rowstride: u32,
        image_stride: u32,
        data: &[u8],
    ) -> Texture3D {
        // Bitmap *bitmap;
        // Texture3D *ret;

        // _RETURN_VAL_IF_FAIL (data, NULL);
        // _RETURN_VAL_IF_FAIL (format != PIXEL_FORMAT_ANY, NULL);

        // Rowstride from width if not given
        let rowstride = match rowstride {
            0 => width * format.bytes_per_pixel(),
            _ => rowstride,
        };

        // Image stride from height and rowstride if not given

        // if (image_stride == 0)
        //     image_stride = height * rowstride;

        // if (image_stride < rowstride * height)
        //     return NULL;

        // GL doesn't support uploading when the image_stride isn't a
        // multiple of the rowstride. If this happens we'll just pack the
        // image into a new bitmap. The documentation for this function
        // recommends avoiding this situation.

        // if (image_stride % rowstride != 0)
        //     {
        //     uint8_t *bmp_data;
        //     int bmp_rowstride;
        //     int z, y;

        //     bitmap = _bitmap_new_with_malloc_buffer (context,
        //                                                     width,
        //                                                     depth * height,
        //                                                     format,
        //                                                     error);
        //     if (!bitmap)
        //         return NULL;

        //     bmp_data = _bitmap_map (bitmap,
        //                                 BUFFER_ACCESS_WRITE,
        //                                 BUFFER_MAP_HINT_DISCARD,
        //                                 error);

        //     if (bmp_data == NULL)
        //         {
        //         object_unref (bitmap);
        //         return NULL;
        //         }

        //     bmp_rowstride = bitmap_get_rowstride (bitmap);

        //     /* Copy all of the images in */
        //     for (z = 0; z < depth; z++)
        //         for (y = 0; y < height; y++)
        //         memcpy (bmp_data + (z * bmp_rowstride * height +
        //                             bmp_rowstride * y),
        //                 data + z * image_stride + rowstride * y,
        //                 bmp_rowstride);

        //     _bitmap_unmap (bitmap);
        // } else {
        //     bitmap = bitmap_new_for_data (context,
        //                                     width,
        //                                     image_stride / rowstride * depth,
        //                                     format,
        //                                     rowstride,
        //                                     (uint8_t *) data);
        // }

        // ret = texture_3d_new_from_bitmap (bitmap,
        //                                         height,
        //                                         depth);

        // object_unref (bitmap);

        // if ret && !texture_allocate (TEXTURE (ret), error) {
        //     object_unref (ret);
        //     return NULL;
        // }

        // return ret;
        unimplemented!()
    }

    // texture_3d_new_with_size:
    // @context: a #Context
    // @width: width of the texture in pixels.
    // @height: height of the texture in pixels.
    // @depth: depth of the texture in pixels.
    //
    // Creates a low-level #Texture3D texture with the specified
    // dimensions and pixel format.
    //
    // The storage for the texture is not allocated before this function
    // returns. You can call texture_allocate() to explicitly
    // allocate the underlying storage or preferably let
    // automatically allocate storage lazily when it may know more about
    // how the texture is going to be used and can optimize how it is
    // allocated.
    //
    // The texture is still configurable until it has been allocated so
    // for example you can influence the internal format of the texture
    // using texture_set_components() and
    // texture_set_premultiplied().
    //
    // This texture will fail to allocate later if
    // %FEATURE_ID_TEXTURE_3D is not advertised. Allocation can also
    // fail if the requested dimensions are not supported by the
    // GPU.
    //
    // Returns: (transfer full): A new #Texture3D object with no storage yet allocated.
    // Since: 1.10
    // Stability: Unstable
    pub fn with_size(context: &Context, width: u32, height: u32, depth: u32) -> Texture3D {
        let loader = TextureLoader::Sized {
            depth,
            width,
            height,
        };

        Self::create_base(
            context,
            width,
            height,
            depth,
            PixelFormat::Rgba8888Pre,
            &loader,
        )
    }

    fn create_base(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        internal_format: PixelFormat,
        loader: &TextureLoader,
    ) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for Texture3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture3D")
    }
}
