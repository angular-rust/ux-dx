glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OnscreenResizeClosure(Boxed<ffi::CoglOnscreenResizeClosure>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::cogl_onscreen_resize_closure_get_gtype(), ptr as *mut _) as *mut ffi::CoglOnscreenResizeClosure,
        free => |ptr| gobject_sys::g_boxed_free(ffi::cogl_onscreen_resize_closure_get_gtype(), ptr as *mut _),
        get_type => || ffi::cogl_onscreen_resize_closure_get_gtype(),
    }
}
