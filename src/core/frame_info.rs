use super::Output;
use std::fmt;

#[derive(Default, Debug)]
pub struct FrameInfo {
    frame_counter: i64,
    presentation_time: i64,
    refresh_rate: f32,

    output: Option<Output>,
}

impl FrameInfo {
    pub fn new() -> Self {
        Default::default()
    }
    /// Gets the frame counter for the `Onscreen` that corresponds
    /// to this frame.
    ///
    /// # Returns
    ///
    /// The frame counter value
    pub fn get_frame_counter(&self) -> i64 {
        self.frame_counter
    }

    /// Gets the `Output` that the swapped frame was presented to.
    ///
    /// # Returns
    ///
    /// The `Output` that the frame was
    ///  presented to, or `None` if this could not be determined.
    pub fn get_output(&self) -> &Option<Output> {
        &self.output
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
        self.presentation_time
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
        self.refresh_rate
    }
}

impl fmt::Display for FrameInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameInfo")
    }
}
