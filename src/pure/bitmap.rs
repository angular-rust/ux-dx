use crate::{Context, Object, PixelBuffer, PixelFormat};

use glib::translate::*;
use std::{fmt, mem, ptr};

glib_wrapper! {
    pub struct Bitmap(Object<ffi::CoglBitmap, BitmapClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_bitmap_get_gtype(),
    }
}

impl Bitmap {
    /// Creates a bitmap using some existing data. The data is not copied
    /// so the application must keep the buffer alive for the lifetime of
    /// the `Bitmap`. This can be used for example with
    /// `Framebuffer::read_pixels_into_bitmap` to read data directly
    /// into an application buffer with the specified rowstride.
    /// ## `context`
    /// A `Context`
    /// ## `width`
    /// The width of the bitmap.
    /// ## `height`
    /// The height of the bitmap.
    /// ## `format`
    /// The format of the pixel data.
    /// ## `rowstride`
    /// The rowstride of the bitmap (the number of bytes from
    ///  the start of one row of the bitmap to the next).
    /// ## `data`
    /// A pointer to the data. The bitmap will take ownership of this data.
    ///
    /// # Returns
    ///
    /// A new `Bitmap`.
    pub fn new_for_data(
        context: &Context,
        width: i32,
        height: i32,
        format: PixelFormat,
        rowstride: i32,
        data: &[u8],
    ) -> Bitmap {
        unsafe {
            from_glib_full(ffi::cogl_bitmap_new_for_data(
                context.to_glib_none().0,
                width,
                height,
                format.to_glib(),
                rowstride,
                data.to_glib_none().0,
            ))
        }
    }

    //pub fn from_buffer(buffer: /*Unknown conversion*//*Unimplemented*/Buffer, format: PixelFormat, width: i32, height: i32, rowstride: i32, offset: i32) -> Bitmap {
    //    unsafe { TODO: call cogl_sys:cogl_bitmap_new_from_buffer() }
    //}

    pub fn from_file(filename: &str) -> Result<Bitmap, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_bitmap_new_from_file(filename.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_size(context: &Context, width: u32, height: u32, format: PixelFormat) -> Bitmap {
        unsafe {
            from_glib_full(ffi::cogl_bitmap_new_with_size(
                context.to_glib_none().0,
                width,
                height,
                format.to_glib(),
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the `PixelBuffer` that this
    ///  buffer uses for storage. Note that if the bitmap was created with
    ///  `Bitmap::new_from_file` then it will not actually be using a
    ///  pixel buffer and this function will return `None`.
    pub fn get_buffer(&self) -> Option<PixelBuffer> {
        unsafe { from_glib_none(ffi::cogl_bitmap_get_buffer(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `PixelFormat` that the data for the bitmap is in.
    pub fn get_format(&self) -> PixelFormat {
        unsafe { from_glib(ffi::cogl_bitmap_get_format(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the height of the bitmap
    pub fn get_height(&self) -> i32 {
        unsafe { ffi::cogl_bitmap_get_height(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the rowstride of the bitmap. This is the number of
    ///  bytes between the address of start of one row to the address of the
    ///  next row in the image.
    pub fn get_rowstride(&self) -> i32 {
        unsafe { ffi::cogl_bitmap_get_rowstride(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the width of the bitmap
    pub fn get_width(&self) -> i32 {
        unsafe { ffi::cogl_bitmap_get_width(self.to_glib_none().0) }
    }

    /// Parses an image file enough to extract the width and height
    /// of the bitmap.
    /// ## `filename`
    /// the file to check
    /// ## `width`
    /// return location for the bitmap width, or `None`
    /// ## `height`
    /// return location for the bitmap height, or `None`
    ///
    /// # Returns
    ///
    /// `true` if the image was successfully parsed
    pub fn get_size_from_file(filename: &str) -> (bool, i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = ffi::cogl_bitmap_get_size_from_file(
                filename.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (ret == crate::TRUE, width, height)
        }
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitmap")
    }
}
