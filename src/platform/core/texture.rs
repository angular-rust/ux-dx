#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]
use super::{Bitmap, PixelFormat, TextureComponents}; // Object,
use crate::prelude::*;
use std::{ffi::c_void, fmt, mem, ptr};

#[derive(Debug)]
pub struct Texture(c_void);

impl Object for Texture {}
impl Is<Texture> for Texture {}

impl AsRef<Texture> for Texture {
    fn as_ref(&self) -> &Texture {
        self
    }
}

/// Trait containing all `Texture` methods.
///
/// # Implementors
///
/// [`Texture2DSliced`](struct.Texture2DSliced.html), [`Texture2D`](struct.Texture2D.html), [`Texture3D`](struct.Texture3D.html), [`TextureRectangle`](struct.TextureRectangle.html), [`Texture`](struct.Texture.html)
pub trait TextureExt: 'static {
    /// Explicitly allocates the storage for the given `self` which
    /// allows you to be sure that there is enough memory for the
    /// texture and if not then the error can be handled gracefully.
    ///
    /// Normally applications don't need to use this api directly
    /// since the texture will be implicitly allocated when data is set on
    /// the texture, or if the texture is attached to a `Offscreen`
    /// framebuffer and rendered too.
    ///
    /// # Returns
    ///
    /// `true` if the texture was successfully allocated,
    ///  otherwise `false` and `error` will be updated if it
    ///  wasn't `None`.
    fn allocate(&self) -> bool;

    /// Queries what components the given `self` stores internally as set
    /// via `Texture::set_components`.
    ///
    /// For textures created by the ‘_with_size’ constructors the default
    /// is `TextureComponents::Rgba`. The other constructors which take
    /// a `Bitmap` or a data pointer default to the same components as
    /// the pixel format of the data.
    fn components(&self) -> TextureComponents;

    /// Copies the pixel data from a cogl texture to system memory.
    ///
    /// Don't pass the value of `texture_get_rowstride` as the
    /// `rowstride` argument, the rowstride should be the rowstride you
    /// want for the destination `data` buffer not the rowstride of the
    /// source texture
    /// ## `format`
    /// the `PixelFormat` to store the texture as.
    /// ## `rowstride`
    /// the rowstride of `data` in bytes or pass 0 to calculate
    ///  from the bytes-per-pixel of `format` multiplied by the
    ///  `self` width.
    /// ## `data`
    /// memory location to write the `self`'s contents, or `None`
    /// to only query the data size through the return value.
    ///
    /// # Returns
    ///
    /// the size of the texture data in bytes
    fn data(&self, format: PixelFormat, rowstride: u32, data: &[u8]) -> i32;

    /// Queries the GL handles for a GPU side texture through its `Texture`.
    ///
    /// If the texture is spliced the data for the first sub texture will be
    /// queried.
    /// ## `out_gl_handle`
    /// pointer to return location for the
    ///  textures GL handle, or `None`.
    /// ## `out_gl_target`
    /// pointer to return location for the
    ///  GL target type, or `None`.
    ///
    /// # Returns
    ///
    /// `true` if the handle was successfully retrieved, `false`
    ///  if the handle was invalid
    fn gl_texture(&self) -> (bool, u32, u32);

    /// Queries the height of a cogl texture.
    ///
    /// # Returns
    ///
    /// the height of the GPU side texture in pixels
    fn height(&self) -> u32;

    /// Queries the maximum wasted (unused) pixels in one dimension of a GPU side
    /// texture.
    ///
    /// # Returns
    ///
    /// the maximum waste
    fn max_waste(&self) -> i32;

    /// Queries the pre-multiplied alpha status for internally stored red,
    /// green and blue components for the given `self` as set by
    /// `Texture::set_premultiplied`.
    ///
    /// By default the pre-multipled state is `true`.
    ///
    /// # Returns
    ///
    /// `true` if red, green and blue components are
    ///  internally stored pre-multiplied by the alpha
    ///  value or `false` if not.
    fn premultiplied(&self) -> bool;

    /// Queries the width of a cogl texture.
    ///
    /// # Returns
    ///
    /// the width of the GPU side texture in pixels
    fn width(&self) -> u32;

    /// Queries if a texture is sliced (stored as multiple GPU side tecture
    /// objects).
    ///
    /// # Returns
    ///
    /// `true` if the texture is sliced, `false` if the texture
    ///  is stored as a single GPU texture
    fn is_sliced(&self) -> bool;

    /// Affects the internal storage format for this texture by specifying
    /// what components will be required for sampling later.
    ///
    /// This api affects how data is uploaded to the GPU since unused
    /// components can potentially be discarded from source data.
    ///
    /// For textures created by the ‘_with_size’ constructors the default
    /// is `TextureComponents::Rgba`. The other constructors which take
    /// a `Bitmap` or a data pointer default to the same components as
    /// the pixel format of the data.
    ///
    /// Note that the `TextureComponents::Rg` format is not available
    /// on all drivers. The availability can be determined by checking for
    /// the `FeatureID::OglFeatureIdTextureRg` feature. If this format is used on
    /// a driver where it is not available then `TextureError::Format`
    /// will be raised when the texture is allocated. Even if the feature
    /// is not available then `PixelFormat::Rg88` can still be used as
    /// an image format as long as `TextureComponents::Rg` isn't used
    /// as the texture's components.
    fn set_components(&self, components: TextureComponents);

    /// `self` a `Texture`.
    /// Sets all the pixels for a given mipmap `level` by copying the pixel
    /// data pointed to by the `data` argument into the given `self`.
    ///
    /// `data` should point to the first pixel to copy corresponding
    /// to the top left of the mipmap `level` being set.
    ///
    /// If `rowstride` equals 0 then it will be automatically calculated
    /// from the width of the mipmap level and the bytes-per-pixel for the
    /// given `format`.
    ///
    /// A mipmap `level` of 0 corresponds to the largest, base image of a
    /// texture and `level` 1 is half the width and height of level 0. If
    /// dividing any dimension of the previous level by two results in a
    /// fraction then round the number down (`floor`), but clamp to 1
    /// something like this:
    ///
    ///
    /// ```text
    ///  next_width = MAX (1, floor (prev_width));
    /// ```
    ///
    /// You can determine the number of mipmap levels for a given texture
    /// like this:
    ///
    ///
    /// ```text
    ///  n_levels = 1 + floor (log2 (max_dimension));
    /// ```
    ///
    /// Where `max_dimension` is the larger of `Texture::get_width` and
    /// `Texture::get_height`.
    ///
    /// It is an error to pass a `level` number >= the number of levels that
    /// `self` can have according to the above calculation.
    ///
    /// Since the storage for a `Texture` is allocated lazily then
    /// if the given `self` has not previously been allocated then this
    /// api can return `false` and throw an exceptional `error` if there is
    /// not enough memory to allocate storage for `self`.
    /// ## `format`
    /// the `PixelFormat` used in the source `data` buffer.
    /// ## `rowstride`
    /// rowstride of the source `data` buffer (computed from
    ///  the texture width and `format` if it equals 0)
    /// ## `data`
    /// the source data, pointing to the first top-left pixel to set
    /// ## `level`
    /// The mipmap level to update (Normally 0 for the largest,
    ///  base texture)
    ///
    /// # Returns
    ///
    /// `true` if the data upload was successful, and
    ///  `false` otherwise
    fn set_data(&self, format: PixelFormat, rowstride: i32, data: &[u8], level: i32) -> bool;

    /// Affects the internal storage format for this texture by specifying
    /// whether red, green and blue color components should be stored as
    /// pre-multiplied alpha values.
    ///
    /// This api affects how data is uploaded to the GPU since  will
    /// convert source data to have premultiplied or unpremultiplied
    /// components according to this state.
    ///
    /// For example if you create a texture via
    /// `Texture2D::new_with_size` and then upload data via
    /// `Texture::set_data` passing a source format of
    /// `PixelFormat::Rgba8888` then  will internally multiply the
    /// red, green and blue components of the source data by the alpha
    /// component, for each pixel so that the internally stored data has
    /// pre-multiplied alpha components. If you instead upload data that
    /// already has pre-multiplied components by passing
    /// `PixelFormat::Rgba8888Pre` as the source format to
    /// `Texture::set_data` then the data can be uploaded without being
    /// converted.
    ///
    /// By default the `premultipled` state is `true`.
    /// ## `premultiplied`
    /// Whether any internally stored red, green or blue
    ///  components are pre-multiplied by an alpha
    ///  component.
    fn set_premultiplied(&self, premultiplied: bool);

    /// Sets the pixels in a rectangular subregion of `self` from an in-memory
    /// buffer containing pixel data.
    ///
    /// The region set can't be larger than the source `data`
    /// ## `src_x`
    /// upper left coordinate to use from source data.
    /// ## `src_y`
    /// upper left coordinate to use from source data.
    /// ## `dst_x`
    /// upper left destination horizontal coordinate.
    /// ## `dst_y`
    /// upper left destination vertical coordinate.
    /// ## `dst_width`
    /// width of destination region to write. (Must be less
    ///  than or equal to `width`)
    /// ## `dst_height`
    /// height of destination region to write. (Must be less
    ///  than or equal to `height`)
    /// ## `width`
    /// width of source data buffer.
    /// ## `height`
    /// height of source data buffer.
    /// ## `format`
    /// the `PixelFormat` used in the source buffer.
    /// ## `rowstride`
    /// rowstride of source buffer (computed from width if none
    /// specified)
    /// ## `data`
    /// the actual pixel data.
    ///
    /// # Returns
    ///
    /// `true` if the subregion upload was successful, and
    ///  `false` otherwise
    fn set_region(
        &self,
        src_x: i32,
        src_y: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: u32,
        dst_height: u32,
        width: u32,
        height: u32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> bool;

    /// Copies a specified source region from `bitmap` to the position
    /// (`src_x`, `src_y`) of the given destination texture `handle`.
    ///
    /// The region updated can't be larger than the source
    /// bitmap
    /// ## `src_x`
    /// upper left coordinate to use from the source bitmap.
    /// ## `src_y`
    /// upper left coordinate to use from the source bitmap
    /// ## `dst_x`
    /// upper left destination horizontal coordinate.
    /// ## `dst_y`
    /// upper left destination vertical coordinate.
    /// ## `dst_width`
    /// width of destination region to write. (Must be less
    ///  than or equal to the bitmap width)
    /// ## `dst_height`
    /// height of destination region to write. (Must be less
    ///  than or equal to the bitmap height)
    /// ## `bitmap`
    /// The source bitmap to read from
    ///
    /// # Returns
    ///
    /// `true` if the subregion upload was successful, and
    ///  `false` otherwise
    fn set_region_from_bitmap(
        &self,
        src_x: i32,
        src_y: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: u32,
        dst_height: u32,
        bitmap: &Bitmap,
    ) -> bool;
}

