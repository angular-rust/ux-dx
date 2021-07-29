use super::SubpixelOrder;
use std::fmt;

// SECTION:output
// @short_description: information about an output device
//
// The #Output object holds information about an output device
// such as a monitor or laptop display. It can be queried to find
// out the position of the output with respect to the screen
// coordinate system and other information such as the resolution
// and refresh rate of the device.
//
// There can be any number of outputs which may overlap: the
// same area of the screen may be displayed by multiple output
// devices.
//
// XXX: though it's possible to query the position of the output
// with respect to screen coordinates, there is currently no way
// of finding out the position of a #Onscreen in screen
// coordinates, at least without using windowing-system specific
// API's, so it's not easy to get the output positions relative
// to the #Onscreen.
#[derive(Default, Debug)]
pub struct Output {
    name: Option<String>,

    // Must be first field for _output_values_equal()
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    mm_width: i32,
    mm_height: i32,
    refresh_rate: f32,
    subpixel_order: SubpixelOrder,
}

impl Output {
    /// Gets the height of the output in pixels.
    ///
    /// # Returns
    ///
    /// the height of the output in pixels
    pub fn get_height(&self) -> i32 {
        self.height
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
        self.mm_height
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
        self.mm_width
    }

    /// Gets the number of times per second that the output device refreshes
    /// the display contents.
    ///
    /// # Returns
    ///
    /// the refresh rate of the output device. A value of zero
    ///  indicates that the refresh rate is unknown.
    pub fn get_refresh_rate(&self) -> f32 {
        self.refresh_rate
    }

    /// For an output device where each pixel is made up of smaller components
    /// with different colors, returns the layout of the subpixel
    /// components.
    ///
    /// # Returns
    ///
    /// the order of subpixel components for the output device
    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        self.subpixel_order
    }

    /// Gets the width of the output in pixels.
    ///
    /// # Returns
    ///
    /// the width of the output in pixels
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Gets the X position of the output with respect to the coordinate
    /// system of the screen.
    ///
    /// # Returns
    ///
    /// the X position of the output as a pixel offset
    ///  from the left side of the screen coordinate space
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Gets the Y position of the output with respect to the coordinate
    /// system of the screen.
    ///
    /// # Returns
    ///
    /// the Y position of the output as a pixel offset
    ///  from the top side of the screen coordinate space
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output")
    }
}
