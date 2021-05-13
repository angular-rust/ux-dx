use super::SwapChain;
use std::fmt;

pub struct OnscreenTemplate {
    // Object _parent;

// FramebufferConfig config;
}

impl OnscreenTemplate {
    // * onscreen_new: (constructor)
    // * @context: A #Context
    // * @width: The desired framebuffer width
    // * @height: The desired framebuffer height
    // *
    // * Instantiates an "unallocated" #Onscreen framebuffer that may be
    // * configured before later being allocated, either implicitly when
    // * it is first used or explicitly via framebuffer_allocate().
    // *
    // * Return value: (transfer full): A newly instantiated #Onscreen framebuffer
    // * Since: 1.8
    // * Stability: unstable
    pub fn new(swap_chain: &SwapChain) -> OnscreenTemplate {
        // OnscreenTemplate *onscreen_template = g_slice_new0 (OnscreenTemplate);
        // char *user_config;

        // onscreen_template->config.swap_chain = swap_chain;
        // if (swap_chain)
        //     object_ref (swap_chain);
        // else
        //     onscreen_template->config.swap_chain = swap_chain_new ();

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

        // return _onscreen_template_object_new (onscreen_template);
        unimplemented!()
    }

    /// Requires that any future Onscreen framebuffers derived from
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
