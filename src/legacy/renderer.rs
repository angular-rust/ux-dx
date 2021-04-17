#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Driver, Object, OnscreenTemplate, Output, RendererConstraint, WinsysID};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Renderer(Object<ffi::CoglRenderer, RendererClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_renderer_get_gtype(),
    }
}

impl Renderer {
    /// Instantiates a new (unconnected) `Renderer` object. A
    /// `Renderer` represents a means to render. It encapsulates the
    /// selection of an underlying driver, such as OpenGL or OpenGL-ES and
    /// a selection of a window system binding API such as GLX, or EGL or
    /// WGL.
    ///
    /// While the renderer is unconnected it can be configured so that
    /// applications may specify backend constraints, such as "must use
    /// x11" for example via `Renderer::add_constraint`.
    ///
    /// There are also some platform specific configuration apis such
    /// as `xlib_renderer_set_foreign_display` that may also be
    /// used while the renderer is unconnected.
    ///
    /// Once the renderer has been configured, then it may (optionally) be
    /// explicitly connected using `Renderer::connect` which allows
    /// errors to be handled gracefully and potentially fallback
    /// configurations can be tried out if there are initial failures.
    ///
    /// If a renderer is not explicitly connected then `Display::new`
    /// will automatically connect the renderer for you. If you don't
    /// have any code to deal with error/fallback situations then its fine
    /// to just let Cogl do the connection for you.
    ///
    /// Once you have setup your renderer then the next step is to create a
    /// `Display` using `Display::new`.
    ///
    /// `<note>`Many applications don't need to explicitly use
    /// `Renderer::new` or `Display::new` and can just jump
    /// straight to `Context::new` and pass a `None` display argument
    /// so Cogl will automatically connect and setup a renderer and
    /// display.`</note>`
    ///
    /// # Returns
    ///
    /// A newly created `Renderer`.
    pub fn new() -> Renderer {
        unsafe { from_glib_full(ffi::cogl_renderer_new()) }
    }

    /// This adds a renderer selection `constraint`.
    ///
    /// Applications should ideally minimize how many of these constraints they
    /// depend on to ensure maximum portability.
    /// ## `constraint`
    /// A `RendererConstraint` to add
    pub fn add_constraint(&self, constraint: RendererConstraint) {
        unsafe {
            ffi::cogl_renderer_add_constraint(self.to_glib_none().0, constraint.to_glib());
        }
    }

    /// Tests if a given `onscreen_template` can be supported with the given
    /// `self`.
    /// ## `onscreen_template`
    /// A `OnscreenTemplate`
    ///
    /// # Returns
    ///
    /// `true` if the `onscreen_template` can be supported,
    ///  else `false`.
    pub fn check_onscreen_template(
        &self,
        onscreen_template: &OnscreenTemplate,
    ) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_renderer_check_onscreen_template(
                self.to_glib_none().0,
                onscreen_template.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Connects the configured `self`. Renderer connection isn't a
    /// very active process, it basically just means validating that
    /// any given constraint criteria can be satisfied and that a
    /// usable driver and window system backend can be found.
    ///
    /// # Returns
    ///
    /// `true` if there was no error while connecting the
    ///  given `self`. `false` if there was an error.
    pub fn connect(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_renderer_connect(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Iterates all known display outputs for the given `self` and
    /// passes a corresponding `Output` pointer to the given `callback`
    /// for each one, along with the given `user_data`.
    /// ## `callback`
    /// A `CoglOutputCallback` to be called for
    ///  each display output
    /// ## `user_data`
    /// A user pointer to be passed to `callback`
    pub fn foreach_output<P: FnMut(&Output)>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Output)>(
            output: *mut ffi::CoglOutput,
            user_data: glib_sys::gpointer,
        ) {
            let output = from_glib_borrow(output);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&output);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::cogl_renderer_foreach_output(
                self.to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    /// Queries what underlying driver is being used by Cogl.
    ///
    /// This may only be called on a connected `Renderer`.
    pub fn get_driver(&self) -> Driver {
        unsafe { from_glib(ffi::cogl_renderer_get_driver(self.to_glib_none().0)) }
    }

    /// Queries how many texture units can be used from fragment programs
    ///
    /// # Returns
    ///
    /// the number of texture image units.
    pub fn get_n_fragment_texture_units(&self) -> i32 {
        unsafe { ffi::cogl_renderer_get_n_fragment_texture_units(self.to_glib_none().0) }
    }

    /// Queries which window system backend Cogl has chosen to use.
    ///
    /// This may only be called on a connected `Renderer`.
    ///
    /// # Returns
    ///
    /// The `WinsysID` corresponding to the chosen window
    ///  system backend.
    pub fn get_winsys_id(&self) -> WinsysID {
        unsafe { from_glib(ffi::cogl_renderer_get_winsys_id(self.to_glib_none().0)) }
    }

    /// This removes a renderer selection `constraint`.
    ///
    /// Applications should ideally minimize how many of these constraints they
    /// depend on to ensure maximum portability.
    /// ## `constraint`
    /// A `RendererConstraint` to remove
    pub fn remove_constraint(&self, constraint: RendererConstraint) {
        unsafe {
            ffi::cogl_renderer_remove_constraint(self.to_glib_none().0, constraint.to_glib());
        }
    }

    /// Requests that Cogl should try to use a specific underlying driver
    /// for rendering.
    ///
    /// If you select an unsupported driver then `Renderer::connect`
    /// will fail and report an error. Most applications should not
    /// explicitly select a driver and should rely on Cogl automatically
    /// choosing the driver.
    ///
    /// This may only be called on an un-connected `Renderer`.
    pub fn set_driver(&self, driver: Driver) {
        unsafe {
            ffi::cogl_renderer_set_driver(self.to_glib_none().0, driver.to_glib());
        }
    }

    /// This allows you to explicitly select a winsys backend to use instead
    /// of letting Cogl automatically select a backend.
    ///
    /// if you select an unsupported backend then `Renderer::connect`
    /// will fail and report an error.
    ///
    /// This may only be called on an un-connected `Renderer`.
    /// ## `winsys_id`
    /// An ID of the winsys you explicitly want to use.
    pub fn set_winsys_id(&self, winsys_id: WinsysID) {
        unsafe {
            ffi::cogl_renderer_set_winsys_id(self.to_glib_none().0, winsys_id.to_glib());
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Renderer")
    }
}
