use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GtypeObject {
    // pub parent_instance: gobject::GTypeInstance, // TODO: deal with it
    pub dummy: u32,
}

#[doc(hidden)]
impl Uninitialized for GtypeObject {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglGtypeObject> for GtypeObject {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglGtypeObject, Self> {
        let ptr: *const GtypeObject = &*self;
        Stash(ptr as *const ffi::CoglGtypeObject, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglGtypeObject> for GtypeObject {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglGtypeObject, Self> {
        let ptr: *mut GtypeObject = &mut *self;
        StashMut(ptr as *mut ffi::CoglGtypeObject, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglGtypeObject> for GtypeObject {
    unsafe fn from_glib_none(ptr: *const ffi::CoglGtypeObject) -> Self {
        *(ptr as *const GtypeObject)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglGtypeObject> for GtypeObject {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglGtypeObject) -> Self {
        *(ptr as *mut GtypeObject)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglGtypeObject> for GtypeObject {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglGtypeObject) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut GtypeObject))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglGtypeObject> for GtypeObject {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglGtypeObject,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const GtypeObject))
    }
}
