use glib::translate::*;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fence(ffi::CoglFence);

impl GlibPtrDefault for Fence {
    type GlibType = ffi::CoglFence;
}

#[doc(hidden)]
impl Uninitialized for Fence {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl<'a> ToGlibPtr<'a, ffi::CoglFence> for Fence {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, ffi::CoglFence, Fence> {
        Stash(self.0, ())
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglFence> for Fence {
    type Storage = ();

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglFence, Fence> {
        StashMut(&mut self.0, ())
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::CoglFence> for &'a Fence {
    type Storage = (
        Vec<Stash<'a, ffi::CoglFence, &'a Fence>>,
        Option<Vec<ffi::CoglFence>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Fence]) -> (*mut ffi::CoglFence, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *mut ffi::CoglFence, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [&'a Fence]) -> (*mut ffi::CoglFence, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglFence>() * (t.len() + 1))
                as *mut ffi::CoglFence;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Fence]) -> *mut ffi::CoglFence {
        unimplemented!()
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *const ffi::CoglFence> for &'a Fence {
    type Storage = (
        Vec<Stash<'a, ffi::CoglFence, &'a Fence>>,
        Option<Vec<ffi::CoglFence>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Fence]) -> (*const ffi::CoglFence, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *const ffi::CoglFence, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [&'a Fence]) -> (*const ffi::CoglFence, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglFence>() * (t.len() + 1))
                as *mut ffi::CoglFence;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr as *const ffi::CoglFence
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Fence]) -> *const ffi::CoglFence {
        unimplemented!()
    }
}

impl FromGlibPtrNone<ffi::CoglFence> for Fence {
    #[inline]
    unsafe fn from_glib_none(ptr: ffi::CoglFence) -> Fence {
        Fence(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglFence> for Fence {
    unsafe fn from_glib_none(ptr: *const ffi::CoglFence) -> Self {
        *(ptr as *const Fence)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglFence> for Fence {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglFence) -> Self {
        *(ptr as *mut Fence)
    }
}

impl FromGlibPtrBorrow<ffi::CoglFence> for Fence {
    #[inline]
    unsafe fn from_glib_borrow(ptr: ffi::CoglFence) -> glib::translate::Borrowed<Fence> {
        glib::translate::Borrowed::new(Fence(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglFence> for Fence {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglFence) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut Fence))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglFence> for Fence {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglFence) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const Fence))
    }
}

impl FromGlibPtrFull<ffi::CoglFence> for Fence {
    #[inline]
    unsafe fn from_glib_full(_: ffi::CoglFence) -> Fence {
        unimplemented!()
    }
}

impl FromGlibContainerAsVec<ffi::CoglFence, *mut ffi::CoglFence> for Fence {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::CoglFence, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::CoglFence, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::CoglFence, num: usize) -> Vec<Self> {
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

impl FromGlibPtrArrayContainerAsVec<ffi::CoglFence, *mut ffi::CoglFence> for Fence {
    unsafe fn from_glib_none_as_vec(ptr: *mut ffi::CoglFence) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::CoglFence) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut ffi::CoglFence) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}
