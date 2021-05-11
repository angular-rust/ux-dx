glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OnscreenDirtyClosure(Boxed<ffi::CoglOnscreenDirtyClosure>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::cogl_onscreen_dirty_closure_get_gtype(), ptr as *mut _) as *mut ffi::CoglOnscreenDirtyClosure,
        free => |ptr| gobject_sys::g_boxed_free(ffi::cogl_onscreen_dirty_closure_get_gtype(), ptr as *mut _),
        get_type => || ffi::cogl_onscreen_dirty_closure_get_gtype(),
    }
}
