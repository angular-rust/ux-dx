use crate::{Display, Object, Renderer};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Context(Object<ffi::CoglContext, ContextClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_context_get_gtype(),
    }
}

impl Context {
    /// Creates a new `Context` which acts as an application sandbox
    /// for any state objects that are allocated.
    /// ## `display`
    /// A `Display` pointer
    ///
    /// # Returns
    ///
    /// A newly allocated `Context`
    pub fn new(display: Option<&Display>) -> Result<Context, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_context_new(display.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Retrieves the `Display` that is internally associated with the
    /// given `self`. This will return the same `Display` that was
    /// passed to `Context::new` or if `None` was passed to
    /// `Context::new` then this function returns a pointer to the
    /// display that was automatically setup internally.
    ///
    /// # Returns
    ///
    /// The `Display` associated with the
    ///  given `self`.
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::cogl_context_get_display(self.to_glib_none().0)) }
    }

    /// Retrieves the `Renderer` that is internally associated with the
    /// given `self`. This will return the same `Renderer` that was
    /// passed to `Display::new` or if `None` was passed to
    /// `Display::new` or `Context::new` then this function returns
    /// a pointer to the renderer that was automatically connected
    /// internally.
    ///
    /// # Returns
    ///
    /// The `Renderer` associated with the
    ///  given `self`.
    pub fn get_renderer(&self) -> Option<Renderer> {
        unsafe { from_glib_none(ffi::cogl_context_get_renderer(self.to_glib_none().0)) }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context")
    }
}
