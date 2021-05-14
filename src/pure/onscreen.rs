use super::{
    Context, FrameClosure, FrameEvent, FrameInfo, Framebuffer, OnscreenDirtyClosure,
    OnscreenDirtyInfo, OnscreenResizeClosure,
};
use std::fmt;
use winit::window::{Window, WindowBuilder};

// typedef struct _OnscreenEvent
// {
//   List link;

//   Onscreen *onscreen;
//   FrameInfo *info;
//   FrameEvent type;
// } OnscreenEvent;

// typedef struct _OnscreenQueuedDirty
// {
//   List link;

//   Onscreen *onscreen;
//   OnscreenDirtyInfo info;
// } OnscreenQueuedDirty;

// @extends Object, @implements Framebuffer;
pub struct Onscreen {
    // Framebuffer  _parent;
    window: Window,
    // #ifdef COGL_HAS_X11_SUPPORT
    //   uint32_t foreign_xid;
    //   OnscreenX11MaskCallback foreign_update_mask_callback;
    //   void *foreign_update_mask_data;
    // #endif

    // #ifdef COGL_HAS_WIN32_SUPPORT
    //   HWND foreign_hwnd;
    // #endif

    // #ifdef COGL_HAS_EGL_PLATFORM_WAYLAND_SUPPORT
    //   struct wl_surface *foreign_surface;
    // #endif

    // #ifdef COGL_HAS_EGL_PLATFORM_MIR_SUPPORT
    //   struct MirSurface *foreign_surface;
    // #endif

    //   Bool swap_throttled;

    //   List frame_closures;

    //   Bool resizable;
    //   List resize_closures;

    //   List dirty_closures;

    //   int64_t frame_counter;
    //   int64_t swap_frame_counter; /* frame counter at last all to
    //                                * onscreen_swap_region() or
    //                                * onscreen_swap_buffers() */
    //   GQueue pending_frame_infos;

    //   void *winsys;
}

impl Onscreen {
    /// Instantiates an "unallocated" `Onscreen` framebuffer that may be
    /// configured before later being allocated, either implicitly when
    /// it is first used or explicitly via `Framebuffer::allocate`.
    /// ## `context`
    /// A `Context`
    /// ## `width`
    /// The desired framebuffer width
    /// ## `height`
    /// The desired framebuffer height
    ///
    /// # Returns
    ///
    /// A newly instantiated `Onscreen` framebuffer
    pub fn new(context: &Context, width: i32, height: i32) -> Onscreen {
        // Onscreen *onscreen = g_new0 (Onscreen, 1);

        // _COGL_GET_CONTEXT (ctx, NULL);

        // _framebuffer_init (COGL_FRAMEBUFFER (onscreen),
        //                         ctx,
        //                         COGL_FRAMEBUFFER_TYPE_ONSCREEN,
        //                         0x1eadbeef, /* width */
        //                         0x1eadbeef); /* height */
        // /* NB: make sure to pass positive width/height numbers here
        // * because otherwise we'll hit input validation assertions!*/
        // _onscreen_init_from_template (onscreen, ctx->display->onscreen_template);

        // COGL_FRAMEBUFFER (onscreen)->allocated = true;

        // /* XXX: Note we don't initialize onscreen->winsys in this case. */
        // return _onscreen_object_new (onscreen);
        unimplemented!()
    }

