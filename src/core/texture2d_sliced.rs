#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Bitmap, Context, PixelFormat, Texture, TextureLoader};
use std::{fmt, ptr};

// @extends Object, @implements Texture;
pub struct Texture2DSliced {
    // Texture _parent;

// GArray *slice_x_spans;
// GArray *slice_y_spans;
// GArray *slice_textures;
// int max_waste;
// PixelFormat internal_format;
}

impl Texture2DSliced {
    pub fn from_bitmap(bmp: &Bitmap, max_waste: i32) -> Texture2DSliced {
        // return _texture_2d_sliced_new_from_bitmap (bmp,
        //     max_waste,
        //     false);
        unimplemented!()
    }

    pub fn from_data(
        ctx: &Context,
        width: u32,
        height: u32,
        max_waste: u32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> Texture2DSliced {
        // Bitmap *bmp;
        // Texture2DSliced *tex_2ds;

        // _RETURN_VAL_IF_FAIL (format != PIXEL_FORMAT_ANY, NULL);
        // _RETURN_VAL_IF_FAIL (data != NULL, NULL);

        // Rowstride from width if not given
        let rowstride = match rowstride {
            0 => width * format.bytes_per_pixel(),
            _ => rowstride,
        };

        // Wrap the data into a bitmap */
        // bmp = bitmap_new_for_data (ctx,
        //                                 width, height,
        //                                 format,
        //                                 rowstride,
        //                                 (uint8_t *) data);

        // tex_2ds = texture_2d_sliced_new_from_bitmap (bmp, max_waste);

        // object_unref (bmp);

        // if (tex_2ds &&
        //     !texture_allocate (TEXTURE (tex_2ds), error))
        //     {
        //     object_unref (tex_2ds);
        //     return NULL;
        //     }

        // return tex_2ds;
        unimplemented!()
    }

    pub fn from_file(ctx: &Context, filename: &str, max_waste: i32) -> Texture2DSliced {
        // Bitmap *bmp;
        // Texture2DSliced *tex_2ds = NULL;

        // _RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

        // bmp = _bitmap_from_file (ctx, filename, error);
        // if (bmp == NULL)
        //     return NULL;

        // tex_2ds = _texture_2d_sliced_new_from_bitmap (bmp,
        //                                         max_waste,
        //                                         true); /* can convert in-place */
        // object_unref (bmp);

        // return tex_2ds;
        unimplemented!()
    }

    pub fn with_size(
        context: &Context,
        width: u32,
        height: u32,
        max_waste: u32,
    ) -> Texture2DSliced {
        let loader = TextureLoader::Sized {
            depth: 0,
            width,
            height,
        };

        Self::create_base(
            context,
            width,
            height,
            max_waste,
            PixelFormat::Rgba8888Pre,
            &loader,
        )
    }

    fn create_base(
        context: &Context,
        width: u32,
        height: u32,
        max_waste: u32,
        internal_format: PixelFormat,
        loader: &TextureLoader,
    ) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for Texture2DSliced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2DSliced")
    }
}
