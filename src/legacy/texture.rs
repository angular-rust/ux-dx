#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Bitmap, Object, PixelFormat, TextureComponents};

use glib::object::IsA;
use glib::translate::*;
use std::{fmt, mem, ptr};

glib_wrapper! {
    pub struct Texture(Interface<ffi::CoglTexture>) @requires Object;

    match fn {
        get_type => || ffi::cogl_texture_get_gtype(),
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
    /// `<note>`Normally applications don't need to use this api directly
    /// since the texture will be implicitly allocated when data is set on
    /// the texture, or if the texture is attached to a `CoglOffscreen`
    /// framebuffer and rendered too.`</note>`
    ///
    /// # Returns
    ///
    /// `true` if the texture was successfully allocated,
    ///  otherwise `false` and `error` will be updated if it
    ///  wasn't `None`.
    fn allocate(&self) -> Result<bool, glib::Error>;

    /// Queries what components the given `self` stores internally as set
    /// via `Texture::set_components`.
    ///
    /// For textures created by the ‘_with_size’ constructors the default
    /// is `TextureComponents::Rgba`. The other constructors which take
    /// a `Bitmap` or a data pointer default to the same components as
    /// the pixel format of the data.
    fn get_components(&self) -> TextureComponents;

    /// Copies the pixel data from a cogl texture to system memory.
    ///
    /// `<note>`Don't pass the value of `texture_get_rowstride` as the
    /// `rowstride` argument, the rowstride should be the rowstride you
    /// want for the destination `data` buffer not the rowstride of the
    /// source texture`</note>`
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
    fn get_data(&self, format: PixelFormat, rowstride: u32, data: &[u8]) -> i32;

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
    fn get_gl_texture(&self) -> (bool, u32, u32);

    /// Queries the height of a cogl texture.
    ///
    /// # Returns
    ///
    /// the height of the GPU side texture in pixels
    fn get_height(&self) -> u32;

    /// Queries the maximum wasted (unused) pixels in one dimension of a GPU side
    /// texture.
    ///
    /// # Returns
    ///
    /// the maximum waste
    fn get_max_waste(&self) -> i32;

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
    fn get_premultiplied(&self) -> bool;

    /// Queries the width of a cogl texture.
    ///
    /// # Returns
    ///
    /// the width of the GPU side texture in pixels
    fn get_width(&self) -> u32;

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
    /// `<note>`Since the storage for a `Texture` is allocated lazily then
    /// if the given `self` has not previously been allocated then this
    /// api can return `false` and throw an exceptional `error` if there is
    /// not enough memory to allocate storage for `self`.`</note>`
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
    fn set_data(
        &self,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
        level: i32,
    ) -> Result<bool, glib::Error>;

    /// Affects the internal storage format for this texture by specifying
    /// whether red, green and blue color components should be stored as
    /// pre-multiplied alpha values.
    ///
    /// This api affects how data is uploaded to the GPU since Cogl will
    /// convert source data to have premultiplied or unpremultiplied
    /// components according to this state.
    ///
    /// For example if you create a texture via
    /// `Texture2D::new_with_size` and then upload data via
    /// `Texture::set_data` passing a source format of
    /// `PixelFormat::Rgba8888` then Cogl will internally multiply the
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
    /// `<note>`The region set can't be larger than the source `data``</note>`
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
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> bool;

    /// Copies a specified source region from `bitmap` to the position
    /// (`src_x`, `src_y`) of the given destination texture `handle`.
    ///
    /// `<note>`The region updated can't be larger than the source
    /// bitmap`</note>`
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

impl<O: IsA<Texture>> TextureExt for O {
    fn allocate(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_allocate(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_components(&self) -> TextureComponents {
        unsafe {
            from_glib(ffi::cogl_texture_get_components(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_data(&self, format: PixelFormat, rowstride: u32, data: &[u8]) -> i32 {
        unsafe {
            ffi::cogl_texture_get_data(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                rowstride,
                data.to_glib_none().0,
            )
        }
    }

    fn get_gl_texture(&self) -> (bool, u32, u32) {
        unsafe {
            let mut out_gl_handle = mem::MaybeUninit::uninit();
            let mut out_gl_target = mem::MaybeUninit::uninit();
            let ret = ffi::cogl_texture_get_gl_texture(
                self.as_ref().to_glib_none().0,
                out_gl_handle.as_mut_ptr(),
                out_gl_target.as_mut_ptr(),
            );
            let out_gl_handle = out_gl_handle.assume_init();
            let out_gl_target = out_gl_target.assume_init();
            (ret == crate::TRUE, out_gl_handle, out_gl_target)
        }
    }

    fn get_height(&self) -> u32 {
        unsafe { ffi::cogl_texture_get_height(self.as_ref().to_glib_none().0) }
    }

    fn get_max_waste(&self) -> i32 {
        unsafe { ffi::cogl_texture_get_max_waste(self.as_ref().to_glib_none().0) }
    }

    fn get_premultiplied(&self) -> bool {
        unsafe {
            ffi::cogl_texture_get_premultiplied(self.as_ref().to_glib_none().0) == crate::TRUE
        }
    }

    fn get_width(&self) -> u32 {
        unsafe { ffi::cogl_texture_get_width(self.as_ref().to_glib_none().0) }
    }

    fn is_sliced(&self) -> bool {
        unsafe { ffi::cogl_texture_is_sliced(self.as_ref().to_glib_none().0) == crate::TRUE }
    }

    fn set_components(&self, components: TextureComponents) {
        unsafe {
            ffi::cogl_texture_set_components(self.as_ref().to_glib_none().0, components.to_glib());
        }
    }

    fn set_data(
        &self,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
        level: i32,
    ) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_texture_set_data(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                rowstride,
                data.to_glib_none().0,
                level,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_premultiplied(&self, premultiplied: bool) {
        unsafe {
            ffi::cogl_texture_set_premultiplied(
                self.as_ref().to_glib_none().0,
                premultiplied as i32,
            );
        }
    }

    fn set_region(
        &self,
        src_x: i32,
        src_y: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: u32,
        dst_height: u32,
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: u32,
        data: &[u8],
    ) -> bool {
        unsafe {
            ffi::cogl_texture_set_region(
                self.as_ref().to_glib_none().0,
                src_x,
                src_y,
                dst_x,
                dst_y,
                dst_width,
                dst_height,
                width,
                height,
                format.to_glib(),
                rowstride,
                data.as_ptr(),
            ) == crate::TRUE
        }
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
        unsafe {
            ffi::cogl_texture_set_region_from_bitmap(
                self.as_ref().to_glib_none().0,
                src_x,
                src_y,
                dst_x,
                dst_y,
                dst_width,
                dst_height,
                bitmap.to_glib_none().0,
            ) == crate::TRUE
        }
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture")
    }
}
