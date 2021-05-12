use crate::{Bitmap, Context, Object, PixelFormat};
use std::{fmt};


// * SECTION:cogl-atlas-texture
// * @short_description: Functions for managing textures in Cogl's global
// *                     set of texture atlases
// *
// * A texture atlas is a texture that contains many smaller images that
// * an application is interested in. These are packed together as a way
// * of optimizing drawing with those images by avoiding the costs of
// * repeatedly telling the hardware to change what texture it should
// * sample from.  This can enable more geometry to be batched together
// * into few draw calls.
// *
// * Each #CoglContext has an shared, pool of texture atlases that are
// * are managed by Cogl.
// *
// * This api lets applications upload texture data into one of Cogl's
// * shared texture atlases using a high-level #CoglAtlasTexture which
// * represents a sub-region of one of these atlases.
// *
// * <note>A #CoglAtlasTexture is a high-level meta texture which has
// * some limitations to be aware of. Please see the documentation for
// * #CoglMetaTexture for more details.</note>
pub struct AtlasTexture {
    // CoglTexture           _parent;

    // /* The format that the texture is in. This isn't necessarily the
    //    same format as the atlas texture because we can store
    //    pre-multiplied and non-pre-multiplied textures together */
    // CoglPixelFormat       internal_format;
  
    // /* The rectangle that was used to add this texture to the
    //    atlas. This includes the 1-pixel border */
    // CoglRectangleMapEntry rectangle;
  
    // /* The atlas that this texture is in. If the texture is no longer in
    //    an atlas then this will be NULL. A reference is taken on the
    //    atlas by the texture (but not vice versa so there is no cycle) */
    // CoglAtlas            *atlas;
  
    // /* Either a CoglSubTexture representing the atlas region for easy
    //  * rendering or if the texture has been migrated out of the atlas it
    //  * may be some other texture type such as CoglTexture2D */
    // CoglTexture          *sub_texture;
}

impl AtlasTexture {
    // * cogl_atlas_texture_new_from_bitmap:
    // * @bitmap: A #CoglBitmap
    // *
    // * Creates a new #CoglAtlasTexture texture based on data residing in a
    // * @bitmap. A #CoglAtlasTexture represents a sub-region within one of
    // * Cogl's shared texture atlases.
    // *
    // * The storage for the texture is not allocated before this function
    // * returns. You can call cogl_texture_allocate() to explicitly
    // * allocate the underlying storage or preferably let Cogl
    // * automatically allocate storage lazily when it may know more about
    // * how the texture is being used and can optimize how it is allocated.
    // *
    // * The texture is still configurable until it has been allocated so
    // * for example you can influence the internal format of the texture
    // * using cogl_texture_set_components() and
    // * cogl_texture_set_premultiplied().
    // *
    // * <note>Allocate call can fail if Cogl considers the internal
    // * format to be incompatible with the format of its internal
    // * atlases.</note>
    // *
    // * <note>The returned #CoglAtlasTexture is a high-level meta-texture
    // * with some limitations. See the documentation for #CoglMetaTexture
    // * for more details.</note>
    // *
    // * Returns: (transfer full): A new #CoglAtlasTexture object.
    // * Since: 1.16
    // * Stability: unstable
    pub fn from_bitmap(bmp: &Bitmap) -> AtlasTexture {
        // CoglTextureLoader *loader;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_bitmap (bmp), NULL);

        // loader = _cogl_texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_BITMAP;
        // loader->src.bitmap.bitmap = cogl_object_ref (bmp);
        // loader->src.bitmap.can_convert_in_place = can_convert_in_place;

        // return _cogl_atlas_texture_create_base (_cogl_bitmap_get_context (bmp),
        //                                         cogl_bitmap_get_width (bmp),
        //                                         cogl_bitmap_get_height (bmp),
        //                                         cogl_bitmap_get_format (bmp),
        //                                         loader);
        unimplemented!()
    }