impl<O: Is<Texture>> TextureExt for O {
    fn allocate(&self) -> bool {
        // if (texture->allocated)
        //     return true;

        // if (texture->components == TEXTURE_COMPONENTS_RG &&
        //     !has_feature (texture->context, FEATURE_ID_TEXTURE_RG))
        //     _set_error (error,
        //                     TEXTURE_ERROR,
        //                     TEXTURE_ERROR_FORMAT,
        //                     "A red-green texture was requested but the driver "
        //                     "does not support them");

        // texture->allocated = texture->vtable->allocate (texture, error);

        // return texture->allocated;
        unimplemented!()
    }

    fn components(&self) -> TextureComponents {
        // return texture->components;
        unimplemented!()
    }

    fn data(&self, format: PixelFormat, rowstride: u32, data: &[u8]) -> i32 {
        // Context *ctx = texture->context;
        // int byte_size;
        // PixelFormat closest_format;
        // GLenum closest_gl_format;
        // GLenum closest_gl_type;
        // Bitmap *target_bmp;
        // int tex_width;
        // int tex_height;
        // PixelFormat texture_format;
        // Error *ignore_error = NULL;

        // TextureGetData tg_data;

        // texture_format = _texture_get_format (texture);

        // Default to internal format if none specified
        // if (format == PIXEL_FORMAT_ANY)
        //     format = texture_format;

        // tex_width = texture_get_width (texture);
        // tex_height = texture_get_height (texture);

        // Rowstride from texture width if none specified

        let bpp = format.bytes_per_pixel();
        // if (rowstride == 0)
        //     rowstride = tex_width * bpp;

        // Return byte size if only that requested

        // byte_size = tex_height * rowstride;
        // if (data == NULL)
        //     return byte_size;

        // closest_format =
        //     ctx->texture_driver->find_best_gl_get_data_format (ctx,
        //                                                     format,
        //                                                     &closest_gl_format,
        //                                                     &closest_gl_type);

        // We can assume that whatever data GL gives us will have the
        //     premult status of the original texture */
        // if (PIXEL_FORMAT_CAN_HAVE_PREMULT (closest_format))
        //     closest_format = ((closest_format & ~PREMULT_BIT) |
        //                     (texture_format & PREMULT_BIT));

        // If the application is requesting a conversion from a
        // component-alpha texture and the driver doesn't support them
        // natively then we can only read into an alpha-format buffer. In
        // this case the driver will be faking the alpha textures with a
        // red-component texture and it won't swizzle to the correct format
        // while reading */
        // if (!_has_private_feature (ctx, PRIVATE_FEATURE_ALPHA_TEXTURES))
        //     {
        //     if (texture_format == PIXEL_FORMAT_A_8)
        //         {
        //         closest_format = PIXEL_FORMAT_A_8;
        //         closest_gl_format = GL_RED;
        //         closest_gl_type = GL_UNSIGNED_BYTE;
        //         }
        //     else if (format == PIXEL_FORMAT_A_8)
        //         {
        //         /* If we are converting to a component-alpha texture then we
        //         * need to read all of the components to a temporary buffer
        //         * because there is no way to get just the 4th component.
        //         * Note: it doesn't matter whether the texture is
        //         * pre-multiplied here because we're only going to look at
        //         * the alpha component */
        //         closest_format = PIXEL_FORMAT_RGBA_8888;
        //         closest_gl_format = GL_RGBA;
        //         closest_gl_type = GL_UNSIGNED_BYTE;
        //         }
        //     }

        // Is the requested format supported? */
        // if (closest_format == format)
        //     /* Target user data directly */
        //     target_bmp = bitmap_new_for_data (ctx,
        //                                         tex_width,
        //                                         tex_height,
        //                                         format,
        //                                         rowstride,
        //                                         data);
        // else
        //     {
        //     target_bmp = _bitmap_new_with_malloc_buffer (ctx,
        //                                                         tex_width, tex_height,
        //                                                         closest_format,
        //                                                         &ignore_error);
        //     if (!target_bmp)
        //         {
        //         error_free (ignore_error);
        //         return 0;
        //         }
        //     }

        // tg_data.target_bits = _bitmap_map (target_bmp, BUFFER_ACCESS_WRITE,
        //                                         BUFFER_MAP_HINT_DISCARD,
        //                                         &ignore_error);
        // if (tg_data.target_bits)
        //     {
        //     tg_data.meta_texture = texture;
        //     tg_data.orig_width = tex_width;
        //     tg_data.orig_height = tex_height;
        //     tg_data.target_bmp = target_bmp;
        //     tg_data.error = NULL;
        //     tg_data.success = true;

        //     /* If there are any dependent framebuffers on the texture then we
        //         need to flush their journals so the texture contents will be
        //         up-to-date */
        //     _texture_flush_journal_rendering (texture);

        //     /* Iterating through the subtextures allows piecing together
        //     * the data for a sliced texture, and allows us to do the
        //     * read-from-framebuffer logic here in a simple fashion rather than
        //     * passing offsets down through the code. */
        //     meta_texture_foreach_in_region (META_TEXTURE (texture),
        //                                         0, 0, 1, 1,
        //                                         PIPELINE_WRAP_MODE_REPEAT,
        //                                         PIPELINE_WRAP_MODE_REPEAT,
        //                                         texture_get_cb,
        //                                         &tg_data);

        //     _bitmap_unmap (target_bmp);
        //     }
        // else
        //     {
        //     error_free (ignore_error);
        //     tg_data.success = false;
        //     }

        // XXX: In some cases _texture_2d_download_from_gl may fail
        // to read back the texture data; such as for GLES which doesn't
        // support glGetTexImage, so here we fallback to drawing the
        // texture and reading the pixels from the framebuffer. */
        // if (!tg_data.success)
        //     {
        //     if (!_texture_draw_and_read (texture, target_bmp,
        //                                         closest_gl_format,
        //                                         closest_gl_type,
        //                                         &ignore_error))
        //         {
        //         /* We have no more fallbacks so we just give up and
        //         * hope for the best */
        //         g_warning ("Failed to read texture since draw-and-read "
        //                     "fallback failed: %s", ignore_error->message);
        //         error_free (ignore_error);
        //         object_unref (target_bmp);
        //         return 0;
        //         }
        //     }

        // Was intermediate used? */
        // if (closest_format != format)
        //     {
        //     Bitmap *new_bmp;
        //     Bool result;
        //     Error *error = NULL;

        //     /* Convert to requested format directly into the user's buffer */
        //     new_bmp = bitmap_new_for_data (ctx,
        //                                         tex_width, tex_height,
        //                                         format,
        //                                         rowstride,
        //                                         data);
        //     result = _bitmap_convert_into_bitmap (target_bmp, new_bmp, &error);

        //     if (!result)
        //         {
        //         error_free (error);
        //         /* Return failure after cleaning up */
        //         byte_size = 0;
        //         }

        //     object_unref (new_bmp);
        //     }

        // object_unref (target_bmp);

        // return byte_size;
        unimplemented!()
    }

