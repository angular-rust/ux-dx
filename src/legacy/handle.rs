use glib::translate::*;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Handle(ffi::CoglHandle);

impl GlibPtrDefault for Handle {
    type GlibType = ffi::CoglHandle;
}

#[doc(hidden)]
impl Uninitialized for Handle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl<'a> ToGlibPtr<'a, ffi::CoglHandle> for Handle {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, ffi::CoglHandle, Handle> {
        Stash(self.0, ())
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglHandle> for Handle {
    type Storage = ();

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglHandle, Handle> {
        StashMut(&mut self.0, ())
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::CoglHandle> for &'a Handle {
    type Storage = (
        Vec<Stash<'a, ffi::CoglHandle, &'a Handle>>,
        Option<Vec<ffi::CoglHandle>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Handle]) -> (*mut ffi::CoglHandle, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *mut ffi::CoglHandle, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [&'a Handle]) -> (*mut ffi::CoglHandle, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglHandle>() * (t.len() + 1))
                as *mut ffi::CoglHandle;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Handle]) -> *mut ffi::CoglHandle {
        unimplemented!()
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *const ffi::CoglHandle> for &'a Handle {
    type Storage = (
        Vec<Stash<'a, ffi::CoglHandle, &'a Handle>>,
        Option<Vec<ffi::CoglHandle>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Handle]) -> (*const ffi::CoglHandle, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *const ffi::CoglHandle, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a Handle],
    ) -> (*const ffi::CoglHandle, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglHandle>() * (t.len() + 1))
                as *mut ffi::CoglHandle;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr as *const ffi::CoglHandle
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Handle]) -> *const ffi::CoglHandle {
        unimplemented!()
    }
}

impl FromGlibPtrNone<ffi::CoglHandle> for Handle {
    #[inline]
    unsafe fn from_glib_none(ptr: ffi::CoglHandle) -> Handle {
        Handle(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglHandle> for Handle {
    unsafe fn from_glib_none(ptr: *const ffi::CoglHandle) -> Self {
        *(ptr as *const Handle)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglHandle> for Handle {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglHandle) -> Self {
        *(ptr as *mut Handle)
    }
}

impl FromGlibPtrBorrow<ffi::CoglHandle> for Handle {
    #[inline]
    unsafe fn from_glib_borrow(ptr: ffi::CoglHandle) -> glib::translate::Borrowed<Handle> {
        glib::translate::Borrowed::new(Handle(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglHandle> for Handle {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglHandle) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut Handle))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglHandle> for Handle {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglHandle) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const Handle))
    }
}

impl FromGlibPtrFull<ffi::CoglHandle> for Handle {
    #[inline]
    unsafe fn from_glib_full(_: ffi::CoglHandle) -> Handle {
        unimplemented!()
    }
}

impl FromGlibContainerAsVec<ffi::CoglHandle, *mut ffi::CoglHandle> for Handle {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::CoglHandle, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::CoglHandle, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::CoglHandle, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_full(ptr::read(ptr.add(i))));
        }
        glib_sys::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrArrayContainerAsVec<ffi::CoglHandle, *mut ffi::CoglHandle> for Handle {
    unsafe fn from_glib_none_as_vec(ptr: *mut ffi::CoglHandle) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::CoglHandle) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut ffi::CoglHandle) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}
