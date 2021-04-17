use glib::translate::*;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialLayer(ffi::CoglMaterialLayer);

impl GlibPtrDefault for MaterialLayer {
    type GlibType = ffi::CoglMaterialLayer;
}

#[doc(hidden)]
impl Uninitialized for MaterialLayer {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl<'a> ToGlibPtr<'a, ffi::CoglMaterialLayer> for MaterialLayer {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, ffi::CoglMaterialLayer, MaterialLayer> {
        Stash(self.0, ())
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglMaterialLayer> for MaterialLayer {
    type Storage = ();

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglMaterialLayer, MaterialLayer> {
        StashMut(&mut self.0, ())
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::CoglMaterialLayer> for &'a MaterialLayer {
    type Storage = (
        Vec<Stash<'a, ffi::CoglMaterialLayer, &'a MaterialLayer>>,
        Option<Vec<ffi::CoglMaterialLayer>>,
    );

    fn to_glib_none_from_slice(
        t: &'a [&'a MaterialLayer],
    ) -> (*mut ffi::CoglMaterialLayer, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (
            v_ptr.as_ptr() as *mut ffi::CoglMaterialLayer,
            (v, Some(v_ptr)),
        )
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a MaterialLayer],
    ) -> (*mut ffi::CoglMaterialLayer, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr =
                glib_sys::g_malloc0(mem::size_of::<ffi::CoglMaterialLayer>() * (t.len() + 1))
                    as *mut ffi::CoglMaterialLayer;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a MaterialLayer]) -> *mut ffi::CoglMaterialLayer {
        unimplemented!()
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *const ffi::CoglMaterialLayer> for &'a MaterialLayer {
    type Storage = (
        Vec<Stash<'a, ffi::CoglMaterialLayer, &'a MaterialLayer>>,
        Option<Vec<ffi::CoglMaterialLayer>>,
    );

    fn to_glib_none_from_slice(
        t: &'a [&'a MaterialLayer],
    ) -> (*const ffi::CoglMaterialLayer, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (
            v_ptr.as_ptr() as *const ffi::CoglMaterialLayer,
            (v, Some(v_ptr)),
        )
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a MaterialLayer],
    ) -> (*const ffi::CoglMaterialLayer, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr =
                glib_sys::g_malloc0(mem::size_of::<ffi::CoglMaterialLayer>() * (t.len() + 1))
                    as *mut ffi::CoglMaterialLayer;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr as *const ffi::CoglMaterialLayer
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a MaterialLayer]) -> *const ffi::CoglMaterialLayer {
        unimplemented!()
    }
}

impl FromGlibPtrNone<ffi::CoglMaterialLayer> for MaterialLayer {
    #[inline]
    unsafe fn from_glib_none(ptr: ffi::CoglMaterialLayer) -> MaterialLayer {
        MaterialLayer(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglMaterialLayer> for MaterialLayer {
    unsafe fn from_glib_none(ptr: *const ffi::CoglMaterialLayer) -> Self {
        *(ptr as *const MaterialLayer)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglMaterialLayer> for MaterialLayer {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglMaterialLayer) -> Self {
        *(ptr as *mut MaterialLayer)
    }
}

impl FromGlibPtrBorrow<ffi::CoglMaterialLayer> for MaterialLayer {
    #[inline]
    unsafe fn from_glib_borrow(
        ptr: ffi::CoglMaterialLayer,
    ) -> glib::translate::Borrowed<MaterialLayer> {
        glib::translate::Borrowed::new(MaterialLayer(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglMaterialLayer> for MaterialLayer {
    unsafe fn from_glib_borrow(
        ptr: *mut ffi::CoglMaterialLayer,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut MaterialLayer))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglMaterialLayer> for MaterialLayer {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglMaterialLayer,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const MaterialLayer))
    }
}

impl FromGlibPtrFull<ffi::CoglMaterialLayer> for MaterialLayer {
    #[inline]
    unsafe fn from_glib_full(_: ffi::CoglMaterialLayer) -> MaterialLayer {
        unimplemented!()
    }
}

impl FromGlibContainerAsVec<ffi::CoglMaterialLayer, *mut ffi::CoglMaterialLayer> for MaterialLayer {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::CoglMaterialLayer, num: usize) -> Vec<Self> {
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
        ptr: *mut ffi::CoglMaterialLayer,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::CoglMaterialLayer, num: usize) -> Vec<Self> {
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

impl FromGlibPtrArrayContainerAsVec<ffi::CoglMaterialLayer, *mut ffi::CoglMaterialLayer>
    for MaterialLayer
{
    unsafe fn from_glib_none_as_vec(ptr: *mut ffi::CoglMaterialLayer) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::CoglMaterialLayer) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut ffi::CoglMaterialLayer) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}
