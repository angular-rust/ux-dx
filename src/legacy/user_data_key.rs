use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserDataKey {
    pub unused: i32,
}

#[doc(hidden)]
impl Uninitialized for UserDataKey {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglUserDataKey> for UserDataKey {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglUserDataKey, Self> {
        let ptr: *const UserDataKey = &*self;
        Stash(ptr as *const ffi::CoglUserDataKey, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglUserDataKey> for UserDataKey {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglUserDataKey, Self> {
        let ptr: *mut UserDataKey = &mut *self;
        StashMut(ptr as *mut ffi::CoglUserDataKey, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglUserDataKey> for UserDataKey {
    unsafe fn from_glib_none(ptr: *const ffi::CoglUserDataKey) -> Self {
        *(ptr as *const UserDataKey)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglUserDataKey> for UserDataKey {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglUserDataKey) -> Self {
        *(ptr as *mut UserDataKey)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglUserDataKey> for UserDataKey {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglUserDataKey) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut UserDataKey))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglUserDataKey> for UserDataKey {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglUserDataKey,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const UserDataKey))
    }
}
