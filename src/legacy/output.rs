use crate::{Object, SubpixelOrder};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Output(Object<ffi::CoglOutput, OutputClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_output_get_gtype(),
    }
}

impl Output {
    /// Gets the height of the output in pixels.
    ///
    /// # Returns
    ///
    /// the height of the output in pixels
    pub fn get_height(&self) -> i32 {
        unsafe { ffi::cogl_output_get_height(self.to_glib_none().0) }
    }

    /// Gets the physical height of the output. In some cases (such as
    /// as a projector), the value returned here might correspond to
    /// nominal resolution rather than the actual physical size of the
    /// output device.
    ///
    /// # Returns
    ///
    /// the height of the output in millimeters. A value
    ///  of 0 indicates that the height is unknown
    pub fn get_mm_height(&self) -> i32 {
        unsafe { ffi::cogl_output_get_mm_height(self.to_glib_none().0) }
    }

    /// Gets the physical width of the output. In some cases (such as
    /// as a projector), the value returned here might correspond to
    /// nominal resolution rather than the actual physical size of the
    /// output device.
    ///
    /// # Returns
    ///
    /// the height of the output in millimeters. A value
    ///  of 0 indicates the width is unknown
    pub fn get_mm_width(&self) -> i32 {
        unsafe { ffi::cogl_output_get_mm_width(self.to_glib_none().0) }
    }

    /// Gets the number of times per second that the output device refreshes
    /// the display contents.
    ///
    /// # Returns
    ///
    /// the refresh rate of the output device. A value of zero
    ///  indicates that the refresh rate is unknown.
    pub fn get_refresh_rate(&self) -> f32 {
        unsafe { ffi::cogl_output_get_refresh_rate(self.to_glib_none().0) }
    }

    /// For an output device where each pixel is made up of smaller components
    /// with different colors, returns the layout of the subpixel
    /// components.
    ///
    /// # Returns
    ///
    /// the order of subpixel components for the output device
    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe { from_glib(ffi::cogl_output_get_subpixel_order(self.to_glib_none().0)) }
    }

    /// Gets the width of the output in pixels.
    ///
    /// # Returns
    ///
    /// the width of the output in pixels
    pub fn get_width(&self) -> i32 {
        unsafe { ffi::cogl_output_get_width(self.to_glib_none().0) }
    }

    /// Gets the X position of the output with respect to the coordinate
    /// system of the screen.
    ///
    /// # Returns
    ///
    /// the X position of the output as a pixel offset
    ///  from the left side of the screen coordinate space
    pub fn get_x(&self) -> i32 {
        unsafe { ffi::cogl_output_get_x(self.to_glib_none().0) }
    }

    /// Gets the Y position of the output with respect to the coordinate
    /// system of the screen.
    ///
    /// # Returns
    ///
    /// the Y position of the output as a pixel offset
    ///  from the top side of the screen coordinate space
    pub fn get_y(&self) -> i32 {
        unsafe { ffi::cogl_output_get_y(self.to_glib_none().0) }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output")
    }
}
