#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Bitmap, Context, PixelFormat, Texture};
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
        width: i32,
        height: i32,
        max_waste: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> Texture2DSliced {
        // Bitmap *bmp;
        // Texture2DSliced *tex_2ds;

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

        // tex_2ds = texture_2d_sliced_new_from_bitmap (bmp, max_waste);

        // object_unref (bmp);

        // if (tex_2ds &&
        //     !texture_allocate (COGL_TEXTURE (tex_2ds), error))
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

        // _COGL_RETURN_VAL_IF_FAIL (error == NULL || *error == NULL, NULL);

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

    pub fn with_size(ctx: &Context, width: i32, height: i32, max_waste: i32) -> Texture2DSliced {
        // TextureLoader *loader = _texture_create_loader ();
        // loader->src_type = COGL_TEXTURE_SOURCE_TYPE_SIZED;
        // loader->src.sized.width = width;
        // loader->src.sized.height = height;

        // return _texture_2d_sliced_create_base (ctx,
        //                                             width,
        //                                             height,
        //                                             max_waste,
        //                                             COGL_PIXEL_FORMAT_RGBA_8888_PRE,
        //                                             loader);
        unimplemented!()
    }
}

impl fmt::Display for Texture2DSliced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2DSliced")
    }
}
