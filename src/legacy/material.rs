use glib::translate::*;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Material(ffi::CoglMaterial);

impl GlibPtrDefault for Material {
    type GlibType = ffi::CoglMaterial;
}

#[doc(hidden)]
impl Uninitialized for Material {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl<'a> ToGlibPtr<'a, ffi::CoglMaterial> for Material {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, ffi::CoglMaterial, Material> {
        Stash(self.0, ())
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglMaterial> for Material {
    type Storage = ();

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglMaterial, Material> {
        StashMut(&mut self.0, ())
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::CoglMaterial> for &'a Material {
    type Storage = (
        Vec<Stash<'a, ffi::CoglMaterial, &'a Material>>,
        Option<Vec<ffi::CoglMaterial>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Material]) -> (*mut ffi::CoglMaterial, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *mut ffi::CoglMaterial, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a Material],
    ) -> (*mut ffi::CoglMaterial, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglMaterial>() * (t.len() + 1))
                as *mut ffi::CoglMaterial;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Material]) -> *mut ffi::CoglMaterial {
        unimplemented!()
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *const ffi::CoglMaterial> for &'a Material {
    type Storage = (
        Vec<Stash<'a, ffi::CoglMaterial, &'a Material>>,
        Option<Vec<ffi::CoglMaterial>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Material]) -> (*const ffi::CoglMaterial, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *const ffi::CoglMaterial, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a Material],
    ) -> (*const ffi::CoglMaterial, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<ffi::CoglMaterial>() * (t.len() + 1))
                as *mut ffi::CoglMaterial;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr as *const ffi::CoglMaterial
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Material]) -> *const ffi::CoglMaterial {
        unimplemented!()
    }
}

impl FromGlibPtrNone<ffi::CoglMaterial> for Material {
    #[inline]
    unsafe fn from_glib_none(ptr: ffi::CoglMaterial) -> Material {
        Material(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglMaterial> for Material {
    unsafe fn from_glib_none(ptr: *const ffi::CoglMaterial) -> Self {
        *(ptr as *const Material)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglMaterial> for Material {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglMaterial) -> Self {
        *(ptr as *mut Material)
    }
}

impl FromGlibPtrBorrow<ffi::CoglMaterial> for Material {
    #[inline]
    unsafe fn from_glib_borrow(ptr: ffi::CoglMaterial) -> glib::translate::Borrowed<Material> {
        glib::translate::Borrowed::new(Material(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglMaterial> for Material {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglMaterial) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut Material))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglMaterial> for Material {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglMaterial) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const Material))
    }
}

impl FromGlibPtrFull<ffi::CoglMaterial> for Material {
    #[inline]
    unsafe fn from_glib_full(_: ffi::CoglMaterial) -> Material {
        unimplemented!()
    }
}

impl FromGlibContainerAsVec<ffi::CoglMaterial, *mut ffi::CoglMaterial> for Material {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::CoglMaterial, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::CoglMaterial, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::CoglMaterial, num: usize) -> Vec<Self> {
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

impl FromGlibPtrArrayContainerAsVec<ffi::CoglMaterial, *mut ffi::CoglMaterial> for Material {
    unsafe fn from_glib_none_as_vec(ptr: *mut ffi::CoglMaterial) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::CoglMaterial) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut ffi::CoglMaterial) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}