    /// Installs a `callback` function that will be called whenever the
    /// window system has lost the contents of a region of the onscreen
    /// buffer and the application should redraw it to repair the buffer.
    /// For example this may happen in a window system without a compositor
    /// if a window that was previously covering up the onscreen window has
    /// been moved causing a region of the onscreen to be exposed.
    ///
    /// The `callback` will be passed a `OnscreenDirtyInfo` struct which
    /// decribes a rectangle containing the newly dirtied region. Note that
    /// this may be called multiple times to describe a non-rectangular
    /// region composed of multiple smaller rectangles.
    ///
    /// The dirty events are separate from `FrameEvent::Sync` events so
    /// the application should also listen for this event before rendering
    /// the dirty region to ensure that the framebuffer is actually ready
    /// for rendering.
    /// ## `callback`
    /// A callback function to call for dirty events
    /// ## `user_data`
    /// A private pointer to be passed to `callback`
    ///
    /// # Returns
    ///
    /// a `OnscreenDirtyClosure` pointer that can be used to
    ///  remove the callback and associated `user_data` later.
    pub fn add_dirty_callback<P: Fn(&Onscreen, &OnscreenDirtyInfo) + 'static>(
        &self,
        callback: P,
    ) -> Option<OnscreenDirtyClosure> {
        // return _closure_list_add (&onscreen->dirty_closures,
        //     callback,
        //     user_data,
        //     destroy);
        unimplemented!()
    }

    /// Installs a `callback` function that will be called for significant
    /// events relating to the given `self` framebuffer.
    ///
    /// The `callback` will be used to notify when the system compositor is
    /// ready for this application to render a new frame. In this case
    /// `FrameEvent::Sync` will be passed as the event argument to the
    /// given `callback` in addition to the `FrameInfo` corresponding to
    /// the frame beeing acknowledged by the compositor.
    ///
    /// The `callback` will also be called to notify when the frame has
    /// ended. In this case `FrameEvent::Complete` will be passed as
    /// the event argument to the given `callback` in addition to the
    /// `FrameInfo` corresponding to the newly presented frame. The
    /// meaning of "ended" here simply means that no more timing
    /// information will be collected within the corresponding
    /// `FrameInfo` and so this is a good opportunity to analyse the
    /// given info. It does not necessarily mean that the GPU has finished
    /// rendering the corresponding frame.
    ///
    /// We highly recommend throttling your application according to
    /// `FrameEvent::Sync` events so that your application can avoid
    /// wasting resources, drawing more frames than your system compositor
    /// can display.
    /// ## `callback`
    /// A callback function to call for frame events
    /// ## `user_data`
    /// A private pointer to be passed to `callback`
    ///
    /// # Returns
    ///
    /// a `FrameClosure` pointer that can be used to
    ///  remove the callback and associated `user_data` later.
    pub fn add_frame_callback<P: Fn(&Onscreen, &FrameEvent, &FrameInfo) + 'static>(
        &self,
        callback: P,
    ) -> Option<FrameClosure> {
        // return _closure_list_add (&onscreen->frame_closures,
        //     callback,
        //     user_data,
        //     destroy);
        unimplemented!()
    }

    /// Registers a `callback` with `self` that will be called whenever
    /// the `self` framebuffer changes size.
    ///
    /// The `callback` can be removed using
    /// `Onscreen::remove_resize_callback` passing the returned closure
    /// pointer.
    ///
    /// `<note>`Since  automatically updates the viewport of an `self`
    /// framebuffer that is resized, a resize callback can also be used to
    /// track when the viewport has been changed automatically by  in
    /// case your application needs more specialized control over the
    /// viewport.`</note>`
    ///
    /// `<note>`A resize callback will only ever be called while dispatching
    ///  events from the system mainloop; so for example during
    /// `poll_renderer_dispatch`. This is so that callbacks shouldn't
    /// occur while an application might have arbitrary locks held for
    /// example.`</note>`
    ///
    /// ## `callback`
    /// A `OnscreenResizeCallback` to call when
    ///  the `self` changes size.
    /// ## `user_data`
    /// Private data to be passed to `callback`.
    /// ## `destroy`
    ///
    /// # Returns
    ///
    /// a `OnscreenResizeClosure` pointer that can be used to
    ///  remove the callback and associated `user_data` later.
    pub fn add_resize_callback<P: Fn(&Onscreen, i32, i32) + 'static>(
        &self,
        callback: P,
    ) -> Option<OnscreenResizeClosure> {
        // return _closure_list_add (&onscreen->resize_closures,
        //     callback,
        //     user_data,
        //     destroy);
        unimplemented!()
    }

    /// Gets the current age of the buffer contents.
    ///
    /// This function allows applications to query the age of the current
    /// back buffer contents for a `Onscreen` as the number of frames
    /// elapsed since the contents were most recently defined.
    ///
    /// These age values exposes enough information to applications about
    /// how  internally manages back buffers to allow applications to
    /// re-use the contents of old frames and minimize how much must be
    /// redrawn for the next frame.
    ///
    /// The back buffer contents can either be reported as invalid (has an
    /// age of 0) or it may be reported to be the same contents as from n
    /// frames prior to the current frame.
    ///
    /// The queried value remains valid until the next buffer swap.
    ///
    /// `<note>`One caveat is that under X11 the buffer age does not reflect
    /// changes to buffer contents caused by the window systems. X11
    /// applications must track Expose events to determine what buffer
    /// regions need to additionally be repaired each frame.`</note>`
    ///
    /// The recommended way to take advantage of this buffer age api is to
    /// build up a circular buffer of length 3 for tracking damage regions
    /// over the last 3 frames and when starting a new frame look at the
    /// age of the buffer and combine the damage regions for the current
    /// frame with the damage regions of previous `age` frames so you know
    /// everything that must be redrawn to update the old contents for the
    /// new frame.
    ///
    /// `<note>`If the system doesn't not support being able to track the age
    /// of back buffers then this function will always return 0 which
    /// implies that the contents are undefined.`</note>`
    ///
    /// `<note>`The `FeatureID::OglFeatureIdBufferAge` feature can optionally be
    /// explicitly checked to determine if  is currently tracking the
    /// age of `Onscreen` back buffer contents. If this feature is
    /// missing then this function will always return 0.`</note>`
    ///
    /// # Returns
    ///
    /// The age of the buffer contents or 0 when the buffer
    ///  contents are undefined.
    pub fn get_buffer_age(&self) -> i32 {
        // Framebuffer *framebuffer = COGL_FRAMEBUFFER (onscreen);
        // const WinsysVtable *winsys;

        // _COGL_RETURN_VAL_IF_FAIL  (framebuffer->type == COGL_FRAMEBUFFER_TYPE_ONSCREEN, 0);

        // winsys = _framebuffer_get_winsys (framebuffer);

        // if (!winsys->onscreen_get_buffer_age)
        //     return 0;

        // return winsys->onscreen_get_buffer_age (onscreen);
        unimplemented!()
    }

    /// Gets the value of the framebuffers frame counter. This is
    /// a counter that increases by one each time
    /// `Onscreen::swap_buffers` or `Onscreen::swap_region`
    /// is called.
    ///
    /// # Returns
    ///
    /// the current frame counter value
    pub fn get_frame_counter(&self) -> i64 {
        // return onscreen->frame_counter;
        unimplemented!()
    }

    /// Lets you query whether `self` has been marked as resizable via
    /// the `Onscreen::set_resizable` api.
    ///
    /// By default, if possible, a `self` will be created by
    /// as non resizable, but it is not guaranteed that this is always
    /// possible for all window systems.
    ///
    /// `<note>`If onscreen_set_resizable(`self`, `true`) has been
    /// previously called then this function will return `true`, but it's
    /// possible that the current windowing system being used does not
    /// support window resizing (consider fullscreen windows on a phone or
    /// a TV). This function is not aware of whether resizing is truly
    /// meaningful with your window system, only whether the `self` has
    /// been marked as resizable.`</note>`
    ///
    ///
    /// # Returns
    ///
    /// Returns whether `self` has been marked as
    ///  resizable or not.
    pub fn get_resizable(&self) -> bool {
        // return onscreen->resizable;
        unimplemented!()
    }

    /// This requests to make `self` invisible to the user.
    ///
    /// Actually the precise semantics of this function depend on the
    /// window system currently in use, and if you don't have a
    /// multi-windowining system this function may in-fact do nothing.
    ///
    /// This function does not implicitly allocate the given `self`
    /// framebuffer before hiding it.
    ///
    /// `<note>`Since  doesn't explicitly track the visibility status of
    /// onscreen framebuffers it wont try to avoid redundant window system
    /// requests e.g. to show an already visible window. This also means
    /// that it's acceptable to alternatively use native APIs to show and
    /// hide windows without confusing .`</note>`
    ///
    pub fn hide(&self) {
        // Framebuffer *framebuffer = COGL_FRAMEBUFFER (onscreen);

        // if (framebuffer->allocated)
        // {
        // const WinsysVtable *winsys =
        //     _framebuffer_get_winsys (framebuffer);
        // if (winsys->onscreen_set_visibility)
        //     winsys->onscreen_set_visibility (onscreen, false);
        // }
        unimplemented!()
    }

    /// Removes a callback and associated user data that were previously
    /// registered using `Onscreen::add_dirty_callback`.
    ///
    /// If a destroy callback was passed to
    /// `Onscreen::add_dirty_callback` to destroy the user data then
    /// this will also get called.
    /// ## `closure`
    /// A `OnscreenDirtyClosure` returned from
    ///  `Onscreen::add_dirty_callback`
    pub fn remove_dirty_callback(&self, closure: &mut OnscreenDirtyClosure) {
        // _COGL_RETURN_IF_FAIL (closure);

        // _closure_disconnect (closure);
        unimplemented!()
    }

    /// Removes a callback and associated user data that were previously
    /// registered using `Onscreen::add_frame_callback`.
    ///
    /// If a destroy callback was passed to
    /// `Onscreen::add_frame_callback` to destroy the user data then
    /// this will get called.
    /// ## `closure`
    /// A `FrameClosure` returned from
    ///  `Onscreen::add_frame_callback`
    pub fn remove_frame_callback(&self, closure: &mut FrameClosure) {
        // _COGL_RETURN_IF_FAIL (closure);

        // _closure_disconnect (closure);
        unimplemented!()
    }

    /// Removes a resize `callback` and `user_data` pair that were previously
    /// associated with `self` via `Onscreen::add_resize_callback`.
    ///
    /// ## `closure`
    /// An identifier returned from `Onscreen::add_resize_callback`
    pub fn remove_resize_callback(&self, closure: &mut OnscreenResizeClosure) {
        // _closure_disconnect (closure);
        unimplemented!()
    }

    /// Lets you request  to mark an `self` framebuffer as
    /// resizable or not.
    ///
    /// By default, if possible, a `self` will be created by
    /// as non resizable, but it is not guaranteed that this is always
    /// possible for all window systems.
    ///
    /// `<note>` does not know whether marking the `self` framebuffer
    /// is truly meaningful for your current window system (consider
    /// applications being run fullscreen on a phone or TV) so this
    /// function may not have any useful effect. If you are running on a
    /// multi windowing system such as X11 or Win32 or OSX then  will
    /// request to the window system that users be allowed to resize the
    /// `self`, although it's still possible that some other window
    /// management policy will block this possibility.`</note>`
    ///
    /// `<note>`Whenever an `self` framebuffer is resized the viewport
    /// will be automatically updated to match the new size of the
    /// framebuffer with an origin of (0,0). If your application needs more
    /// specialized control of the viewport it will need to register a
    /// resize handler using `Onscreen::add_resize_callback` so that it
    /// can track when the viewport has been changed automatically.`</note>`
    ///
    pub fn set_resizable(&self, resizable: bool) {
        // Framebuffer *framebuffer;
        // const WinsysVtable *winsys;

        // if (onscreen->resizable == resizable)
        //     return;

        // onscreen->resizable = resizable;

        // framebuffer = COGL_FRAMEBUFFER (onscreen);
        // if (framebuffer->allocated)
        //     {
        //     winsys = _framebuffer_get_winsys (COGL_FRAMEBUFFER (onscreen));

        //     if (winsys->onscreen_set_resizable)
        //         winsys->onscreen_set_resizable (onscreen, resizable);
        //     }
        unimplemented!()
    }

    /// Requests that the given `self` framebuffer should have swap buffer
    /// requests (made using `Onscreen::swap_buffers`) throttled either by a
    /// displays vblank period or perhaps some other mechanism in a composited
    /// environment.
    /// ## `throttled`
    /// Whether swap throttling is wanted or not.
    pub fn set_swap_throttled(&self, throttled: bool) {
        // Framebuffer *framebuffer = COGL_FRAMEBUFFER (onscreen);
        // framebuffer->config.swap_throttled = throttled;
        // if (framebuffer->allocated)
        // {
        //     const WinsysVtable *winsys =
        //         _framebuffer_get_winsys (framebuffer);
        //     winsys->onscreen_update_swap_throttled (onscreen);
        // }
        unimplemented!()
    }

    /// This requests to make `self` visible to the user.
    ///
    /// Actually the precise semantics of this function depend on the
    /// window system currently in use, and if you don't have a
    /// multi-windowining system this function may in-fact do nothing.
    ///
    /// This function will implicitly allocate the given `self`
    /// framebuffer before showing it if it hasn't already been allocated.
    ///
    /// When using the Wayland winsys calling this will set the surface to
    /// a toplevel type which will make it appear. If the application wants
    /// to set a different type for the surface, it can avoid calling
    /// `Onscreen::show` and set its own type directly with the Wayland
    /// client API via `wayland_onscreen_get_surface`.
    ///
    /// `<note>`Since  doesn't explicitly track the visibility status of
    /// onscreen framebuffers it wont try to avoid redundant window system
    /// requests e.g. to show an already visible window. This also means
    /// that it's acceptable to alternatively use native APIs to show and
    /// hide windows without confusing .`</note>`
    ///
    pub fn show(&self) {
        // Framebuffer *framebuffer = COGL_FRAMEBUFFER (onscreen);
        // const WinsysVtable *winsys;

        // if (!framebuffer->allocated)
        //     {
        //     if (!framebuffer_allocate (framebuffer, NULL))
        //         return;
        //     }

        // winsys = _framebuffer_get_winsys (framebuffer);
        // if (winsys->onscreen_set_visibility)
        //     winsys->onscreen_set_visibility (onscreen, true);
        unimplemented!()
    }

    /// Swaps the current back buffer being rendered too, to the front for display.
    ///
    /// This function also implicitly discards the contents of the color, depth and
    /// stencil buffers as if `Framebuffer::discard_buffers` were used. The
    /// significance of the discard is that you should not expect to be able to
    /// start a new frame that incrementally builds on the contents of the previous
    /// frame.
    ///
    /// `<note>`It is highly recommended that applications use
    /// `Onscreen::swap_buffers_with_damage` instead whenever possible
    /// and also use the `Onscreen::get_buffer_age` api so they can
    /// perform incremental updates to older buffers instead of having to
    /// render a full buffer for every frame.`</note>`
    pub fn swap_buffers(&self) {
        // onscreen_swap_buffers_with_damage (onscreen, NULL, 0);
        unimplemented!()
    }

    /// Swaps the current back buffer being rendered too, to the front for
    /// display and provides information to any system compositor about
    /// what regions of the buffer have changed (damage) with respect to
    /// the last swapped buffer.
    ///
    /// This function has the same semantics as
    /// `framebuffer_swap_buffers` except that it additionally allows
    /// applications to pass a list of damaged rectangles which may be
    /// passed on to a compositor so that it can minimize how much of the
    /// screen is redrawn in response to this applications newly swapped
    /// front buffer.
    ///
    /// For example if your application is only animating a small object in
    /// the corner of the screen and everything else is remaining static
    /// then it can help the compositor to know that only the bottom right
    /// corner of your newly swapped buffer has really changed with respect
    /// to your previously swapped front buffer.
    ///
    /// If `n_rectangles` is 0 then the whole buffer will implicitly be
    /// reported as damaged as if `Onscreen::swap_buffers` had been
    /// called.
    ///
    /// This function also implicitly discards the contents of the color,
    /// depth and stencil buffers as if `Framebuffer::discard_buffers`
    /// were used. The significance of the discard is that you should not
    /// expect to be able to start a new frame that incrementally builds on
    /// the contents of the previous frame. If you want to perform
    /// incremental updates to older back buffers then please refer to the
    /// `Onscreen::get_buffer_age` api.
    ///
    /// Whenever possible it is recommended that applications use this
    /// function instead of `Onscreen::swap_buffers` to improve
    /// performance when running under a compositor.
    ///
    /// `<note>`It is highly recommended to use this API in conjunction with
    /// the `Onscreen::get_buffer_age` api so that your application can
    /// perform incremental rendering based on old back buffers.`</note>`
    /// ## `rectangles`
    /// An array of integer 4-tuples representing damaged
    ///  rectangles as (x, y, width, height) tuples.
    /// ## `n_rectangles`
    /// The number of 4-tuples to be read from `rectangles`
    pub fn swap_buffers_with_damage(&self, rectangles: &[i32], n_rectangles: i32) {
        // Framebuffer *framebuffer = COGL_FRAMEBUFFER (onscreen);
        // const WinsysVtable *winsys;
        // FrameInfo *info;

        // _COGL_RETURN_IF_FAIL  (framebuffer->type == COGL_FRAMEBUFFER_TYPE_ONSCREEN);

        // info = _frame_info_new ();
        // info->frame_counter = onscreen->frame_counter;
        // g_queue_push_tail (&onscreen->pending_frame_infos, info);

        // /* FIXME: we shouldn't need to flush *all* journals here! */
        // flush ();

        // winsys = _framebuffer_get_winsys (framebuffer);
        // winsys->onscreen_swap_buffers_with_damage (onscreen,
        //                                             rectangles, n_rectangles);
        // framebuffer_discard_buffers (framebuffer,
        //                                     COGL_BUFFER_BIT_COLOR |
        //                                     COGL_BUFFER_BIT_DEPTH |
        //                                     COGL_BUFFER_BIT_STENCIL);

        // if (!_winsys_has_feature (COGL_WINSYS_FEATURE_SYNC_AND_COMPLETE_EVENT))
        //     {
        //     FrameInfo *info;

        //     g_warn_if_fail (onscreen->pending_frame_infos.length == 1);

        //     info = g_queue_pop_tail (&onscreen->pending_frame_infos);

        //     _onscreen_queue_event (onscreen, COGL_FRAME_EVENT_SYNC, info);
        //     _onscreen_queue_event (onscreen, COGL_FRAME_EVENT_COMPLETE, info);

        //     object_unref (info);
        //     }

        // onscreen->frame_counter++;
        // framebuffer->mid_scene = false;
        unimplemented!()
    }

    /// Swaps a region of the back buffer being rendered too, to the front for
    /// display. `rectangles` represents the region as array of `n_rectangles` each
    /// defined by 4 sequential (x, y, width, height) integers.
    ///
    /// This function also implicitly discards the contents of the color, depth and
    /// stencil buffers as if `Framebuffer::discard_buffers` were used. The
    /// significance of the discard is that you should not expect to be able to
    /// start a new frame that incrementally builds on the contents of the previous
    /// frame.
    /// ## `rectangles`
    /// An array of integer 4-tuples representing rectangles as
    ///  (x, y, width, height) tuples.
    /// ## `n_rectangles`
    /// The number of 4-tuples to be read from `rectangles`
    pub fn swap_region(&self, rectangles: &[i32], n_rectangles: i32) {
        // Framebuffer *framebuffer = COGL_FRAMEBUFFER (onscreen);
        // const WinsysVtable *winsys;
        // FrameInfo *info;

        // _COGL_RETURN_IF_FAIL  (framebuffer->type == COGL_FRAMEBUFFER_TYPE_ONSCREEN);

        // info = _frame_info_new ();
        // info->frame_counter = onscreen->frame_counter;
        // g_queue_push_tail (&onscreen->pending_frame_infos, info);

        // /* FIXME: we shouldn't need to flush *all* journals here! */
        // flush ();

        // winsys = _framebuffer_get_winsys (framebuffer);

        // /* This should only be called if the winsys advertises
        //     COGL_WINSYS_FEATURE_SWAP_REGION */
        // _COGL_RETURN_IF_FAIL (winsys->onscreen_swap_region != NULL);

        // winsys->onscreen_swap_region (COGL_ONSCREEN (framebuffer),
        //                                 rectangles,
        //                                 n_rectangles);

        // framebuffer_discard_buffers (framebuffer,
        //                                     COGL_BUFFER_BIT_COLOR |
        //                                     COGL_BUFFER_BIT_DEPTH |
        //                                     COGL_BUFFER_BIT_STENCIL);

        // if (!_winsys_has_feature (COGL_WINSYS_FEATURE_SYNC_AND_COMPLETE_EVENT))
        //     {
        //     FrameInfo *info;

        //     g_warn_if_fail (onscreen->pending_frame_infos.length == 1);

        //     info = g_queue_pop_tail (&onscreen->pending_frame_infos);

        //     _onscreen_queue_event (onscreen, COGL_FRAME_EVENT_SYNC, info);
        //     _onscreen_queue_event (onscreen, COGL_FRAME_EVENT_COMPLETE, info);

        //     object_unref (info);
        //     }

        // onscreen->frame_counter++;
        // framebuffer->mid_scene = false;
        unimplemented!()
    }
}

impl fmt::Display for Onscreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Onscreen")
    }
}
