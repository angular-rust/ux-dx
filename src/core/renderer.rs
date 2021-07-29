#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Driver, OnscreenTemplate, Output, RendererConstraint, WinsysID};
use std::{fmt, ptr};

// typedef struct _DriverDescription
// {
//   Driver id;
//   const char *name;
//   RendererConstraint constraints;
//   /* It would be nice to make this a pointer and then use a compound
//    * literal from C99 to initialise it but we probably can't get away
//    * with using C99 here. Instead we'll just use a fixed-size array.
//    * GCC should complain if someone adds an 8th feature to a
//    * driver. */
//   const PrivateFeature private_features[8];
//   const DriverVtable *vtable;
//   const TextureDriver *texture_driver;
//   const char *libgl_name;
// } DriverDescription;

// SECTION:renderer
// @short_description: Choosing a means to render
//
// A #Renderer represents a means to render. It encapsulates the
// selection of an underlying driver, such as OpenGL or OpenGL-ES and
// a selection of a window system binding API such as GLX, or EGL or
// WGL.
//
// A #Renderer has two states, "unconnected" and "connected". When
// a renderer is first instantiated using renderer_new() it is
// unconnected so that it can be configured and constraints can be
// specified for how the backend driver and window system should be
// chosen.
//
// After configuration a #Renderer can (optionally) be explicitly
// connected using renderer_connect() which allows for the
// handling of connection errors so that fallback configurations can
// be tried if necessary. Applications that don't support any
// fallbacks though can skip using renderer_connect() and leave
//  to automatically connect the renderer.
//
// Once you have a configured #Renderer it can be used to create a
// #Display object using display_new().
//
// <note>Many applications don't need to explicitly use
// renderer_new() or display_new() and can just jump
// straight to context_new() and pass a %NULL display argument so
//  will automatically connect and setup a renderer and
// display.</note>
#[derive(Default, Debug, Clone)]
pub struct Renderer {
    connected: bool,
    driver_override: Driver,
    // const DriverVtable *driver_vtable;
    // const TextureDriver *texture_driver;
    // const WinsysVtable *winsys_vtable;
    // WinsysID winsys_id_override;
    // GList *constraints;

    // GArray *poll_fds;
    poll_fds_age: i32,
    // GList *poll_sources;

    // List idle_closures;

    // GList *outputs;
    #[cfg(feature = "xlib")]
    foreign_xdpy: Option<Display>,
    #[cfg(feature = "xlib")]
    xlib_enable_event_retrieval: bool,

    #[cfg(feature = "win32")]
    win32_enable_event_retrieval: bool,

    driver: Driver,
    // unsigned long private_features
    //     [FLAGS_N_LONGS_FOR_SIZE (N_PRIVATE_FEATURES)];
    #[cfg(feature = "directly_linked_gl_library")]
    libgl_module: Option<GModule>,

    #[cfg(feature = "egl_platform_wayland")]
    foreign_wayland_display: Option<wl_display>,
    #[cfg(feature = "egl_platform_wayland")]
    wayland_enable_event_dispatch: bool,

    #[cfg(feature = "egl_platform_mir")]
    foreign_mir_connection: Option<MirConnection>,

    #[cfg(feature = "egl_platform_kms")]
    kms_fd: i32,

    #[cfg(feature = "sdl")]
    sdl_event_type_set: bool,
    #[cfg(feature = "sdl")]
    sdl_event_type: u32,
    // List of callback functions that will be given every native event

