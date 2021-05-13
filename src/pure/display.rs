use super::{OnscreenTemplate, Renderer};
use std::fmt;

// * SECTION:cogl-display
// * @short_description: Common aspects of a display pipeline
// *
// * The basic intention for this object is to let the application
// * configure common display preferences before creating a context, and
// * there are a few different aspects to this...
// *
// * Firstly there are options directly relating to the physical display
// * pipeline that is currently being used including the digital to
// * analogue conversion hardware and the screens the user sees.
// *
// * Another aspect is that display options may constrain or affect how
// * onscreen framebuffers should later be configured. The original
// * rationale for the display object in fact was to let us handle GLX
// * and EGLs requirements that framebuffers must be "compatible" with
// * the config associated with the current context meaning we have to
// * force the user to describe how they would like to create their
// * onscreen windows before we can choose a suitable fbconfig and
// * create a GLContext.

pub struct Display {
    //     Object _parent;

//     Bool setup;
//     Renderer *renderer;
//     OnscreenTemplate *onscreen_template;

//   #ifdef COGL_HAS_WAYLAND_EGL_SERVER_SUPPORT
//     struct wl_display *wayland_compositor_display;
//   #endif

//   #ifdef COGL_HAS_EGL_PLATFORM_GDL_SUPPORT
//     gdl_plane_id_t gdl_plane;
//   #endif

//     void *winsys;
}

impl Display {
    /// Explicitly allocates a new `Display` object to encapsulate the
    /// common state of the display pipeline that applies to the whole
    /// application.
    ///
    /// `<note>`Many applications don't need to explicitly use
    /// `Display::new` and can just jump straight to `Context::new`
    /// and pass a `None` display argument so  will automatically
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
    /// display to `Context::new` and  will implicitly apply your
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
        // Display *display = g_slice_new0 (Display);
        // Error *error = NULL;

        // _init ();

        // display->renderer = renderer;
        // if (renderer)
        //     object_ref (renderer);
        // else
        //     display->renderer = renderer_new ();

        // if (!renderer_connect (display->renderer, &error))
        //     g_error ("Failed to connect to renderer: %s\n", error->message);

        // display->setup = FALSE;

        // #ifdef COGL_HAS_EGL_PLATFORM_GDL_SUPPORT
        // display->gdl_plane = GDL_PLANE_ID_UPP_C;
        // #endif

        // display = _display_object_new (display);

        // display_set_onscreen_template (display, onscreen_template);

        // return display;
        unimplemented!()
    }

    /// Queries the `Renderer` associated with the given `self`.
    ///
    /// # Returns
    ///
    /// The associated `Renderer`
    pub fn get_renderer(&self) -> Option<Renderer> {
        // return display->renderer;
        unimplemented!()
    }

    /// Specifies a template for creating `Onscreen` framebuffers.
    ///
    /// Depending on the system, the constraints for creating `Onscreen`
    /// framebuffers need to be known before setting up a `Display` because the
    /// final setup of the display may constrain how onscreen framebuffers may be
    /// allocated. If  knows how an application wants to allocate onscreen
    /// framebuffers then it can try to make sure to setup the display accordingly.
    /// ## `onscreen_template`
    /// A template for creating `Onscreen` framebuffers
    pub fn set_onscreen_template(&self, onscreen_template: &OnscreenTemplate) {
        // _COGL_RETURN_IF_FAIL (display->setup == FALSE);

        // if (onscreen_template)
        //     object_ref (onscreen_template);

        // if (display->onscreen_template)
        //     object_unref (display->onscreen_template);

        // display->onscreen_template = onscreen_template;

        // /* NB: we want to maintain the invariable that there is always an
        // * onscreen template associated with a Display... */
        // if (!onscreen_template)
        //     display->onscreen_template = onscreen_template_new (NULL);
        unimplemented!()
    }

    /// Explicitly sets up the given `self` object. Use of this api is
    /// optional since  will internally setup the display if not done
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
    /// If you instead rely on  implicitly calling `Display::setup`
    /// for you then if there is an error with the configuration you won't
    /// get an opportunity to handle that and the application may abort
    /// with a message. For simple applications that don't have any
    /// fallback options this behaviour may be fine.
    ///
    /// # Returns
    ///
    /// Returns `true` if there was no error, else it returns
    ///  `false` and returns an exception via `error`.
    pub fn setup(&self) -> bool {
        // const WinsysVtable *winsys;

        // if (display->setup)
        //     return TRUE;

        // winsys = _display_get_winsys (display);
        // if (!winsys->display_setup (display, error))
        //     return FALSE;

        // display->setup = TRUE;

        // return TRUE;
        unimplemented!()
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display")
    }
}
