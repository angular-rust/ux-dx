use glib::translate::*;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FenceClosure(ffi::CoglFenceClosure);

impl GlibPtrDefault for FenceClosure {
    type GlibType = ffi::CoglFenceClosure;
}

#[doc(hidden)]
impl Uninitialized for FenceClosure {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl<'a> ToGlibPtr<'a, ffi::CoglFenceClosure> for FenceClosure {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, ffi::CoglFenceClosure, FenceClosure> {
        Stash(self.0, ())
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglFenceClosure> for FenceClosure {
    type Storage = ();

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglFenceClosure, FenceClosure> {
        StashMut(&mut self.0, ())
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::CoglFenceClosure> for &'a FenceClosure {
    type Storage = (
        Vec<Stash<'a, ffi::CoglFenceClosure, &'a FenceClosure>>,
        Option<Vec<ffi::CoglFenceClosure>>,
    );

    fn to_glib_none_from_slice(
        t: &'a [&'a FenceClosure],
    ) -> (*mut ffi::CoglFenceClosure, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (
            v_ptr.as_ptr() as *mut ffi::CoglFenceClosure,
            (v, Some(v_ptr)),
        )
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a FenceClosure],
    ) -> (*mut ffi::CoglFenceClosure, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglFenceClosure>() * (t.len() + 1))
                as *mut ffi::CoglFenceClosure;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a FenceClosure]) -> *mut ffi::CoglFenceClosure {
        unimplemented!()
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *const ffi::CoglFenceClosure> for &'a FenceClosure {
    type Storage = (
        Vec<Stash<'a, ffi::CoglFenceClosure, &'a FenceClosure>>,
        Option<Vec<ffi::CoglFenceClosure>>,
    );

    fn to_glib_none_from_slice(
        t: &'a [&'a FenceClosure],
    ) -> (*const ffi::CoglFenceClosure, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (
            v_ptr.as_ptr() as *const ffi::CoglFenceClosure,
            (v, Some(v_ptr)),
        )
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a FenceClosure],
    ) -> (*const ffi::CoglFenceClosure, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglFenceClosure>() * (t.len() + 1))
                as *mut ffi::CoglFenceClosure;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr as *const ffi::CoglFenceClosure
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a FenceClosure]) -> *const ffi::CoglFenceClosure {
        unimplemented!()
    }
}

impl FromGlibPtrNone<ffi::CoglFenceClosure> for FenceClosure {
    #[inline]
    unsafe fn from_glib_none(ptr: ffi::CoglFenceClosure) -> FenceClosure {
        FenceClosure(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_none(ptr: *const ffi::CoglFenceClosure) -> Self {
        *(ptr as *const FenceClosure)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglFenceClosure) -> Self {
        *(ptr as *mut FenceClosure)
    }
}

impl FromGlibPtrBorrow<ffi::CoglFenceClosure> for FenceClosure {
    #[inline]
    unsafe fn from_glib_borrow(
        ptr: ffi::CoglFenceClosure,
    ) -> glib::translate::Borrowed<FenceClosure> {
        glib::translate::Borrowed::new(FenceClosure(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglFenceClosure) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut FenceClosure))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglFenceClosure,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const FenceClosure))
    }
}

impl FromGlibPtrFull<ffi::CoglFenceClosure> for FenceClosure {
    #[inline]
    unsafe fn from_glib_full(_: ffi::CoglFenceClosure) -> FenceClosure {
        unimplemented!()
    }
}

impl FromGlibContainerAsVec<ffi::CoglFenceClosure, *mut ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::CoglFenceClosure, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut ffi::CoglFenceClosure,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::CoglFenceClosure, num: usize) -> Vec<Self> {
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

impl FromGlibPtrArrayContainerAsVec<ffi::CoglFenceClosure, *mut ffi::CoglFenceClosure>
    for FenceClosure
{
    unsafe fn from_glib_none_as_vec(ptr: *mut ffi::CoglFenceClosure) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::CoglFenceClosure) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut ffi::CoglFenceClosure) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}