    // GSList *event_filters;
    // void *winsys;
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
    /// to just let  do the connection for you.
    ///
    /// Once you have setup your renderer then the next step is to create a
    /// `Display` using `Display::new`.
    ///
    /// `<note>`Many applications don't need to explicitly use
    /// `Renderer::new` or `Display::new` and can just jump
    /// straight to `Context::new` and pass a `None` display argument
    /// so  will automatically connect and setup a renderer and
    /// display.`</note>`
    ///
    /// # Returns
    ///
    /// A newly created `Renderer`.
    pub fn new() -> Renderer {
        super::init();

        let renderer = Self::default();

        // self.poll_fds = g_array_new (false, true, sizeof (PollFD));

        // _list_init (&self.idle_closures);

        // #[cfg(feature = "xlib")]
        // renderer.xlib_enable_event_retrieval = true;

        // #[cfg(feature = "win32")]
        // renderer.win32_enable_event_retrieval = true;

        // #[cfg(feature = "egl_platform_wayland")]
        // renderer.wayland_enable_event_dispatch = true;

        // #[cfg(feature = "egl_platform_kms")]
        // renderer.kms_fd = -1;

        renderer
    }

    /// This adds a renderer selection `constraint`.
    ///
    /// Applications should ideally minimize how many of these constraints they
    /// depend on to ensure maximum portability.
    /// ## `constraint`
    /// A `RendererConstraint` to add
    pub fn add_constraint(&self, constraint: RendererConstraint) {
        // g_return_if_fail (!self.connected);
        // self.constraints = g_list_prepend (self.constraints,
        //                                   GUINT_TO_POINTER (constraint));
        unimplemented!()
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
    pub fn check_onscreen_template(&self, onscreen_template: &OnscreenTemplate) -> bool {
        // Display *display;

        // if (!renderer_connect (renderer, error))
        //     return false;

        // display = display_new (renderer, onscreen_template);
        // if !display_setup(display, error) {
        //     object_unref (display);
        //     return false;
        // }

        // object_unref (display);

        // return true;
        unimplemented!()
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
    pub fn connect(&self) -> bool {
        // int i;
        // GString *error_message;
        // Bool constraints_failed = false;

        if self.connected {
            return true;
        }

        // The driver needs to be chosen before connecting the renderer
        // because eglInitialize requires the library containing the GL API
        // to be loaded before its called

        // if !_renderer_choose_driver(renderer, error) {
        //     return false;
        // }

        // error_message = g_string_new ("");
        // for (i = 0; i < G_N_ELEMENTS (_winsys_vtable_getters); i++) {
        //     const WinsysVtable *winsys = _winsys_vtable_getters[i]();
        //     Error *tmp_error = NULL;
        //     GList *l;
        //     let skip_due_to_constraints = false;

        //     if self.winsys_id_override != WINSYS_ID_ANY {
        //         if (self.winsys_id_override != winsys->id)
        //             continue;
        //     } else {
        //         char *user_choice = getenv ("RENDERER");
        //         if !user_choice {
        //             user_choice = _config_renderer;
        //         }
        //         if user_choice &&
        //             g_ascii_strcasecmp (winsys->name, user_choice) != 0 {
        //             continue;
        //         }
        //     }

        //     for (l = self.constraints; l; l = l->next) {
        //         RendererConstraint constraint = GPOINTER_TO_UINT (l->data);
        //         if !(winsys->constraints & constraint) {
        //             skip_due_to_constraints = true;
        //             break;
        //         }
        //     }
        //     if skip_due_to_constraints {
        //         constraints_failed |= true;
        //         continue;
        //     }

        // At least temporarily we will associate this winsys with
        // the renderer in-case ->renderer_connect calls API that
        // wants to query the current winsys...

        //     self.winsys_vtable = winsys;

        //     if !winsys->renderer_connect (renderer, &tmp_error) {
        //         g_string_append_c (error_message, '\n');
        //         g_string_append (error_message, tmp_error->message);
        //         error_free (tmp_error);
        //     } else {
        //         self.connected = true;
        //         g_string_free (error_message, true);
        //         return true;
        //     }
        // }

        if !self.connected {
            // if constraints_failed {
            //     _set_error (error, RENDERER_ERROR,
            //                 RENDERER_ERROR_BAD_CONSTRAINT,
            //                 "Failed to connected to any renderer due to constraints");
            //     return false;
            // }

            // self.winsys_vtable = NULL;
            // _set_error (error, WINSYS_ERROR,
            //             WINSYS_ERROR_INIT,
            //             "Failed to connected to any renderer: %s",
            //             error_message->str);
            // g_string_free (error_message, true);
            // return false;
        }

        true
    }

    /// Iterates all known display outputs for the given `self` and
    /// passes a corresponding `Output` pointer to the given `callback`
    /// for each one, along with the given `user_data`.
    /// ## `callback`
    /// A `OutputCallback` to be called for
    ///  each display output
    /// ## `user_data`
    /// A user pointer to be passed to `callback`
    pub fn foreach_output<P: FnMut(&Output)>(&self, callback: P) {
        // GList *l;

        // _RETURN_IF_FAIL (self.connected);
        // _RETURN_IF_FAIL (callback != NULL);

        // for (l = self.outputs; l; l = l->next)
        //   callback (l->data, user_data);
        unimplemented!()
    }

    /// Queries what underlying driver is being used by .
    ///
    /// This may only be called on a connected `Renderer`.
    pub fn get_driver(&self) -> Driver {
        // _RETURN_VAL_IF_FAIL (self.connected, 0);

        // return self.driver;
        unimplemented!()
    }

    /// Queries how many texture units can be used from fragment programs
    ///
    /// # Returns
    ///
    /// the number of texture image units.
    pub fn get_n_fragment_texture_units(&self) -> i32 {
        // int n = 0;

        //     _GET_CONTEXT (ctx, 0);

        // #if defined (HAVE_GL) || defined (HAVE_GLES2)
        //     if (has_feature (ctx, FEATURE_ID_GLSL) ||
        //         has_feature (ctx, FEATURE_ID_ARBFP))
        //     GE (ctx, glGetIntegerv (GL_MAX_TEXTURE_IMAGE_UNITS, &n));
        // #endif

        // return n;
        unimplemented!()
    }

    /// Queries which window system backend  has chosen to use.
    ///
    /// This may only be called on a connected `Renderer`.
    ///
    /// # Returns
    ///
    /// The `WinsysID` corresponding to the chosen window
    ///  system backend.
    pub fn get_winsys_id(&self) -> WinsysID {
        // _RETURN_VAL_IF_FAIL (self.connected, 0);

        // return self.winsys_vtable->id;
        unimplemented!()
    }

    /// This removes a renderer selection `constraint`.
    ///
    /// Applications should ideally minimize how many of these constraints they
    /// depend on to ensure maximum portability.
    /// ## `constraint`
    /// A `RendererConstraint` to remove
    pub fn remove_constraint(&self, constraint: RendererConstraint) {
        // g_return_if_fail (!self.connected);
        // self.constraints = g_list_remove (self.constraints,
        //                                        GUINT_TO_POINTER (constraint));
        unimplemented!()
    }

    /// Requests that  should try to use a specific underlying driver
    /// for rendering.
    ///
    /// If you select an unsupported driver then `Renderer::connect`
    /// will fail and report an error. Most applications should not
    /// explicitly select a driver and should rely on  automatically
    /// choosing the driver.
    ///
    /// This may only be called on an un-connected `Renderer`.
    pub fn set_driver(&self, driver: Driver) {
        // _RETURN_IF_FAIL (!self.connected);
        // self.driver_override = driver;
        unimplemented!()
    }

    /// This allows you to explicitly select a winsys backend to use instead
    /// of letting  automatically select a backend.
    ///
    /// if you select an unsupported backend then `Renderer::connect`
    /// will fail and report an error.
    ///
    /// This may only be called on an un-connected `Renderer`.
    /// ## `winsys_id`
    /// An ID of the winsys you explicitly want to use.
    pub fn set_winsys_id(&self, winsys_id: WinsysID) {
        // _RETURN_IF_FAIL (!self.connected);

        // self.winsys_id_override = winsys_id;
        unimplemented!()
    }
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Renderer")
    }
}
