use crate::{Context, Object};

use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct GLES2Context(Object<ffi::CoglGLES2Context, GLES2ContextClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_gles2_context_get_gtype(),
    }
}

impl GLES2Context {
    /// Allocates a new OpenGLES 2.0 context that can be used to render to
    /// `CoglOffscreen` framebuffers (Rendering to `Onscreen`
    /// framebuffers is not currently supported).
    ///
    /// To actually access the OpenGLES 2.0 api itself you need to use
    /// `GLES2Context::get_vtable`. You should not try to directly link
    /// to and use the symbols provided by the a system OpenGLES 2.0
    /// driver.
    ///
    /// Once you have allocated an OpenGLES 2.0 context you can make it
    /// current using `cogl_push_gles2_context`. For those familiar with
    /// using the EGL api, this serves a similar purpose to eglMakeCurrent.
    ///
    /// `<note>`Before using this api applications can check for OpenGLES 2.0
    /// api support by checking for `FeatureID::OglFeatureIdGles2Context` support
    /// with `cogl_has_feature`. This function will return `false` and
    /// return an `GLES2ContextError::Unsupported` error if the
    /// feature isn't available.`</note>`
    ///
    /// ## `ctx`
    /// A `Context`
    ///
    /// # Returns
    ///
    /// A newly allocated `GLES2Context` or `None` if there
    ///  was an error and `error` will be updated in that case.
    pub fn new(ctx: &Context) -> Result<GLES2Context, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_gles2_context_new(ctx.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    // /// Queries the OpenGLES 2.0 api function pointers that should be
    // /// used for rendering with the given `self`.
    // ///
    // /// `<note>`You should not try to directly link to and use the symbols
    // /// provided by any system OpenGLES 2.0 driver.`</note>`
    // ///
    // ///
    // /// # Returns
    // ///
    // /// A pointer to a `GLES2Vtable` providing pointers
    // ///  to functions for the full OpenGLES 2.0 api.
    // pub fn get_vtable(&self) -> Option<GLES2Vtable> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_gles2_context_get_vtable(
    //             self.to_glib_none().0,
    //         ))
    //     }
    // }
}

impl fmt::Display for GLES2Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLES2Context")
    }
}
