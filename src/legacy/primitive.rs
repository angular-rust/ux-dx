use crate::{
    Attribute, Context, Framebuffer, Indices, Object, Pipeline, VertexP2, VertexP2C4, VertexP2T2,
    VertexP2T2C4, VertexP3, VertexP3C4, VertexP3T2, VertexP3T2C4, VerticesMode,
};

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Primitive(Object<ffi::CoglPrimitive, PrimitiveClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_primitive_get_gtype(),
    }
}

impl Primitive {
    //pub fn new(mode: VerticesMode, n_vertices: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Primitive {
    //    unsafe { TODO: call cogl_sys:cogl_primitive_new() }
    //}

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// attribute with a `Attribute` and upload your data.
    ///
    /// For example to draw a convex polygon you can do:
    ///
    /// ```text
    /// CoglVertexP2 triangle[] =
    /// {
    ///   { 0,   300 },
    ///   { 150, 0,  },
    ///   { 300, 300 }
    /// };
    /// prim = primitive_new_p2 (VERTICES_MODE_TRIANGLE_FAN,
    ///                               3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An array
    ///  of `VertexP2` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p2(context: &Context, mode: VerticesMode, data: &[&VertexP2]) -> Primitive {
        let data: Vec<ffi::CoglVertexP2> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p2(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// and color attributes with `Attribute`<!-- -->s and upload
    /// your data.
    ///
    /// For example to draw a convex polygon with a linear gradient you
    /// can do:
    ///
    /// ```text
    /// CoglVertexP2C4 triangle[] =
    /// {
    ///   { 0,   300,  0xff, 0x00, 0x00, 0xff },
    ///   { 150, 0,    0x00, 0xff, 0x00, 0xff },
    ///   { 300, 300,  0xff, 0x00, 0x00, 0xff }
    /// };
    /// prim = primitive_new_p2c4 (VERTICES_MODE_TRIANGLE_FAN,
    ///                                 3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An array
    ///  of `VertexP2C4` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p2c4(context: &Context, mode: VerticesMode, data: &[&VertexP2C4]) -> Primitive {
        let data: Vec<ffi::CoglVertexP2C4> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p2c4(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position and
    /// texture coordinate attributes with `Attribute`<!-- -->s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping you can
    /// do:
    ///
    /// ```text
    /// CoglVertexP2T2 triangle[] =
    /// {
    ///   { 0,   300,  0.0, 1.0},
    ///   { 150, 0,    0.5, 0.0},
    ///   { 300, 300,  1.0, 1.0}
    /// };
    /// prim = primitive_new_p2t2 (VERTICES_MODE_TRIANGLE_FAN,
    ///                                 3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An array
    ///  of `VertexP2T2` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p2t2(context: &Context, mode: VerticesMode, data: &[&VertexP2T2]) -> Primitive {
        let data: Vec<ffi::CoglVertexP2T2> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p2t2(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position, texture
    /// coordinate and color attributes with `Attribute`<!-- -->s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping and a
    /// linear gradient you can do:
    ///
    /// ```text
    /// CoglVertexP2T2C4 triangle[] =
    /// {
    ///   { 0,   300,  0.0, 1.0,  0xff, 0x00, 0x00, 0xff},
    ///   { 150, 0,    0.5, 0.0,  0x00, 0xff, 0x00, 0xff},
    ///   { 300, 300,  1.0, 1.0,  0xff, 0x00, 0x00, 0xff}
    /// };
    /// prim = primitive_new_p2t2c4 (VERTICES_MODE_TRIANGLE_FAN,
    ///                                   3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An
    ///  array of `VertexP2T2C4` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p2t2c4(context: &Context, mode: VerticesMode, data: &[&VertexP2T2C4]) -> Primitive {
        let data: Vec<ffi::CoglVertexP2T2C4> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p2t2c4(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// attribute with a `Attribute` and upload your data.
    ///
    /// For example to draw a convex polygon you can do:
    ///
    /// ```text
    /// CoglVertexP3 triangle[] =
    /// {
    ///   { 0,   300, 0 },
    ///   { 150, 0,   0 },
    ///   { 300, 300, 0 }
    /// };
    /// prim = primitive_new_p3 (VERTICES_MODE_TRIANGLE_FAN,
    ///                               3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An array of
    ///  `VertexP3` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p3(context: &Context, mode: VerticesMode, data: &[&VertexP3]) -> Primitive {
        let data: Vec<ffi::CoglVertexP3> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p3(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// and color attributes with `Attribute`<!-- -->s and upload
    /// your data.
    ///
    /// For example to draw a convex polygon with a linear gradient you
    /// can do:
    ///
    /// ```text
    /// CoglVertexP3C4 triangle[] =
    /// {
    ///   { 0,   300, 0,  0xff, 0x00, 0x00, 0xff },
    ///   { 150, 0,   0,  0x00, 0xff, 0x00, 0xff },
    ///   { 300, 300, 0,  0xff, 0x00, 0x00, 0xff }
    /// };
    /// prim = primitive_new_p3c4 (VERTICES_MODE_TRIANGLE_FAN,
    ///                                 3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An array
    ///  of `VertexP3C4` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p3c4(context: &Context, mode: VerticesMode, data: &[&VertexP3C4]) -> Primitive {
        let data: Vec<ffi::CoglVertexP3C4> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p3c4(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position and
    /// texture coordinate attributes with `Attribute`<!-- -->s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping you can
    /// do:
    ///
    /// ```text
    /// CoglVertexP3T2 triangle[] =
    /// {
    ///   { 0,   300, 0,  0.0, 1.0},
    ///   { 150, 0,   0,  0.5, 0.0},
    ///   { 300, 300, 0,  1.0, 1.0}
    /// };
    /// prim = primitive_new_p3t2 (VERTICES_MODE_TRIANGLE_FAN,
    ///                                 3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An array
    ///  of `VertexP3T2` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p3t2(context: &Context, mode: VerticesMode, data: &[&VertexP3T2]) -> Primitive {
        let data: Vec<ffi::CoglVertexP3T2> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p3t2(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position, texture
    /// coordinate and color attributes with `Attribute`<!-- -->s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping and a
    /// linear gradient you can do:
    ///
    /// ```text
    /// CoglVertexP3T2C4 triangle[] =
    /// {
    ///   { 0,   300, 0,  0.0, 1.0,  0xff, 0x00, 0x00, 0xff},
    ///   { 150, 0,   0,  0.5, 0.0,  0x00, 0xff, 0x00, 0xff},
    ///   { 300, 300, 0,  1.0, 1.0,  0xff, 0x00, 0x00, 0xff}
    /// };
    /// prim = primitive_new_p3t2c4 (VERTICES_MODE_TRIANGLE_FAN,
    ///                                   3, triangle);
    /// primitive_draw (prim);
    /// ```
    ///
    /// The value passed as `n_vertices` is initially used to determine how
    /// much can be read from `data` but it will also be used to update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to read when drawing.
    ///
    /// `<note>`The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)`</note>`
    /// ## `context`
    /// A `Context`
    /// ## `mode`
    /// A `VerticesMode` defining how to draw the vertices
    /// ## `n_vertices`
    /// The number of vertices to read from `data` and also
    ///  the number of vertices to read when later drawing.
    /// ## `data`
    /// An
    ///  array of `VertexP3T2C4` vertices
    ///
    /// # Returns
    ///
    /// A newly allocated `Primitive`
    /// with a reference of 1. This can be freed using `Object::unref`.
    pub fn new_p3t2c4(context: &Context, mode: VerticesMode, data: &[&VertexP3T2C4]) -> Primitive {
        let data: Vec<ffi::CoglVertexP3T2C4> = data
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n_vertices = data.len() as i32;
        unsafe {
            from_glib_full(ffi::cogl_primitive_new_p3t2c4(
                context.to_glib_none().0,
                mode.to_glib(),
                n_vertices,
                data.as_ptr(),
            ))
        }
    }

    // TODO:
    // pub fn with_attributes(
    //     mode: VerticesMode,
    //     n_vertices: i32,
    //     attributes: &[&Attribute],
    //     n_attributes: i32,
    // ) -> Primitive {
    //
    //     unsafe {
    //         from_glib_full(ffi::cogl_primitive_new_with_attributes(
    //             mode.to_glib(),
    //             n_vertices,
    //             attributes.to_glib_none().0,
    //             n_attributes,
    //         ))
    //     }
    // }

    /// Makes a copy of an existing `Primitive`. Note that the primitive
    /// is a shallow copy which means it will use the same attributes and
    /// attribute buffers as the original primitive.
    ///
    /// # Returns
    ///
    /// the new primitive
    pub fn copy(&self) -> Option<Primitive> {
        unsafe { from_glib_full(ffi::cogl_primitive_copy(self.to_glib_none().0)) }
    }

    /// Draws the given `self` geometry to the specified destination
    /// `framebuffer` using the graphics processing state described by `pipeline`.
    ///
    /// This drawing api doesn't support high-level meta texture types such
    /// as `Texture2DSliced` so it is the user's responsibility to
    /// ensure that only low-level textures that can be directly sampled by
    /// a GPU such as `Texture2D`, `TextureRectangle` or `Texture3D`
    /// are associated with layers of the given `pipeline`.
    /// ## `framebuffer`
    /// A destination `Framebuffer`
    /// ## `pipeline`
    /// A `Pipeline` state object
    pub fn draw<P: IsA<Framebuffer>>(&self, framebuffer: &P, pipeline: &Pipeline) {
        unsafe {
            ffi::cogl_primitive_draw(
                self.to_glib_none().0,
                framebuffer.as_ref().to_glib_none().0,
                pipeline.to_glib_none().0,
            );
        }
    }

    /// Iterates all the attributes of the given `Primitive`.
    /// ## `callback`
    /// A `CoglPrimitiveAttributeCallback` to be
    ///  called for each attribute
    /// ## `user_data`
    /// Private data that will be passed to the
    ///  callback
    pub fn foreach_attribute<P: FnMut(&Primitive, &Attribute) -> i32>(&self, callback: P) {
        //TODO: should replace i32 to bool in callback
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Primitive, &Attribute) -> i32>(
            primitive: *mut ffi::CoglPrimitive,
            attribute: *mut ffi::CoglAttribute,
            user_data: glib_sys::gpointer,
        ) -> ffi::CoglBool {
            let primitive = from_glib_borrow(primitive);
            let attribute = from_glib_borrow(attribute);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&primitive, &attribute)
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::cogl_primitive_foreach_attribute(
                self.to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    pub fn get_first_vertex(&self) -> i32 {
        unsafe { ffi::cogl_primitive_get_first_vertex(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the indices that were set with
    /// `Primitive::set_indices` or `None` if no indices were set.
    pub fn get_indices(&self) -> Option<Indices> {
        unsafe { from_glib_none(ffi::cogl_primitive_get_indices(self.to_glib_none().0)) }
    }

    pub fn get_mode(&self) -> VerticesMode {
        unsafe { from_glib(ffi::cogl_primitive_get_mode(self.to_glib_none().0)) }
    }

    /// Queries the number of vertices to read when drawing the given
    /// `self`. Usually this value is implicitly set when associating
    /// vertex data or indices with a `Primitive`.
    ///
    /// If `Primitive::set_indices` has been used to associate a
    /// sequence of `Indices` with the given `self` then the
    /// number of vertices to read can also be phrased as the number
    /// of indices to read.
    ///
    /// `<note>`To be clear; it doesn't refer to the number of vertices - in
    /// terms of data - associated with the primitive it's just the number
    /// of vertices to read and draw.`</note>`
    ///
    /// # Returns
    ///
    /// The number of vertices to read when drawing.
    pub fn get_n_vertices(&self) -> i32 {
        unsafe { ffi::cogl_primitive_get_n_vertices(self.to_glib_none().0) }
    }

    // TODO:
    // /// Replaces all the attributes of the given `Primitive` object.
    // /// ## `attributes`
    // /// an array of `Attribute` pointers
    // /// ## `n_attributes`
    // /// the number of elements in `attributes`
    // pub fn set_attributes(&self, attributes: &[&Attribute], n_attributes: i32) {
    //     unsafe {
    //         ffi::cogl_primitive_set_attributes(
    //             self.to_glib_none().0,
    //             attributes.to_glib_none().0,
    //             n_attributes,
    //         );
    //     }
    // }

    pub fn set_first_vertex(&self, first_vertex: i32) {
        unsafe {
            ffi::cogl_primitive_set_first_vertex(self.to_glib_none().0, first_vertex);
        }
    }

    /// Associates a sequence of `Indices` with the given `self`.
    ///
    /// `Indices` provide a way to virtualize your real vertex data by
    /// providing a sequence of indices that index into your real vertex
    /// data. The GPU will walk though the index values to indirectly
    /// lookup the data for each vertex instead of sequentially walking
    /// through the data directly. This lets you save memory by indexing
    /// shared data multiple times instead of duplicating the data.
    ///
    /// The value passed as `n_indices` will simply update the
    /// `Primitive` `<structfield>`n_vertices`</structfield>` property as if
    /// `Primitive::set_n_vertices` were called. This property defines
    /// the number of vertices to draw or, put another way, how many
    /// indices should be read from `indices` when drawing.
    ///
    /// `<note>`The `Primitive` `<structfield>`first_vertex`</structfield>` property
    /// also affects drawing with indices by defining the first entry of the
    /// indices to start drawing from.`</note>`
    /// ## `indices`
    /// A `Indices` array
    /// ## `n_indices`
    /// The number of indices to reference when drawing
    pub fn set_indices(&self, indices: &Indices, n_indices: i32) {
        unsafe {
            ffi::cogl_primitive_set_indices(
                self.to_glib_none().0,
                indices.to_glib_none().0,
                n_indices,
            );
        }
    }

    pub fn set_mode(&self, mode: VerticesMode) {
        unsafe {
            ffi::cogl_primitive_set_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    /// Specifies how many vertices should be read when drawing the given
    /// `self`.
    ///
    /// Usually this value is set implicitly when associating vertex data
    /// or indices with a `Primitive`.
    ///
    /// `<note>`To be clear; it doesn't refer to the number of vertices - in
    /// terms of data - associated with the primitive it's just the number
    /// of vertices to read and draw.`</note>`
    /// ## `n_vertices`
    /// The number of vertices to read when drawing.
    pub fn set_n_vertices(&self, n_vertices: i32) {
        unsafe {
            ffi::cogl_primitive_set_n_vertices(self.to_glib_none().0, n_vertices);
        }
    }

    //pub fn texture_set_auto_mipmap(primitive_texture: /*Unknown conversion*//*Unimplemented*/PrimitiveTexture, value: Bool) {
    //    unsafe { TODO: call cogl_sys:cogl_primitive_texture_set_auto_mipmap() }
    //}
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Primitive")
    }
}
