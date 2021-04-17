#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PollFD {
    pub fd: i32,
    //TODO: _truncated_record_marker: c_void,
    // // /*Ignored*/field events has incomplete type
}

#[doc(hidden)]
impl Uninitialized for PollFD {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglPollFD> for PollFD {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglPollFD, Self> {
        let ptr: *const PollFD = &*self;
        Stash(ptr as *const ffi::CoglPollFD, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglPollFD> for PollFD {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglPollFD, Self> {
        let ptr: *mut PollFD = &mut *self;
        StashMut(ptr as *mut ffi::CoglPollFD, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglPollFD> for PollFD {
    unsafe fn from_glib_none(ptr: *const ffi::CoglPollFD) -> Self {
        *(ptr as *const PollFD)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglPollFD> for PollFD {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglPollFD) -> Self {
        *(ptr as *mut PollFD)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglPollFD> for PollFD {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglPollFD) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut PollFD))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglPollFD> for PollFD {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglPollFD) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const PollFD))
    }
}