    fn gl_texture(&self) -> (bool, u32, u32) {
        // if (!texture->allocated)
        //     texture_allocate (texture, NULL);

        // return texture->vtable->get_gl_texture (texture,
        //                                         out_gl_handle, out_gl_target);
        unimplemented!()
    }

    fn height(&self) -> u32 {
        // return texture->height;
        unimplemented!()
    }

    fn max_waste(&self) -> i32 {
        // return texture->vtable->get_max_waste (texture);
        unimplemented!()
    }

    fn premultiplied(&self) -> bool {
        // return texture->premultiplied;
        unimplemented!()
    }

    fn width(&self) -> u32 {
        // return texture->width;
        unimplemented!()
    }

    fn is_sliced(&self) -> bool {
        // if (!texture->allocated)
        //     texture_allocate (texture, NULL);
        // return texture->vtable->is_sliced (texture);
        unimplemented!()
    }

    fn set_components(&self, components: TextureComponents) {
        // _RETURN_IF_FAIL (!texture->allocated);

        // if (texture->components == components)
        //     return;

        // texture->components = components;
        unimplemented!()
    }

    fn set_data(&self, format: PixelFormat, rowstride: i32, data: &[u8], level: i32) -> bool {
        // int level_width;
        // int level_height;

        // _texture_get_level_size (texture,
        //                                 level,
        //                                 &level_width,
        //                                 &level_height,
        //                                 NULL);

        // return _texture_set_region (texture,
        //                                 level_width,
        //                                 level_height,
        //                                 format,
        //                                 rowstride,
        //                                 data,
        //                                 0, 0, /* dest x, y */
        //                                 level,
        //                                 error);
        unimplemented!()
    }

