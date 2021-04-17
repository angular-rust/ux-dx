glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FrameClosure(Boxed<ffi::CoglFrameClosure>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::cogl_frame_closure_get_gtype(), ptr as *mut _) as *mut ffi::CoglFrameClosure,
        free => |ptr| gobject_sys::g_boxed_free(ffi::cogl_frame_closure_get_gtype(), ptr as *mut _),
        get_type => || ffi::cogl_frame_closure_get_gtype(),
    }
}
