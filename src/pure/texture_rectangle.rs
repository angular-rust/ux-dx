#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Bitmap, Context, PixelFormat, Texture};
use std::fmt;

// * SECTION:cogl-texture-rectangle
// * @short_description: Functions for creating and manipulating rectangle
// *                     textures for use with non-normalized coordinates.
// *
// * These functions allow low-level "rectangle" textures to be allocated.
// * These textures are never constrained to power-of-two sizes but they
// * also don't support having a mipmap and can only be wrapped with
// * %COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE.
// *
// * The most notable difference between rectangle textures and 2D
// * textures is that rectangle textures are sampled using un-normalized
// * texture coordinates, so instead of using coordinates (0,0) and
// * (1,1) to map to the top-left and bottom right corners of the
// * texture you would instead use (0,0) and (width,height).
// *
// * The use of non-normalized coordinates can be particularly
// * convenient when writing glsl shaders that use a texture as a lookup
// * table since you don't need to upload separate uniforms to map
// * normalized coordinates to texels.
// *
// * If you want to sample from a rectangle texture from GLSL you should
// * use the sampler2DRect sampler type.
// *
// * Applications wanting to use #TextureRectangle should first check
// * for the %COGL_FEATURE_ID_TEXTURE_RECTANGLE feature using
// * has_feature().
// @extends Object, @implements Texture;
pub struct TextureRectangle {
    // Texture _parent;

// /* The internal format of the texture represented as a
//     PixelFormat */
// PixelFormat internal_format;

// /* TODO: factor out these OpenGL specific members into some form
// * of driver private state. */

// /* The internal format of the GL texture represented as a GL enum */
// GLenum gl_format;
// /* The texture object number */
// GLuint gl_texture;
// GLenum gl_legacy_texobj_min_filter;
// GLenum gl_legacy_texobj_mag_filter;
// GLint gl_legacy_texobj_wrap_mode_s;
// GLint gl_legacy_texobj_wrap_mode_t;
// Bool is_foreign;
}

impl TextureRectangle {
    // * texture_rectangle_new_from_bitmap:
    // * @bitmap: A #Bitmap
    // *
    // * Allocates a new #TextureRectangle texture which will be
    // * initialized with the pixel data from @bitmap. This texture is a
    // * low-level texture that the GPU can sample from directly unlike
    // * high-level textures such as #Texture2DSliced and
    // * #AtlasTexture.
    // *
    // * <note>Unlike for #Texture2D textures, coordinates for
    // * #TextureRectangle textures should not be normalized. So instead
    // * of using the coordinate (1, 1) to sample the bottom right corner of
    // * a rectangle texture you would use (@width, @height) where @width
    // * and @height are the width and height of the texture.</note>
    // *
    // * <note>If you want to sample from a rectangle texture from GLSL you
    // * should use the sampler2DRect sampler type.</note>
    // *
    // * <note>Applications wanting to use #TextureRectangle should
    // * first check for the %COGL_FEATURE_ID_TEXTURE_RECTANGLE feature
    // * using has_feature().</note>
    // *
    // * The storage for the texture is not allocated before this function
    // * returns. You can call texture_allocate() to explicitly
    // * allocate the underlying storage or preferably let
    // * automatically allocate storage lazily when it may know more about
    // * how the texture is going to be used and can optimize how it is
    // * allocated.
    // *
    // * Return value: (transfer full): A pointer to a new
    // *               #TextureRectangle texture.
    // * Since: 2.0
    // * Stability: unstable
    pub fn from_bitmap(bitmap: &Bitmap) -> TextureRectangle {
        // TextureLoader *loader;

        // _COGL_RETURN_VAL_IF_FAIL (is_bitmap (bmp), NULL);

        // loader = _texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_BITMAP;
        // loader->src.bitmap.bitmap = object_ref (bmp);
        // loader->src.bitmap.can_convert_in_place = FALSE; /* TODO add api for this */
        // return _texture_rectangle_create_base (_bitmap_get_context (bmp),
        //                                             bitmap_get_width (bmp),
        //                                             bitmap_get_height (bmp),
        //                                             bitmap_get_format (bmp),
        //                                             loader);
        unimplemented!()
    }

