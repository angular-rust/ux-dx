use crate::{Object, SwapChain};
use std::fmt;

pub struct OnscreenTemplate {
    // CoglObject _parent;

    // CoglFramebufferConfig config;
}

impl OnscreenTemplate {
    // * cogl_onscreen_new: (constructor)
    // * @context: A #CoglContext
    // * @width: The desired framebuffer width
    // * @height: The desired framebuffer height
    // *
    // * Instantiates an "unallocated" #CoglOnscreen framebuffer that may be
    // * configured before later being allocated, either implicitly when
    // * it is first used or explicitly via cogl_framebuffer_allocate().
    // *
    // * Return value: (transfer full): A newly instantiated #CoglOnscreen framebuffer
    // * Since: 1.8
    // * Stability: unstable
    pub fn new(swap_chain: &SwapChain) -> OnscreenTemplate {
        // CoglOnscreenTemplate *onscreen_template = g_slice_new0 (CoglOnscreenTemplate);
        // char *user_config;

        // onscreen_template->config.swap_chain = swap_chain;
        // if (swap_chain)
        //     cogl_object_ref (swap_chain);
        // else
        //     onscreen_template->config.swap_chain = cogl_swap_chain_new ();

        // onscreen_template->config.swap_throttled = TRUE;
        // onscreen_template->config.need_stencil = TRUE;
        // onscreen_template->config.samples_per_pixel = 0;

        // user_config = getenv ("COGL_POINT_SAMPLES_PER_PIXEL");
        // if (user_config)
        //     {
        //     unsigned long samples_per_pixel = strtoul (user_config, NULL, 10);
        //     if (samples_per_pixel != ULONG_MAX)
        //         onscreen_template->config.samples_per_pixel =
        //         samples_per_pixel;
        //     }

        // return _cogl_onscreen_template_object_new (onscreen_template);
        unimplemented!()
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
        // onscreen_template->config.samples_per_pixel = samples_per_pixel;
        unimplemented!()
    }

    /// Sets whether future `Onscreen` framebuffers derived from this
    /// template are attempted to be created with both left and right
    /// buffers, for use with stereo display. If the display system
    /// does not support stereo, then creation of the framebuffer will
    /// fail.
    /// ## `enabled`
    /// Whether framebuffers are created with stereo buffers
    pub fn set_stereo_enabled(&self, enabled: bool) {
        // onscreen_template->config.stereo_enabled = enabled;
        unimplemented!()
    }

    /// Requests that any future `Onscreen` framebuffers derived from this
    /// template should enable or disable swap throttling according to the given
    /// `throttled` argument.
    /// ## `throttled`
    /// Whether throttling should be enabled
    pub fn set_swap_throttled(&self, throttled: bool) {
        // onscreen_template->config.swap_throttled = throttled;
        unimplemented!()
    }
}

impl fmt::Display for OnscreenTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OnscreenTemplate")
    }
}