    fn set_premultiplied(&self, premultiplied: bool) {
        // _RETURN_IF_FAIL (!texture->allocated);

        // premultiplied = !!premultiplied;

        // if (texture->premultiplied == premultiplied)
        //   return;

        // texture->premultiplied = premultiplied;
        unimplemented!()
    }

    fn set_region(
        &self,
        src_x: i32,
        src_y: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: u32,
        dst_height: u32,
        width: u32,
        height: u32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> bool {
        // Error *ignore_error = NULL;
        // const uint8_t *first_pixel;
        let bytes_per_pixel = format.bytes_per_pixel();
        let status: bool;

        // Rowstride from width if none specified
        let rowstride = match rowstride {
            0 => width * bytes_per_pixel,
            _ => rowstride,
        };

        // first_pixel = data + rowstride * src_y + bytes_per_pixel * src_x;

        // status = _texture_set_region (texture,
        //                                     dst_width,
        //                                     dst_height,
        //                                     format,
        //                                     rowstride,
        //                                     first_pixel,
        //                                     dst_x,
        //                                     dst_y,
        //                                     0,
        //                                     &ignore_error);
        // if (!status)
        //     error_free (ignore_error);
        // return status;
        unimplemented!()
    }

    fn set_region_from_bitmap(
        &self,
        src_x: i32,
        src_y: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: u32,
        dst_height: u32,
        bitmap: &Bitmap,
    ) -> bool {
        // Error *ignore_error = NULL;
        // Bool status =
        //     _texture_set_region_from_bitmap (texture,
        //                                         src_x, src_y,
        //                                         dst_width, dst_height,
        //                                         bitmap,
        //                                         dst_x, dst_y,
        //                                         0, /* level */
        //                                         &ignore_error);

        // if (!status)
        //     error_free (ignore_error);
        // return status;
        unimplemented!()
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture")
    }
}
