use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug)] // PartialEq, Eq, Hash
pub struct DepthState {
    pub private_member_magic: u32,
    pub private_member_test_enabled: bool,  // CoglBool
    pub private_member_test_function: i32, // CoglDepthTestFunction, // TODO: possible should be enum
    pub private_member_write_enabled: bool, // CoglBool
    pub private_member_range_near: f32,
    pub private_member_range_far: f32,
    pub private_member_padding0: u32,
    pub private_member_padding1: u32,
    pub private_member_padding2: u32,
    pub private_member_padding3: u32,
    pub private_member_padding4: u32,
    pub private_member_padding5: u32,
    pub private_member_padding6: u32,
    pub private_member_padding7: u32,
    pub private_member_padding8: u32,
    pub private_member_padding9: u32,
}

//pub const COGL_DEPTH_TEST_FUNCTION_NEVER: CoglDepthTestFunction = 512;
//pub const COGL_DEPTH_TEST_FUNCTION_LESS: CoglDepthTestFunction = 513;
//pub const COGL_DEPTH_TEST_FUNCTION_EQUAL: CoglDepthTestFunction = 514;
//pub const COGL_DEPTH_TEST_FUNCTION_LEQUAL: CoglDepthTestFunction = 515;
//pub const COGL_DEPTH_TEST_FUNCTION_GREATER: CoglDepthTestFunction = 516;
//pub const COGL_DEPTH_TEST_FUNCTION_NOTEQUAL: CoglDepthTestFunction = 517;
//pub const COGL_DEPTH_TEST_FUNCTION_GEQUAL: CoglDepthTestFunction = 518;
//pub const COGL_DEPTH_TEST_FUNCTION_ALWAYS: CoglDepthTestFunction = 519;

#[doc(hidden)]
impl Uninitialized for DepthState {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglDepthState> for DepthState {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglDepthState, Self> {
        let ptr: *const DepthState = &*self;
        Stash(ptr as *const ffi::CoglDepthState, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglDepthState> for DepthState {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglDepthState, Self> {
        let ptr: *mut DepthState = &mut *self;
        StashMut(ptr as *mut ffi::CoglDepthState, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglDepthState> for DepthState {
    unsafe fn from_glib_none(ptr: *const ffi::CoglDepthState) -> Self {
        *(ptr as *const DepthState)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglDepthState> for DepthState {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglDepthState) -> Self {
        *(ptr as *mut DepthState)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglDepthState> for DepthState {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglDepthState) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut DepthState))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglDepthState> for DepthState {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglDepthState) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const DepthState))
    }
}
