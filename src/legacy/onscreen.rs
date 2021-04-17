use crate::{
    Context, FrameClosure, FrameEvent, FrameInfo, Framebuffer, Object, OnscreenDirtyClosure,
    OnscreenDirtyInfo, OnscreenResizeClosure,
};

use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;

glib_wrapper! {
    pub struct Onscreen(Object<ffi::CoglOnscreen, OnscreenClass>) @extends Object, @implements Framebuffer;

    match fn {
        get_type => || ffi::cogl_onscreen_get_gtype(),
    }
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
        unsafe {
            from_glib_full(ffi::cogl_onscreen_new(
                context.to_glib_none().0,
                width,
                height,
            ))
        }
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
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&Onscreen, &OnscreenDirtyInfo) + 'static>(
            onscreen: *mut ffi::CoglOnscreen,
            info: *const ffi::CoglOnscreenDirtyInfo,
            user_data: glib_sys::gpointer,
        ) {
            let onscreen = from_glib_borrow(onscreen);
            let info = from_glib_borrow(info);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&onscreen, &info);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::cogl_onscreen_add_dirty_callback(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                None,
            ))
        }
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
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&Onscreen, &FrameEvent, &FrameInfo) + 'static>(
            onscreen: *mut ffi::CoglOnscreen,
            event: ffi::CoglFrameEvent,
            info: *mut ffi::CoglFrameInfo,
            user_data: glib_sys::gpointer,
        ) {
            let onscreen = from_glib_borrow(onscreen);
            let event = from_glib(event);
            let info = from_glib_borrow(info);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&onscreen, &event, &info);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::cogl_onscreen_add_frame_callback(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                None,
            ))
        }
    }

    /// Registers a `callback` with `self` that will be called whenever
    /// the `self` framebuffer changes size.
    ///
    /// The `callback` can be removed using
    /// `Onscreen::remove_resize_callback` passing the returned closure
    /// pointer.
    ///
    /// `<note>`Since Cogl automatically updates the viewport of an `self`
    /// framebuffer that is resized, a resize callback can also be used to
    /// track when the viewport has been changed automatically by Cogl in
    /// case your application needs more specialized control over the
    /// viewport.`</note>`
    ///
    /// `<note>`A resize callback will only ever be called while dispatching
    /// Cogl events from the system mainloop; so for example during
    /// `poll_renderer_dispatch`. This is so that callbacks shouldn't
    /// occur while an application might have arbitrary locks held for
    /// example.`</note>`
    ///
    /// ## `callback`
    /// A `CoglOnscreenResizeCallback` to call when
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
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&Onscreen, i32, i32) + 'static>(
            onscreen: *mut ffi::CoglOnscreen,
            width: libc::c_int,
            height: libc::c_int,
            user_data: glib_sys::gpointer,
        ) {
            let onscreen = from_glib_borrow(onscreen);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&onscreen, width, height);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::cogl_onscreen_add_resize_callback(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                None,
            ))
        }
    }

    /// Gets the current age of the buffer contents.
    ///
    /// This function allows applications to query the age of the current
    /// back buffer contents for a `Onscreen` as the number of frames
    /// elapsed since the contents were most recently defined.
    ///
    /// These age values exposes enough information to applications about
    /// how Cogl internally manages back buffers to allow applications to
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
    /// explicitly checked to determine if Cogl is currently tracking the
    /// age of `Onscreen` back buffer contents. If this feature is
    /// missing then this function will always return 0.`</note>`
    ///
    /// # Returns
    ///
    /// The age of the buffer contents or 0 when the buffer
    ///  contents are undefined.
    pub fn get_buffer_age(&self) -> i32 {
        unsafe { ffi::cogl_onscreen_get_buffer_age(self.to_glib_none().0) }
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
        unsafe { ffi::cogl_onscreen_get_frame_counter(self.to_glib_none().0) }
    }

    /// Lets you query whether `self` has been marked as resizable via
    /// the `Onscreen::set_resizable` api.
    ///
    /// By default, if possible, a `self` will be created by Cogl
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
        unsafe {
            let ret = ffi::cogl_onscreen_get_resizable(self.to_glib_none().0);
            ret == crate::TRUE
        }
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
    /// `<note>`Since Cogl doesn't explicitly track the visibility status of
    /// onscreen framebuffers it wont try to avoid redundant window system
    /// requests e.g. to show an already visible window. This also means
    /// that it's acceptable to alternatively use native APIs to show and
    /// hide windows without confusing Cogl.`</note>`
    ///
    pub fn hide(&self) {
        unsafe {
            ffi::cogl_onscreen_hide(self.to_glib_none().0);
        }
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
        unsafe {
            ffi::cogl_onscreen_remove_dirty_callback(
                self.to_glib_none().0,
                closure.to_glib_none_mut().0,
            );
        }
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
        unsafe {
            ffi::cogl_onscreen_remove_frame_callback(
                self.to_glib_none().0,
                closure.to_glib_none_mut().0,
            );
        }
    }

    /// Removes a resize `callback` and `user_data` pair that were previously
    /// associated with `self` via `Onscreen::add_resize_callback`.
    ///
    /// ## `closure`
    /// An identifier returned from `Onscreen::add_resize_callback`
    pub fn remove_resize_callback(&self, closure: &mut OnscreenResizeClosure) {
        unsafe {
            ffi::cogl_onscreen_remove_resize_callback(
                self.to_glib_none().0,
                closure.to_glib_none_mut().0,
            );
        }
    }

    /// Lets you request Cogl to mark an `self` framebuffer as
    /// resizable or not.
    ///
    /// By default, if possible, a `self` will be created by Cogl
    /// as non resizable, but it is not guaranteed that this is always
    /// possible for all window systems.
    ///
    /// `<note>`Cogl does not know whether marking the `self` framebuffer
    /// is truly meaningful for your current window system (consider
    /// applications being run fullscreen on a phone or TV) so this
    /// function may not have any useful effect. If you are running on a
    /// multi windowing system such as X11 or Win32 or OSX then Cogl will
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
        unsafe {
            ffi::cogl_onscreen_set_resizable(self.to_glib_none().0, resizable as i32);
        }
    }

    /// Requests that the given `self` framebuffer should have swap buffer
    /// requests (made using `Onscreen::swap_buffers`) throttled either by a
    /// displays vblank period or perhaps some other mechanism in a composited
    /// environment.
    /// ## `throttled`
    /// Whether swap throttling is wanted or not.
    pub fn set_swap_throttled(&self, throttled: bool) {
        unsafe {
            ffi::cogl_onscreen_set_swap_throttled(self.to_glib_none().0, throttled as i32);
        }
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
    /// `<note>`Since Cogl doesn't explicitly track the visibility status of
    /// onscreen framebuffers it wont try to avoid redundant window system
    /// requests e.g. to show an already visible window. This also means
    /// that it's acceptable to alternatively use native APIs to show and
    /// hide windows without confusing Cogl.`</note>`
    ///
    pub fn show(&self) {
        unsafe {
            ffi::cogl_onscreen_show(self.to_glib_none().0);
        }
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
        unsafe {
            ffi::cogl_onscreen_swap_buffers(self.to_glib_none().0);
        }
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
        unsafe {
            ffi::cogl_onscreen_swap_buffers_with_damage(
                self.to_glib_none().0,
                rectangles.as_ptr(),
                n_rectangles,
            );
        }
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
        unsafe {
            ffi::cogl_onscreen_swap_region(
                self.to_glib_none().0,
                rectangles.as_ptr(),
                n_rectangles,
            );
        }
    }
}

impl fmt::Display for Onscreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Onscreen")
    }
}