    // * texture_rectangle_new_from_foreign:
    // * @ctx: A #Context
    // * @gl_handle: A GL handle for a GL_TEXTURE_RECTANGLE texture object
    // * @width: Width of the foreign GL texture
    // * @height: Height of the foreign GL texture
    // * @format: The format of the texture
    // *
    // * Wraps an existing GL_TEXTURE_RECTANGLE texture object as a
    // * #TextureRectangle.  This can be used for integrating  with
    // * software using OpenGL directly.
    // *
    // * <note>Unlike for #Texture2D textures, coordinates for
    // * #TextureRectangle textures should not be normalized. So instead
    // * of using the coordinate (1, 1) to sample the bottom right corner of
    // * a rectangle texture you would use (@width, @height) where @width
    // * and @height are the width and height of the texture.</note>
    // *
    // * <note>The results are undefined for passing an invalid @gl_handle
    // * or if @width or @height don't have the correct texture
    // * geometry.</note>
    // *
    // * <note>If you want to sample from a rectangle texture from GLSL you
    // * should use the sampler2DRect sampler type.</note>
    // *
    // * <note>Applications wanting to use #TextureRectangle should
    // * first check for the %COGL_FEATURE_ID_TEXTURE_RECTANGLE feature
    // * using has_feature().</note>
    // *
    // * The texture is still configurable until it has been allocated so
    // * for example you can declare whether the texture is premultiplied
    // * with texture_set_premultiplied().
    // *
    // * Return value: (transfer full): A new #TextureRectangle texture
    pub fn from_foreign(
        ctx: &Context,
        gl_handle: u32,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> TextureRectangle {
        // TextureLoader *loader;

        // /* NOTE: width, height and internal format are not queriable in
        // * GLES, hence such a function prototype. Also in the case of full
        // * opengl the user may be creating a  texture for a
        // * texture_from_pixmap object where glTexImage2D may not have been
        // * called and the texture_from_pixmap spec doesn't clarify that it
        // * is reliable to query back the size from OpenGL.
        // */
        // /* Assert that it is a valid GL texture object */
        // _COGL_RETURN_VAL_IF_FAIL (ctx->glIsTexture (gl_handle), NULL);

        // /* Validate width and height */
        // _COGL_RETURN_VAL_IF_FAIL (width > 0 && height > 0, NULL);

        // loader = _texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_GL_FOREIGN;
        // loader->src.gl_foreign.gl_handle = gl_handle;
        // loader->src.gl_foreign.width = width;
        // loader->src.gl_foreign.height = height;
        // loader->src.gl_foreign.format = format;

        // return _texture_rectangle_create_base (ctx, width, height,
        //                                             format, loader);
        unimplemented!()
    }

    // * texture_rectangle_new_with_size:
    // * @ctx: A #Context pointer
    // * @width: The texture width to allocate
    // * @height: The texture height to allocate
    // *
    // * Creates a new #TextureRectangle texture with a given @width,
    // * and @height. This texture is a low-level texture that the GPU can
    // * sample from directly unlike high-level textures such as
    // * #Texture2DSliced and #AtlasTexture.
    // *
    // * <note>Unlike for #Texture2D textures, coordinates for
    // * #TextureRectangle textures should not be normalized. So instead
    // * of using the coordinate (1, 1) to sample the bottom right corner of
    // * a rectangle texture you would use (@width, @height) where @width
    // * and @height are the width and height of the texture.</note>
    // *
    // * <note>If you want to sample from a rectangle texture from GLSL you
    // * should use the sampler2DRect sampler type.</note>
    // *
    // * <note>Applications wanting to use #TextureRectangle should
    // * first check for the %COGL_FEATURE_ID_TEXTURE_RECTANGLE feature
    // * using has_feature().</note>
    // *
    // * The storage for the texture is not allocated before this function
    // * returns. You can call texture_allocate() to explicitly
    // * allocate the underlying storage or preferably let
    // * automatically allocate storage lazily when it may know more about
    // * how the texture is going to be used and can optimize how it is
    // * allocated.
    // *
    // * Returns value: (transfer full): A pointer to a new #TextureRectangle
    // *          object with no storage allocated yet.
    // *
    // * Since: 1.10
    // * Stability: unstable
    pub fn with_size(ctx: &Context, width: i32, height: i32) -> TextureRectangle {
        // TextureLoader *loader = _texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_SIZED;
        // loader->src.sized.width = width;
        // loader->src.sized.height = height;

        // return _texture_rectangle_create_base (ctx, width, height,
        //                                             COGL_PIXEL_FORMAT_RGBA_8888_PRE,
        //                                             loader);
        unimplemented!()
    }
}

impl fmt::Display for TextureRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureRectangle")
    }
}