    // * cogl_atlas_texture_new_from_data:
    // * @ctx: A #CoglContext
    // * @width: width of texture in pixels
    // * @height: height of texture in pixels
    // * @format: the #CoglPixelFormat the buffer is stored in in RAM
    // * @rowstride: the memory offset in bytes between the start of each
    // *    row in @data. A value of 0 will make Cogl automatically
    // *    calculate @rowstride from @width and @format.
    // * @data: pointer to the memory region where the source buffer resides
    // * @error: A #CoglError to catch exceptional errors or %NULL
    // *
    // * Creates a new #CoglAtlasTexture texture based on data residing in
    // * memory. A #CoglAtlasTexture represents a sub-region within one of
    // * Cogl's shared texture atlases.
    // *
    // * <note>This api will always immediately allocate GPU memory for the
    // * texture and upload the given data so that the @data pointer does
    // * not need to remain valid once this function returns. This means it
    // * is not possible to configure the texture before it is allocated. If
    // * you do need to configure the texture before allocation (to specify
    // * constraints on the internal format for example) then you can
    // * instead create a #CoglBitmap for your data and use
    // * cogl_atlas_texture_new_from_bitmap() or use
    // * cogl_atlas_texture_new_with_size() and then upload data using
    // * cogl_texture_set_data()</note>
    // *
    // * <note>Allocate call can fail if Cogl considers the internal
    // * format to be incompatible with the format of its internal
    // * atlases.</note>
    // *
    // * <note>The returned #CoglAtlasTexture is a high-level
    // * meta-texture with some limitations. See the documentation for
    // * #CoglMetaTexture for more details.</note>
    // *
    // * Return value: (transfer full): A new #CoglAtlasTexture object or
    // *          %NULL on failure and @error will be updated.
    // * Since: 1.16
    // * Stability: unstable
    pub fn from_data(
        ctx: &Context,
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> AtlasTexture {
        // CoglBitmap *bmp;
        // CoglAtlasTexture *atlas_tex;

        // _COGL_RETURN_VAL_IF_FAIL (format != COGL_PIXEL_FORMAT_ANY, NULL);
        // _COGL_RETURN_VAL_IF_FAIL (data != NULL, NULL);

        // /* Rowstride from width if not given */
        // if (rowstride == 0)
        //     rowstride = width * _cogl_pixel_format_get_bytes_per_pixel (format);

        // /* Wrap the data into a bitmap */
        // bmp = cogl_bitmap_new_for_data (ctx,
        //                                 width, height,
        //                                 format,
        //                                 rowstride,
        //                                 (uint8_t *) data);

        // atlas_tex = cogl_atlas_texture_new_from_bitmap (bmp);

        // cogl_object_unref (bmp);

        // if (atlas_tex &&
        //     !cogl_texture_allocate (COGL_TEXTURE (atlas_tex), error))
        //     {
        //     cogl_object_unref (atlas_tex);
        //     return NULL;
        //     }

        // return atlas_tex;
        unimplemented!()
    }

    // * cogl_atlas_texture_new_from_file:
    // * @ctx: A #CoglContext
    // * @filename: the file to load
    // * @error: A #CoglError to catch exceptional errors or %NULL
    // *
    // * Creates a #CoglAtlasTexture from an image file. A #CoglAtlasTexture
    // * represents a sub-region within one of Cogl's shared texture
    // * atlases.
    // *
    // * The storage for the texture is not allocated before this function
    // * returns. You can call cogl_texture_allocate() to explicitly
    // * allocate the underlying storage or let Cogl automatically allocate
    // * storage lazily.
    // *
    // * The texture is still configurable until it has been allocated so
    // * for example you can influence the internal format of the texture
    // * using cogl_texture_set_components() and
    // * cogl_texture_set_premultiplied().
    // *
    // * <note>Allocate call can fail if Cogl considers the internal
    // * format to be incompatible with the format of its internal
    // * atlases.</note>
    // *
    // * <note>The returned #CoglAtlasTexture is a high-level meta-texture
    // * with some limitations. See the documentation for #CoglMetaTexture
    // * for more details.</note>
    // *
    // * Return value: (transfer full): A new #CoglAtlasTexture object or
    // *          %NULL on failure and @error will be updated.
    // * Since: 1.16
    // * Stability: unstable
    pub fn from_file(ctx: &Context, filename: &str) -> AtlasTexture {
        // CoglBitmap *bmp;
        // CoglAtlasTexture *atlas_tex = NULL;

        // _COGL_RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

        // bmp = cogl_bitmap_new_from_file (filename, error);
        // if (bmp == NULL)
        //     return NULL;

        // atlas_tex = _cogl_atlas_texture_new_from_bitmap (bmp,
        //                                                 TRUE); /* convert in-place */

        // cogl_object_unref (bmp);

        // return atlas_tex;
        unimplemented!()
    }

    // * cogl_atlas_texture_new_with_size:
    // * @ctx: A #CoglContext
    // * @width: The width of your atlased texture.
    // * @height: The height of your atlased texture.
    // *
    // * Creates a #CoglAtlasTexture with a given @width and @height. A
    // * #CoglAtlasTexture represents a sub-region within one of Cogl's
    // * shared texture atlases.
    // *
    // * The storage for the texture is not allocated before this function
    // * returns. You can call cogl_texture_allocate() to explicitly
    // * allocate the underlying storage or let Cogl automatically allocate
    // * storage lazily.
    // *
    // * The texture is still configurable until it has been allocated so
    // * for example you can influence the internal format of the texture
    // * using cogl_texture_set_components() and
    // * cogl_texture_set_premultiplied().
    // *
    // * <note>Allocate call can fail if Cogl considers the internal
    // * format to be incompatible with the format of its internal
    // * atlases.</note>
    // *
    // * <note>The returned #CoglAtlasTexture is a high-level meta-texture
    // * with some limitations. See the documentation for #CoglMetaTexture
    // * for more details.</note>
    pub fn with_size(ctx: &Context, width: i32, height: i32) -> AtlasTexture {
        // CoglTextureLoader *loader;

        // /* We can't atlas zero-sized textures because it breaks the atlas
        //  * data structure */
        // _COGL_RETURN_VAL_IF_FAIL (width > 0 && height > 0, NULL);
      
        // loader = _cogl_texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_SIZED;
        // loader->src.sized.width = width;
        // loader->src.sized.height = height;
      
        // return _cogl_atlas_texture_create_base (ctx, width, height,
        //                                         COGL_PIXEL_FORMAT_RGBA_8888_PRE,
        //                                         loader);
        unimplemented!()
    }
}

impl fmt::Display for AtlasTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AtlasTexture")
    }
}
