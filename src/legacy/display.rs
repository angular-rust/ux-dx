use crate::{Object, OnscreenTemplate, Renderer};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Display(Object<ffi::CoglDisplay, DisplayClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_display_get_gtype(),
    }
}

impl Display {
    /// Explicitly allocates a new `Display` object to encapsulate the
    /// common state of the display pipeline that applies to the whole
    /// application.
    ///
    /// `<note>`Many applications don't need to explicitly use
    /// `Display::new` and can just jump straight to `Context::new`
    /// and pass a `None` display argument so Cogl will automatically
    /// connect and setup a renderer and display.`</note>`
    ///
    /// A `display` can only be made for a specific choice of renderer which
    /// is why this takes the `renderer` argument.
    ///
    /// A common use for explicitly allocating a display object is to
    /// define a template for allocating onscreen framebuffers which is
    /// what the `onscreen_template` argument is for, or alternatively
    /// you can use `Display::set_onscreen_template`.
    ///
    /// When a display is first allocated via `Display::new` it is in a
    /// mutable configuration mode. It's designed this way so we can
    /// extend the apis available for configuring a display without
    /// requiring huge numbers of constructor arguments.
    ///
    /// When you have finished configuring a display object you can
    /// optionally call `Display::setup` to explicitly apply the
    /// configuration and check for errors. Alternaitvely you can pass the
    /// display to `Context::new` and Cogl will implicitly apply your
    /// configuration but if there are errors then the application will
    /// abort with a message. For simple applications with no fallback
    /// options then relying on the implicit setup can be fine.
    /// ## `renderer`
    /// A `Renderer`
    /// ## `onscreen_template`
    /// A `OnscreenTemplate`
    ///
    /// # Returns
    ///
    /// A newly allocated `Display`
    ///  object in a mutable configuration mode.
    pub fn new(renderer: &Renderer, onscreen_template: &OnscreenTemplate) -> Display {
        unsafe {
            from_glib_full(ffi::cogl_display_new(
                renderer.to_glib_none().0,
                onscreen_template.to_glib_none().0,
            ))
        }
    }

    /// Queries the `Renderer` associated with the given `self`.
    ///
    /// # Returns
    ///
    /// The associated `Renderer`
    pub fn get_renderer(&self) -> Option<Renderer> {
        unsafe { from_glib_none(ffi::cogl_display_get_renderer(self.to_glib_none().0)) }
    }

    /// Specifies a template for creating `Onscreen` framebuffers.
    ///
    /// Depending on the system, the constraints for creating `Onscreen`
    /// framebuffers need to be known before setting up a `Display` because the
    /// final setup of the display may constrain how onscreen framebuffers may be
    /// allocated. If Cogl knows how an application wants to allocate onscreen
    /// framebuffers then it can try to make sure to setup the display accordingly.
    /// ## `onscreen_template`
    /// A template for creating `Onscreen` framebuffers
    pub fn set_onscreen_template(&self, onscreen_template: &OnscreenTemplate) {
        unsafe {
            ffi::cogl_display_set_onscreen_template(
                self.to_glib_none().0,
                onscreen_template.to_glib_none().0,
            );
        }
    }

    /// Explicitly sets up the given `self` object. Use of this api is
    /// optional since Cogl will internally setup the display if not done
    /// explicitly.
    ///
    /// When a display is first allocated via `Display::new` it is in a
    /// mutable configuration mode. This allows us to extend the apis
    /// available for configuring a display without requiring huge numbers
    /// of constructor arguments.
    ///
    /// Its possible to request a configuration that might not be
    /// supportable on the current system and so this api provides a means
    /// to apply the configuration explicitly but if it fails then an
    /// exception will be returned so you can handle the error gracefully
    /// and perhaps fall back to an alternative configuration.
    ///
    /// If you instead rely on Cogl implicitly calling `Display::setup`
    /// for you then if there is an error with the configuration you won't
    /// get an opportunity to handle that and the application may abort
    /// with a message. For simple applications that don't have any
    /// fallback options this behaviour may be fine.
    ///
    /// # Returns
    ///
    /// Returns `true` if there was no error, else it returns
    ///  `false` and returns an exception via `error`.
    pub fn setup(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_display_setup(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display")
    }
}
