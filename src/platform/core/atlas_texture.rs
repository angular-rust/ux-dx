use super::{Atlas, Bitmap, Context, PixelFormat, Texture, TextureLoader};
use std::fmt;

// @short_description: Functions for managing textures in 's global
//                     set of texture atlases
//
// A texture atlas is a texture that contains many smaller images that
// an application is interested in. These are packed together as a way
// of optimizing drawing with those images by avoiding the costs of
// repeatedly telling the hardware to change what texture it should
// sample from.  This can enable more geometry to be batched together
// into few draw calls.
//
// Each #Context has an shared, pool of texture atlases that are
// are managed by .
//
// This api lets applications upload texture data into one of 's
// shared texture atlases using a high-level #AtlasTexture which
// represents a sub-region of one of these atlases.
//
// A #AtlasTexture is a high-level meta texture which has
// some limitations to be aware of. Please see the documentation for
// #MetaTexture for more details.
pub struct AtlasTexture {
    parent: Texture,

    // The format that the texture is in. This isn't necessarily the
    // same format as the atlas texture because we can store
    // pre-multiplied and non-pre-multiplied textures together
    internal_format: PixelFormat,

    // The rectangle that was used to add this texture to the
    // atlas. This includes the 1-pixel border
    // rectangle: RectangleMapEntry,

    // The atlas that this texture is in. If the texture is no longer in
    // an atlas then this will be NULL. A reference is taken on the
    // atlas by the texture (but not vice versa so there is no cycle)
    atlas: Option<Atlas>,

    // Either a SubTexture representing the atlas region for easy
    // rendering or if the texture has been migrated out of the atlas it
    // may be some other texture type such as Texture2D
    sub_texture: Option<Texture>,
}

impl AtlasTexture {
    // atlas_texture_new_from_bitmap:
    // @bitmap: A #Bitmap
    //
    // Creates a new #AtlasTexture texture based on data residing in a
    // @bitmap. A #AtlasTexture represents a sub-region within one of
    // 's shared texture atlases.
    //
    // The storage for the texture is not allocated before this function
    // returns. You can call texture_allocate() to explicitly
    // allocate the underlying storage or preferably let
    // automatically allocate storage lazily when it may know more about
    // how the texture is being used and can optimize how it is allocated.
    //
    // The texture is still configurable until it has been allocated so
    // for example you can influence the internal format of the texture
    // using texture_set_components() and
    // texture_set_premultiplied().
    //
    // Allocate call can fail if  considers the internal
    // format to be incompatible with the format of its internal
    // atlases.
    //
    // The returned #AtlasTexture is a high-level meta-texture
    // with some limitations. See the documentation for #MetaTexture
    // for more details.
    //
    // Returns: (transfer full): A new #AtlasTexture object::
    // Since: 1.16
    // Stability: unstable
    pub fn from_bitmap(bitmap: &Bitmap) -> AtlasTexture {
        // _RETURN_VAL_IF_FAIL (is_bitmap (bmp), NULL);

        // TODO: fix bitmap
        let loader = TextureLoader::Bitmap {
            bitmap: bitmap.clone(),
            can_convert_in_place: true,
            height: 0,
            depth: 0,
        };

        Self::create_base(
            &Context::global(),
            bitmap.width(),
            bitmap.height(),
            bitmap.format(),
            &loader,
        )
    }

