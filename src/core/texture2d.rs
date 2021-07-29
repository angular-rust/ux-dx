use super::{Bitmap, Context, PixelFormat, Texture, TextureLoader};
use std::{fmt, ptr};

// @extends Object, @implements Texture;
pub struct Texture2D {
    // Texture _parent;

// The internal format of the GL texture represented as a
//     PixelFormat */
// PixelFormat internal_format;

// Bool auto_mipmap;
// Bool mipmaps_dirty;
// Bool is_foreign;

// TODO: factor out these OpenGL specific members into some form
// of driver private state. */

// The internal format of the GL texture represented as a GL enum */
// GLenum gl_internal_format;
// The texture object number */
// GLuint gl_texture;
// GLenum gl_legacy_texobj_min_filter;
// GLenum gl_legacy_texobj_mag_filter;
// GLint gl_legacy_texobj_wrap_mode_s;
// GLint gl_legacy_texobj_wrap_mode_t;
// TexturePixel first_pixel;
}

impl Texture2D {
    /// Wraps an existing GL_TEXTURE_2D texture object as a `Texture2D`.
    /// This can be used for integrating  with software using OpenGL
    /// directly.
    ///
    /// The texture is still configurable until it has been allocated so
    /// for example you can declare whether the texture is premultiplied
    /// with `Texture::set_premultiplied`.
    ///
    /// `<note>`The results are undefined for passing an invalid `gl_handle`
    /// or if `width` or `height` don't have the correct texture
    /// geometry.`</note>`
    ///
    /// ## `ctx`
    /// A `Context`
    /// ## `gl_handle`
    /// A GL handle for a GL_TEXTURE_2D texture object
    /// ## `width`
    /// Width of the foreign GL texture
    /// ## `height`
    /// Height of the foreign GL texture
    /// ## `format`
    /// The format of the texture
    ///
    /// # Returns
    ///
    /// A newly allocated `Texture2D`
    pub fn gl_new_from_foreign(
        ctx: &Context,
        gl_handle: u32,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> Texture2D {
        // ffi::texture_2d_gl_new_from_foreign
        unimplemented!()
    }

    pub fn from_bitmap(bitmap: &Bitmap) -> Texture2D {
        // _RETURN_VAL_IF_FAIL (bmp != NULL, NULL);

        let loader = TextureLoader::Bitmap {
            bitmap: bitmap.clone(),
            can_convert_in_place: false,
            height: 0,
            depth: 0,
        };

        Self::create_base(
            &Context::global(),
            bitmap.get_width(),
            bitmap.get_height(),
            bitmap.get_format(),
            &loader,
        )
    }

    pub fn from_data(
        ctx: &Context,
        width: u32,
        height: u32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> Texture2D {
        // Bitmap *bmp;
        // Texture2D *tex_2d;

        // _RETURN_VAL_IF_FAIL (format != PIXEL_FORMAT_ANY, NULL);
        // _RETURN_VAL_IF_FAIL (data != NULL, NULL);

        // Rowstride from width if not given
        let rowstride = match rowstride {
            0 => width * format.bytes_per_pixel(),
            _ => rowstride,
        };

        // Wrap the data into a bitmap
        // bmp = bitmap_new_for_data (ctx,
        //                                 width, height,
        //                                 format,
        //                                 rowstride,
        //                                 (uint8_t *) data);

        // tex_2d = texture_2d_new_from_bitmap (bmp);

        // object_unref (bmp);

        // if (tex_2d &&
        //     !texture_allocate (TEXTURE (tex_2d), error))
        //     {
        //     object_unref (tex_2d);
        //     return NULL;
        //     }

        // return tex_2d;
        unimplemented!()
    }

    pub fn from_file(ctx: &Context, filename: &str) -> Texture2D {
        // Bitmap *bmp;
        // Texture2D *tex_2d = NULL;

        // _RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

        // bmp = _bitmap_from_file (ctx, filename, error);
        // if (bmp == NULL)
        //     return NULL;

        // tex_2d = _texture_2d_new_from_bitmap (bmp,
        //                                             true); /* can convert in-place */
        // object_unref (bmp);

        // return tex_2d;
        unimplemented!()
    }

    pub fn with_size(context: &Context, width: u32, height: u32) -> Texture2D {
        let loader = TextureLoader::Sized {
            depth: 0,
            width: 0,
            height: 0,
        };

        Self::create_base(context, width, height, PixelFormat::Rgba8888Pre, &loader)
    }

    fn create_base(
        context: &Context,
        width: u32,
        height: u32,
        internal_format: PixelFormat,
        loader: &TextureLoader,
    ) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for Texture2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2D")
    }
}
