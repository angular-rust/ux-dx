use crate::{Object, SubpixelOrder};
use std::fmt;

// /**
//  * SubpixelOrder:
//  * @COGL_SUBPIXEL_ORDER_UNKNOWN: the layout of subpixel
//  *   components for the device is unknown.
//  * @COGL_SUBPIXEL_ORDER_NONE: the device displays colors
//  *   without geometrically-separated subpixel components,
//  *   or the positioning or colors of the components do not
//  *   match any of the values in the enumeration.
//  * @COGL_SUBPIXEL_ORDER_HORIZONTAL_RGB: the device has
//  *   horizontally arranged components in the order
//  *   red-green-blue from left to right.
//  * @COGL_SUBPIXEL_ORDER_HORIZONTAL_BGR: the device has
//  *   horizontally arranged  components in the order
//  *   blue-green-red from left to right.
//  * @COGL_SUBPIXEL_ORDER_VERTICAL_RGB: the device has
//  *   vertically arranged components in the order
//  *   red-green-blue from top to bottom.
//  * @COGL_SUBPIXEL_ORDER_VERTICAL_BGR: the device has
//  *   vertically arranged components in the order
//  *   blue-green-red from top to bottom.
//  *
//  * Some output devices (such as LCD panels) display colors
//  * by making each pixel consist of smaller "subpixels"
//  * that each have a particular color. By using knowledge
//  * of the layout of this subpixel components, it is possible
//  * to create image content with higher resolution than the
//  * pixel grid.
//  *
//  * Since: 1.14
//  * Stability: unstable
//  */
//  typedef enum {
//     COGL_SUBPIXEL_ORDER_UNKNOWN,
//     COGL_SUBPIXEL_ORDER_NONE,
//     COGL_SUBPIXEL_ORDER_HORIZONTAL_RGB,
//     COGL_SUBPIXEL_ORDER_HORIZONTAL_BGR,
//     COGL_SUBPIXEL_ORDER_VERTICAL_RGB,
//     COGL_SUBPIXEL_ORDER_VERTICAL_BGR
//   } SubpixelOrder;

// * SECTION:cogl-output
// * @short_description: information about an output device
// *
// * The #Output object holds information about an output device
// * such as a monitor or laptop display. It can be queried to find
// * out the position of the output with respect to the screen
// * coordinate system and other information such as the resolution
// * and refresh rate of the device.
// *
// * There can be any number of outputs which may overlap: the
// * same area of the screen may be displayed by multiple output
// * devices.
// *
// * XXX: though it's possible to query the position of the output
// * with respect to screen coordinates, there is currently no way
// * of finding out the position of a #Onscreen in screen
// * coordinates, at least without using windowing-system specific
// * API's, so it's not easy to get the output positions relative
// * to the #Onscreen.
pub struct Output {
    // Object _parent;

    // char *name;

    // int x; /* Must be first field for _output_values_equal() */
    // int y;
    // int width;
    // int height;
    // int mm_width;
    // int mm_height;
    // float refresh_rate;
    // SubpixelOrder subpixel_order
}

impl Output {
    /// Gets the height of the output in pixels.
    ///
    /// # Returns
    ///
    /// the height of the output in pixels
    pub fn get_height(&self) -> i32 {
        // return output->height;
        unimplemented!()
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
        // return output->mm_height;
        unimplemented!()
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
        // return output->mm_width;
        unimplemented!()
    }

    /// Gets the number of times per second that the output device refreshes
    /// the display contents.
    ///
    /// # Returns
    ///
    /// the refresh rate of the output device. A value of zero
    ///  indicates that the refresh rate is unknown.
    pub fn get_refresh_rate(&self) -> f32 {
        // return output->refresh_rate;
        unimplemented!()
    }

    /// For an output device where each pixel is made up of smaller components
    /// with different colors, returns the layout of the subpixel
    /// components.
    ///
    /// # Returns
    ///
    /// the order of subpixel components for the output device
    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        // return output->subpixel_order;
        unimplemented!()
    }

    /// Gets the width of the output in pixels.
    ///
    /// # Returns
    ///
    /// the width of the output in pixels
    pub fn get_width(&self) -> i32 {
        // return output->width;
        unimplemented!()
    }

    /// Gets the X position of the output with respect to the coordinate
    /// system of the screen.
    ///
    /// # Returns
    ///
    /// the X position of the output as a pixel offset
    ///  from the left side of the screen coordinate space
    pub fn get_x(&self) -> i32 {
        // return output->x;
        unimplemented!()
    }

    /// Gets the Y position of the output with respect to the coordinate
    /// system of the screen.
    ///
    /// # Returns
    ///
    /// the Y position of the output as a pixel offset
    ///  from the top side of the screen coordinate space
    pub fn get_y(&self) -> i32 {
        // return output->y;
        unimplemented!()
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output")
    }
}
