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

// * SECTION:cogl-renderer
// * @short_description: Choosing a means to render
// *
// * A #Renderer represents a means to render. It encapsulates the
// * selection of an underlying driver, such as OpenGL or OpenGL-ES and
// * a selection of a window system binding API such as GLX, or EGL or
// * WGL.
// *
// * A #Renderer has two states, "unconnected" and "connected". When
// * a renderer is first instantiated using renderer_new() it is
// * unconnected so that it can be configured and constraints can be
// * specified for how the backend driver and window system should be
// * chosen.
// *
// * After configuration a #Renderer can (optionally) be explicitly
// * connected using renderer_connect() which allows for the
// * handling of connection errors so that fallback configurations can
// * be tried if necessary. Applications that don't support any
// * fallbacks though can skip using renderer_connect() and leave
// *  to automatically connect the renderer.
// *
// * Once you have a configured #Renderer it can be used to create a
// * #Display object using display_new().
// *
// * <note>Many applications don't need to explicitly use
// * renderer_new() or display_new() and can just jump
// * straight to context_new() and pass a %NULL display argument so
// *  will automatically connect and setup a renderer and
// * display.</note>
pub struct Renderer {
    // Object _parent;
// Bool connected;
// Driver driver_override;
// const DriverVtable *driver_vtable;
// const TextureDriver *texture_driver;
// const WinsysVtable *winsys_vtable;
// WinsysID winsys_id_override;
// GList *constraints;

// GArray *poll_fds;
// int poll_fds_age;
// GList *poll_sources;

// List idle_closures;

// GList *outputs;

// #ifdef COGL_HAS_XLIB_SUPPORT
// Display *foreign_xdpy;
// Bool xlib_enable_event_retrieval;
// #endif

// #ifdef COGL_HAS_WIN32_SUPPORT
// Bool win32_enable_event_retrieval;
// #endif

// Driver driver;
// unsigned long private_features
//     [COGL_FLAGS_N_LONGS_FOR_SIZE (COGL_N_PRIVATE_FEATURES)];
// #ifndef HAVE_DIRECTLY_LINKED_GL_LIBRARY
// GModule *libgl_module;
// #endif

// #if defined (COGL_HAS_EGL_PLATFORM_WAYLAND_SUPPORT)
// struct wl_display *foreign_wayland_display;
// Bool wayland_enable_event_dispatch;
// #endif

// #if defined (COGL_HAS_EGL_PLATFORM_MIR_SUPPORT)
// MirConnection *foreign_mir_connection;
// #endif

// #if defined (COGL_HAS_EGL_PLATFORM_KMS_SUPPORT)
// int kms_fd;
// #endif

// #ifdef COGL_HAS_SDL_SUPPORT
// Bool sdl_event_type_set;
// uint32_t sdl_event_type;
// #endif

// /* List of callback functions that will be given every native event */
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
        // Renderer *renderer = g_new0 (Renderer, 1);

        // _init ();

        // renderer->connected = FALSE;
        // renderer->event_filters = NULL;

        // renderer->poll_fds = g_array_new (FALSE, TRUE, sizeof (PollFD));

        // _list_init (&renderer->idle_closures);

        // #ifdef COGL_HAS_XLIB_SUPPORT
        // renderer->xlib_enable_event_retrieval = TRUE;
        // #endif

        // #ifdef COGL_HAS_WIN32_SUPPORT
        // renderer->win32_enable_event_retrieval = TRUE;
        // #endif

        // #ifdef COGL_HAS_EGL_PLATFORM_WAYLAND_SUPPORT
        // renderer->wayland_enable_event_dispatch = TRUE;
        // #endif

        // #ifdef COGL_HAS_EGL_PLATFORM_KMS_SUPPORT
        // renderer->kms_fd = -1;
        // #endif