    // atlas_texture_new_from_data:
    // @ctx: A #Context
    // @width: width of texture in pixels
    // @height: height of texture in pixels
    // @format: the #PixelFormat the buffer is stored in in RAM
    // @rowstride: the memory offset in bytes between the start of each
    //    row in @data. A value of 0 will make  automatically
    //    calculate @rowstride from @width and @format.
    // @data: pointer to the memory region where the source buffer resides
    // @error: A #Error to catch exceptional errors or %NULL
    //
    // Creates a new #AtlasTexture texture based on data residing in
    // memory. A #AtlasTexture represents a sub-region within one of
    // 's shared texture atlases.
    //
    // This api will always immediately allocate GPU memory for the
    // texture and upload the given data so that the @data pointer does
    // not need to remain valid once this fn returns. This means it
    // is not possible to configure the texture before it is allocated. If
    // you do need to configure the texture before allocation (to specify
    // constraints on the internal format for example) then you can
    // instead create a #Bitmap for your data and use
    // atlas_texture_new_from_bitmap() or use
    // atlas_texture_new_with_size() and then upload data using
    // texture_set_data()
    //
    // Allocate call can fail if  considers the internal
    // format to be incompatible with the format of its internal
    // atlases.
    //
    // The returned #AtlasTexture is a high-level
    // meta-texture with some limitations. See the documentation for
    // #MetaTexture for more details.
    //
    // Return value: (transfer full): A new #AtlasTexture object or
    //          %NULL on failure and @error will be updated.
    // Since: 1.16
    // Stability: unstable
    pub fn from_data(
        ctx: &Context,
        width: u32,
        height: u32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> AtlasTexture {
        // Bitmap *bmp;
        // AtlasTexture *atlas_tex;

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

        // atlas_tex = atlas_texture_new_from_bitmap (bmp);

        // object_unref (bmp);

        // if atlas_tex && !texture_allocate (TEXTURE (atlas_tex), error) {
        //     object_unref (atlas_tex);
        //     return NULL;
        // }

        // return atlas_tex;
        unimplemented!()
    }

    // atlas_texture_new_from_file:
    // @ctx: A #Context
    // @filename: the file to load
    // @error: A #Error to catch exceptional errors or %NULL
    //
    // Creates a #AtlasTexture from an image file. A #AtlasTexture
    // represents a sub-region within one of 's shared texture
    // atlases.
    //
    // The storage for the texture is not allocated before this function
    // returns. You can call texture_allocate() to explicitly
    // allocate the underlying storage or let  automatically allocate
    // storage lazily.
    //
    // The texture is still configurable until it has been allocated so
    // for example you can influence the internal format of the texture
    // using texture_set_components() and
    // texture_set_premultiplied().
    //
    // Allocate call can fail if  considers the internal
    // format to be incompatible with the format of its internal
    // atlases.
    //
    // The returned #AtlasTexture is a high-level meta-texture
    // with some limitations. See the documentation for #MetaTexture
    // for more details.
    //
    // Return value: (transfer full): A new #AtlasTexture object or
    //          %NULL on failure and @error will be updated.
    // Since: 1.16
    // Stability: unstable
    pub fn from_file(ctx: &Context, filename: &str) -> AtlasTexture {
        // Bitmap *bmp;
        // AtlasTexture *atlas_tex = NULL;

        // _RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

        // bmp = bitmap_new_from_file (filename, error);
        // if (bmp == NULL)
        //     return NULL;

        // atlas_tex = _atlas_texture_new_from_bitmap (bmp, true); // convert in-place
        // object_unref (bmp);

        // return atlas_tex;
        unimplemented!()
    }

    // atlas_texture_new_with_size:
    // @ctx: A #Context
    // @width: The width of your atlased texture.
    // @height: The height of your atlased texture.
    //
    // Creates a #AtlasTexture with a given @width and @height. A
    // #AtlasTexture represents a sub-region within one of 's
    // shared texture atlases.
    //
    // The storage for the texture is not allocated before this function
    // returns. You can call texture_allocate() to explicitly
    // allocate the underlying storage or let  automatically allocate
    // storage lazily.
    //
    // The texture is still configurable until it has been allocated so
    // for example you can influence the internal format of the texture
    // using texture_set_components() and
    // texture_set_premultiplied().
    //
    // Allocate call can fail if  considers the internal
    // format to be incompatible with the format of its internal
    // atlases.
    //
    // The returned #AtlasTexture is a high-level meta-texture
    // with some limitations. See the documentation for #MetaTexture
    // for more details.
    pub fn with_size(context: &Context, width: u32, height: u32) -> AtlasTexture {
        // We can't atlas zero-sized textures because it breaks the atlas
        // data structure

        let loader = TextureLoader::Sized {
            depth: 0,
            width,
            height,
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

impl fmt::Display for AtlasTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AtlasTexture")
    }
}
