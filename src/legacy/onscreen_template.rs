use crate::{Object, SwapChain};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct OnscreenTemplate(Object<ffi::CoglOnscreenTemplate, OnscreenTemplateClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_onscreen_template_get_gtype(),
    }
}

impl OnscreenTemplate {
    pub fn new(swap_chain: &SwapChain) -> OnscreenTemplate {
        unsafe { from_glib_full(ffi::cogl_onscreen_template_new(swap_chain.to_glib_none().0)) }
    }

    /// Requires that any future CoglOnscreen framebuffers derived from
    /// this template must support making at least `n` samples per pixel
    /// which will all contribute to the final resolved color for that
    /// pixel.
    ///
    /// By default this value is usually set to 0 and that is referred to
    /// as "single-sample" rendering. A value of 1 or greater is referred
    /// to as "multisample" rendering.
    ///
    /// `<note>`There are some semantic differences between single-sample
    /// rendering and multisampling with just 1 point sample such as it
    /// being redundant to use the `Framebuffer::resolve_samples` and
    /// `Framebuffer::resolve_samples_region` apis with single-sample
    /// rendering.`</note>`
    /// ## `n`
    /// The minimum number of samples per pixel
    pub fn set_samples_per_pixel(&self, n: i32) {
        unsafe {
            ffi::cogl_onscreen_template_set_samples_per_pixel(self.to_glib_none().0, n);
        }
    }

    /// Sets whether future `Onscreen` framebuffers derived from this
    /// template are attempted to be created with both left and right
    /// buffers, for use with stereo display. If the display system
    /// does not support stereo, then creation of the framebuffer will
    /// fail.
    /// ## `enabled`
    /// Whether framebuffers are created with stereo buffers
    pub fn set_stereo_enabled(&self, enabled: bool) {
        unsafe {
            ffi::cogl_onscreen_template_set_stereo_enabled(self.to_glib_none().0, enabled as i32);
        }
    }

    /// Requests that any future `Onscreen` framebuffers derived from this
    /// template should enable or disable swap throttling according to the given
    /// `throttled` argument.
    /// ## `throttled`
    /// Whether throttling should be enabled
    pub fn set_swap_throttled(&self, throttled: bool) {
        unsafe {
            ffi::cogl_onscreen_template_set_swap_throttled(self.to_glib_none().0, throttled as i32);
        }
    }
}

impl fmt::Display for OnscreenTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OnscreenTemplate")
    }
}
