use super::{Bitmap, Context, PixelFormat, Texture};
use std::{fmt, ptr};

// @extends Object, @implements Texture;
pub struct Texture2D {
    // Texture _parent;

// /* The internal format of the GL texture represented as a
//     PixelFormat */
// PixelFormat internal_format;

// Bool auto_mipmap;
// Bool mipmaps_dirty;
// Bool is_foreign;

// /* TODO: factor out these OpenGL specific members into some form
// * of driver private state. */

// /* The internal format of the GL texture represented as a GL enum */
// GLenum gl_internal_format;
// /* The texture object number */
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
        // TextureLoader *loader;

        // _COGL_RETURN_VAL_IF_FAIL (bmp != NULL, NULL);

        // loader = _texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_BITMAP;
        // loader->src.bitmap.bitmap = object_ref (bmp);
        // loader->src.bitmap.can_convert_in_place = can_convert_in_place;

        // return  _texture_2d_create_base (_bitmap_get_context (bmp),
        //                                       bitmap_get_width (bmp),
        //                                       bitmap_get_height (bmp),
        //                                       bitmap_get_format (bmp),
        //                                       loader);
        unimplemented!()
    }

    pub fn from_data(
        ctx: &Context,
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> Texture2D {
        // Bitmap *bmp;
        // Texture2D *tex_2d;

        // _COGL_RETURN_VAL_IF_FAIL (format != COGL_PIXEL_FORMAT_ANY, NULL);
        // _COGL_RETURN_VAL_IF_FAIL (data != NULL, NULL);

        // /* Rowstride from width if not given */
        // if (rowstride == 0)
        //     rowstride = width * _pixel_format_get_bytes_per_pixel (format);

        // /* Wrap the data into a bitmap */
        // bmp = bitmap_new_for_data (ctx,
        //                                 width, height,
        //                                 format,
        //                                 rowstride,
        //                                 (uint8_t *) data);

        // tex_2d = texture_2d_new_from_bitmap (bmp);

        // object_unref (bmp);

        // if (tex_2d &&
        //     !texture_allocate (COGL_TEXTURE (tex_2d), error))
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

        // _COGL_RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

        // bmp = _bitmap_from_file (ctx, filename, error);
        // if (bmp == NULL)
        //     return NULL;

        // tex_2d = _texture_2d_new_from_bitmap (bmp,
        //                                             true); /* can convert in-place */
        // object_unref (bmp);

        // return tex_2d;
        unimplemented!()
    }

    pub fn with_size(ctx: &Context, width: i32, height: i32) -> Texture2D {
        // TextureLoader *loader;

        // loader = _texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_SIZED;
        // loader->src.sized.width = width;
        // loader->src.sized.height = height;

        // return _texture_2d_create_base (ctx, width, height,
        //                                     COGL_PIXEL_FORMAT_RGBA_8888_PRE, loader);
        unimplemented!()
    }
}

impl fmt::Display for Texture2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2D")
    }
}