        // return _renderer_object_new (renderer);
        unimplemented!()
    }

    /// This adds a renderer selection `constraint`.
    ///
    /// Applications should ideally minimize how many of these constraints they
    /// depend on to ensure maximum portability.
    /// ## `constraint`
    /// A `RendererConstraint` to add
    pub fn add_constraint(&self, constraint: RendererConstraint) {
        // g_return_if_fail (!renderer->connected);
        // renderer->constraints = g_list_prepend (renderer->constraints,
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
        //     return FALSE;

        // display = display_new (renderer, onscreen_template);
        // if (!display_setup (display, error))
        //     {
        //     object_unref (display);
        //     return FALSE;
        //     }

        // object_unref (display);

        // return TRUE;
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
        // Bool constraints_failed = FALSE;

        // if (renderer->connected)
        //     return TRUE;

        // /* The driver needs to be chosen before connecting the renderer
        //     because eglInitialize requires the library containing the GL API
        //     to be loaded before its called */
        // if (!_renderer_choose_driver (renderer, error))
        //     return FALSE;

        // error_message = g_string_new ("");
        // for (i = 0; i < G_N_ELEMENTS (_winsys_vtable_getters); i++)
        //     {
        //     const WinsysVtable *winsys = _winsys_vtable_getters[i]();
        //     Error *tmp_error = NULL;
        //     GList *l;
        //     Bool skip_due_to_constraints = FALSE;

        //     if (renderer->winsys_id_override != COGL_WINSYS_ID_ANY)
        //         {
        //         if (renderer->winsys_id_override != winsys->id)
        //             continue;
        //         }
        //     else
        //         {
        //         char *user_choice = getenv ("COGL_RENDERER");
        //         if (!user_choice)
        //             user_choice = _config_renderer;
        //         if (user_choice &&
        //             g_ascii_strcasecmp (winsys->name, user_choice) != 0)
        //             continue;
        //         }

        //     for (l = renderer->constraints; l; l = l->next)
        //         {
        //         RendererConstraint constraint = GPOINTER_TO_UINT (l->data);
        //         if (!(winsys->constraints & constraint))
        //             {
        //             skip_due_to_constraints = TRUE;
        //             break;
        //             }
        //         }
        //     if (skip_due_to_constraints)
        //         {
        //         constraints_failed |= TRUE;
        //         continue;
        //         }

        //     /* At least temporarily we will associate this winsys with
        //     * the renderer in-case ->renderer_connect calls API that
        //     * wants to query the current winsys... */
        //     renderer->winsys_vtable = winsys;

        //     if (!winsys->renderer_connect (renderer, &tmp_error))
        //         {
        //         g_string_append_c (error_message, '\n');
        //         g_string_append (error_message, tmp_error->message);
        //         error_free (tmp_error);
        //         }
        //     else
        //         {
        //         renderer->connected = TRUE;
        //         g_string_free (error_message, TRUE);
        //         return TRUE;
        //         }
        //     }

        // if (!renderer->connected)
        //     {
        //     if (constraints_failed)
        //         {
        //         _set_error (error, COGL_RENDERER_ERROR,
        //                     COGL_RENDERER_ERROR_BAD_CONSTRAINT,
        //                     "Failed to connected to any renderer due to constraints");
        //         return FALSE;
        //         }

        //     renderer->winsys_vtable = NULL;
        //     _set_error (error, COGL_WINSYS_ERROR,
        //                 COGL_WINSYS_ERROR_INIT,
        //                 "Failed to connected to any renderer: %s",
        //                 error_message->str);
        //     g_string_free (error_message, TRUE);
        //     return FALSE;
        //     }

        // return TRUE;
        unimplemented!()
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

        // _COGL_RETURN_IF_FAIL (renderer->connected);
        // _COGL_RETURN_IF_FAIL (callback != NULL);

        // for (l = renderer->outputs; l; l = l->next)
        //   callback (l->data, user_data);
        unimplemented!()
    }

    /// Queries what underlying driver is being used by .
    ///
    /// This may only be called on a connected `Renderer`.
    pub fn get_driver(&self) -> Driver {
        // _COGL_RETURN_VAL_IF_FAIL (renderer->connected, 0);

        // return renderer->driver;
        unimplemented!()
    }

    /// Queries how many texture units can be used from fragment programs
    ///
    /// # Returns
    ///
    /// the number of texture image units.
    pub fn get_n_fragment_texture_units(&self) -> i32 {
        // int n = 0;

        //     _COGL_GET_CONTEXT (ctx, 0);

        // #if defined (HAVE_COGL_GL) || defined (HAVE_COGL_GLES2)
        //     if (has_feature (ctx, COGL_FEATURE_ID_GLSL) ||
        //         has_feature (ctx, COGL_FEATURE_ID_ARBFP))
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
        // _COGL_RETURN_VAL_IF_FAIL (renderer->connected, 0);

        // return renderer->winsys_vtable->id;
        unimplemented!()
    }

    /// This removes a renderer selection `constraint`.
    ///
    /// Applications should ideally minimize how many of these constraints they
    /// depend on to ensure maximum portability.
    /// ## `constraint`
    /// A `RendererConstraint` to remove
    pub fn remove_constraint(&self, constraint: RendererConstraint) {
        // g_return_if_fail (!renderer->connected);
        // renderer->constraints = g_list_remove (renderer->constraints,
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
        // _COGL_RETURN_IF_FAIL (!renderer->connected);
        // renderer->driver_override = driver;
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
        // _COGL_RETURN_IF_FAIL (!renderer->connected);

        // renderer->winsys_id_override = winsys_id;
        unimplemented!()
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
