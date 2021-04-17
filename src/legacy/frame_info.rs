use crate::{Object, Output};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct FrameInfo(Object<ffi::CoglFrameInfo, FrameInfoClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_frame_info_get_gtype(),
    }
}

impl FrameInfo {
    /// Gets the frame counter for the `Onscreen` that corresponds
    /// to this frame.
    ///
    /// # Returns
    ///
    /// The frame counter value
    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::cogl_frame_info_get_frame_counter(self.to_glib_none().0) }
    }

    /// Gets the `Output` that the swapped frame was presented to.
    ///
    /// # Returns
    ///
    /// The `Output` that the frame was
    ///  presented to, or `None` if this could not be determined.
    pub fn get_output(&self) -> Option<Output> {
        unsafe { from_glib_none(ffi::cogl_frame_info_get_output(self.to_glib_none().0)) }
    }

    /// Gets the presentation time for the frame. This is the time at which
    /// the frame became visible to the user.
    ///
    /// The presentation time measured in nanoseconds is based on a
    /// monotonic time source. The time source is not necessarily
    /// correlated with system/wall clock time and may represent the time
    /// elapsed since some undefined system event such as when the system
    /// last booted.
    ///
    /// `<note>`Linux kernel version less that 3.8 can result in
    /// non-monotonic timestamps being reported when using a drm based
    /// OpenGL driver. Also some buggy Mesa drivers up to 9.0.1 may also
    /// incorrectly report non-monotonic timestamps.`</note>`
    ///
    /// # Returns
    ///
    /// the presentation time for the frame
    pub fn get_presentation_time(&self) -> i64 {
        unsafe { ffi::cogl_frame_info_get_presentation_time(self.to_glib_none().0) }
    }

    /// Gets the refresh rate in Hertz for the output that the frame was on
    /// at the time the frame was presented.
    ///
    /// `<note>`Some platforms can't associate a `Output` with a
    /// `FrameInfo` object but are able to report a refresh rate via
    /// this api. Therefore if you need this information then this api is
    /// more reliable than using `FrameInfo::get_output` followed by
    /// `Output::get_refresh_rate`.`</note>`
    ///
    /// # Returns
    ///
    /// the refresh rate in Hertz
    pub fn get_refresh_rate(&self) -> f32 {
        unsafe { ffi::cogl_frame_info_get_refresh_rate(self.to_glib_none().0) }
    }
}

impl fmt::Display for FrameInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameInfo")
    }
}
