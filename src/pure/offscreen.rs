use crate::Texture;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Offscreen(Object<ffi::CoglOffscreen, OffscreenClass>);

    match fn {
        get_type => || ffi::cogl_offscreen_get_gtype(),
    }
}

impl Offscreen {
    pub fn with_texture<P: IsA<Texture>>(texture: &P) -> Offscreen {
        unsafe {
            from_glib_full(ffi::cogl_offscreen_new_with_texture(
                texture.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Offscreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offscreen")
    }
}
