use std::{ffi::c_void, fmt};

use crate::prelude::*;

use crate::foundation::vertex::{P2, P2C4, P2T2, P2T2C4, P3, P3C4, P3T2, P3T2C4};

use super::{Attribute, Context, Framebuffer, Indices, Pipeline, VerticesMode};

pub struct PrimitiveTexture(c_void);

// typedef struct _MultiTexturedRect
// {
//   const float *position; /* x0,y0,x1,y1 */
//   const float *tex_coords; /* (tx0,ty0,tx1,ty1)(tx0,ty0,tx1,ty1)(... */
//   int tex_coords_len; /* number of floats in tex_coords? */
// } MultiTexturedRect;

pub struct Primitive {
    // Indices *indices;
    // VerticesMode mode;
    // int first_vertex;
    // int n_vertices;

    // int immutable_ref;

    // Attribute **attributes;
    // int n_attributes;

    // int n_embedded_attributes;
    // Attribute *embedded_attribute;
}

impl Primitive {
    // primitive_new:
    // @mode: A #VerticesMode defining how to draw the vertices
    // @n_vertices: The number of vertices to process when drawing
    // @...: A %NULL terminated list of attributes
    //
    // Combines a set of #Attributes with a specific draw @mode
    // and defines a vertex count so a #Primitive object can be retained and
    // drawn later with no addition information required.
    //
    // The value passed as @n_vertices will simply update the
    // #Primitive <structfield>n_vertices</structfield> property as if
    // primitive_set_n_vertices() were called. This property defines
    // the number of vertices to read when drawing.
    //
    // Return value: (transfer full): A newly allocated #Primitive object
    pub fn new(mode: VerticesMode, n_vertices: i32, args: &[f64]) -> Primitive {
        // va_list ap;
        // int n_attributes;
        // Attribute **attributes;
        // int i;
        // Attribute *attribute;

        // va_start (ap, n_vertices);
        // for (n_attributes = 0; va_arg (ap, Attribute *); n_attributes++)
        //   ;
        // va_end (ap);

        // attributes = g_alloca (sizeof (Attribute *) * n_attributes);

        // va_start (ap, n_vertices);
        // for (i = 0; (attribute = va_arg (ap, Attribute *)); i++)
        //   attributes[i] = attribute;
        // va_end (ap);

        // return primitive_new_with_attributes (mode, n_vertices,
        //                                            attributes,
        //                                            i);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// attribute with a `Attribute` and upload your data.
    ///
    /// For example to draw a convex polygon you can do:
    ///
    /// ```text
    /// VertexP2 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p2(context: &Context, mode: VerticesMode, data: &[&P2]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context, n_vertices * sizeof (VertexP2), data);
        // Attribute *attributes[1];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP2),
        //                                     offsetof (VertexP2, x),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     1);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// and color attributes with `Attribute`s and upload
    /// your data.
    ///
    /// For example to draw a convex polygon with a linear gradient you
    /// can do:
    ///
    /// ```text
    /// VertexP2C4 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p2c4(context: &Context, mode: VerticesMode, data: &[&P2C4]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context, n_vertices * sizeof (VertexP2C4), data);
        // Attribute *attributes[2];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP2C4),
        //                                     offsetof (VertexP2C4, x),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[1] = attribute_new (attribute_buffer,
        //                                     "color_in",
        //                                     sizeof (VertexP2C4),
        //                                     offsetof (VertexP2C4, r),
        //                                     4,
        //                                     ATTRIBUTE_TYPE_UNSIGNED_BYTE);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     2);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position and
    /// texture coordinate attributes with `Attribute`s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping you can
    /// do:
    ///
    /// ```text
    /// VertexP2T2 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p2t2(context: &Context, mode: VerticesMode, data: &[&P2T2]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context, n_vertices * sizeof (VertexP2T2), data);
        // Attribute *attributes[2];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP2T2),
        //                                     offsetof (VertexP2T2, x),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[1] = attribute_new (attribute_buffer,
        //                                     "tex_coord0_in",
        //                                     sizeof (VertexP2T2),
        //                                     offsetof (VertexP2T2, s),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     2);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position, texture
    /// coordinate and color attributes with `Attribute`s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping and a
    /// linear gradient you can do:
    ///
    /// ```text
    /// VertexP2T2C4 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p2t2c4(context: &Context, mode: VerticesMode, data: &[&P2T2C4]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context,
        //                             n_vertices * sizeof (VertexP2T2C4), data);
        // Attribute *attributes[3];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP2T2C4),
        //                                     offsetof (VertexP2T2C4, x),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[1] = attribute_new (attribute_buffer,
        //                                     "tex_coord0_in",
        //                                     sizeof (VertexP2T2C4),
        //                                     offsetof (VertexP2T2C4, s),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[2] = attribute_new (attribute_buffer,
        //                                     "color_in",
        //                                     sizeof (VertexP2T2C4),
        //                                     offsetof (VertexP2T2C4, r),
        //                                     4,
        //                                     ATTRIBUTE_TYPE_UNSIGNED_BYTE);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     3);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// attribute with a `Attribute` and upload your data.
    ///
    /// For example to draw a convex polygon you can do:
    ///
    /// ```text
    /// VertexP3 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p3(context: &Context, mode: VerticesMode, data: &[&P3]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context, n_vertices * sizeof (VertexP3), data);
        // Attribute *attributes[1];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP3),
        //                                     offsetof (VertexP3, x),
        //                                     3,
        //                                     ATTRIBUTE_TYPE_FLOAT);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     1);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position
    /// and color attributes with `Attribute`s and upload
    /// your data.
    ///
    /// For example to draw a convex polygon with a linear gradient you
    /// can do:
    ///
    /// ```text
    /// VertexP3C4 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p3c4(context: &Context, mode: VerticesMode, data: &[&P3C4]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context, n_vertices * sizeof (VertexP3C4), data);
        // Attribute *attributes[2];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP3C4),
        //                                     offsetof (VertexP3C4, x),
        //                                     3,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[1] = attribute_new (attribute_buffer,
        //                                     "color_in",
        //                                     sizeof (VertexP3C4),
        //                                     offsetof (VertexP3C4, r),
        //                                     4,
        //                                     ATTRIBUTE_TYPE_UNSIGNED_BYTE);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     2);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position and
    /// texture coordinate attributes with `Attribute`s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping you can
    /// do:
    ///
    /// ```text
    /// VertexP3T2 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p3t2(context: &Context, mode: VerticesMode, data: &[&P3T2]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context, n_vertices * sizeof (VertexP3T2), data);
        // Attribute *attributes[2];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP3T2),
        //                                     offsetof (VertexP3T2, x),
        //                                     3,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[1] = attribute_new (attribute_buffer,
        //                                     "tex_coord0_in",
        //                                     sizeof (VertexP3T2),
        //                                     offsetof (VertexP3T2, s),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     2);
        unimplemented!()
    }

    /// Provides a convenient way to describe a primitive, such as a single
    /// triangle strip or a triangle fan, that will internally allocate the
    /// necessary `AttributeBuffer` storage, describe the position, texture
    /// coordinate and color attributes with `Attribute`s and
    /// upload your data.
    ///
    /// For example to draw a convex polygon with texture mapping and a
    /// linear gradient you can do:
    ///
    /// ```text
    /// VertexP3T2C4 triangle[] =
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
    /// The primitive API doesn't support drawing with sliced
    /// textures (since switching between slices implies changing state and
    /// so that implies multiple primitives need to be submitted). You
    /// should pass the `TextureFlags::NoSlicing` flag to all textures that
    /// might be used while drawing with this API. If your hardware doesn't
    /// support non-power of two textures (For example you are using GLES
    /// 1.1) then you will need to make sure your assets are resized to a
    /// power-of-two size (though they don't have to be square)
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
    pub fn new_p3t2c4(context: &Context, mode: VerticesMode, data: &[&P3T2C4]) -> Primitive {
        // AttributeBuffer *attribute_buffer =
        //     attribute_buffer_new (context,
        //                             n_vertices * sizeof (VertexP3T2C4), data);
        // Attribute *attributes[3];

        // attributes[0] = attribute_new (attribute_buffer,
        //                                     "position_in",
        //                                     sizeof (VertexP3T2C4),
        //                                     offsetof (VertexP3T2C4, x),
        //                                     3,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[1] = attribute_new (attribute_buffer,
        //                                     "tex_coord0_in",
        //                                     sizeof (VertexP3T2C4),
        //                                     offsetof (VertexP3T2C4, s),
        //                                     2,
        //                                     ATTRIBUTE_TYPE_FLOAT);
        // attributes[2] = attribute_new (attribute_buffer,
        //                                     "color_in",
        //                                     sizeof (VertexP3T2C4),
        //                                     offsetof (VertexP3T2C4, r),
        //                                     4,
        //                                     ATTRIBUTE_TYPE_UNSIGNED_BYTE);

        // object_unref (attribute_buffer);

        // return _primitive_new_with_attributes_unref (mode, n_vertices,
        //                                                     attributes,
        //                                                     3);
        unimplemented!()
    }

    // primitive_new_with_attributes:
    // @mode: A #VerticesMode defining how to draw the vertices
    // @n_vertices: The number of vertices to process when drawing
    // @attributes: An array of Attribute
    // @n_attributes: The number of attributes
    //
    // Combines a set of #Attributes with a specific draw @mode
    // and defines a vertex count so a #Primitive object can be retained and
    // drawn later with no addition information required.
    //
    // The value passed as @n_vertices will simply update the
    // #Primitive <structfield>n_vertices</structfield> property as if
    // primitive_set_n_vertices() were called. This property defines
    // the number of vertices to read when drawing.
    //
    // Return value: (transfer full): A newly allocated #Primitive object
    pub fn with_attributes(
        mode: VerticesMode,
        n_vertices: i32,
        attributes: &[&Attribute],
        n_attributes: i32,
    ) -> Primitive {
        // Primitive *primitive;
        // int i;

        // primitive = g_slice_alloc (sizeof (Primitive) +
        //                             sizeof (Attribute *) * (n_attributes - 1));
        // primitive->mode = mode;
        // primitive->first_vertex = 0;
        // primitive->n_vertices = n_vertices;
        // primitive->indices = NULL;
        // primitive->immutable_ref = 0;

        // primitive->n_attributes = n_attributes;
        // primitive->n_embedded_attributes = n_attributes;
        // primitive->attributes = &primitive->embedded_attribute;
        // for (i = 0; i < n_attributes; i++)
        //     {
        //     Attribute *attribute = attributes[i];
        //     object_ref (attribute);

        //     _RETURN_VAL_IF_FAIL (is_attribute (attribute), NULL);

        //     primitive->attributes[i] = attribute;
        //     }

        // return _primitive_object_new (primitive);
        unimplemented!()
    }

    /// Makes a copy of an existing `Primitive`. Note that the primitive
    /// is a shallow copy which means it will use the same attributes and
    /// attribute buffers as the original primitive.
    ///
    /// # Returns
    ///
    /// the new primitive
    pub fn copy(&self) -> Option<Primitive> {
        // Primitive *copy;

        // copy = primitive_new_with_attributes (primitive->mode,
        //                                             primitive->n_vertices,
        //                                             primitive->attributes,
        //                                             primitive->n_attributes);

        // primitive_set_indices (copy, primitive->indices, primitive->n_vertices);
        // primitive_set_first_vertex (copy, primitive->first_vertex);

        // return copy;

        unimplemented!()
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
    pub fn draw<P: Is<Framebuffer>>(&self, framebuffer: &P, pipeline: &Pipeline) {
        // if (primitive->indices)
        //     _framebuffer_draw_indexed_attributes (framebuffer,
        //                                             pipeline,
        //                                             primitive->mode,
        //                                             primitive->first_vertex,
        //                                             primitive->n_vertices,
        //                                             primitive->indices,
        //                                             primitive->attributes,
        //                                             primitive->n_attributes,
        //                                             0);
        // else
        //     _framebuffer_draw_attributes (framebuffer,
        //                                     pipeline,
        //                                     primitive->mode,
        //                                     primitive->first_vertex,
        //                                     primitive->n_vertices,
        //                                     primitive->attributes,
        //                                     primitive->n_attributes,
        //                                     0);
        unimplemented!()
    }

    /// Iterates all the attributes of the given `Primitive`.
    /// ## `callback`
    /// A `PrimitiveAttributeCallback` to be
    ///  called for each attribute
    /// ## `user_data`
    /// Private data that will be passed to the
    ///  callback
    pub fn foreach_attribute<P: FnMut(&Primitive, &Attribute) -> i32>(&self, callback: P) {
        // int i;

        // for (i = 0; i < primitive->n_attributes; i++)
        //     if (!callback (primitive, primitive->attributes[i], user_data))
        //     break;
        unimplemented!()
    }

    pub fn first_vertex(&self) -> i32 {
        // _RETURN_VAL_IF_FAIL (is_primitive (primitive), 0);

        // return primitive->first_vertex;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the indices that were set with
    /// `Primitive::set_indices` or `None` if no indices were set.
    pub fn indices(&self) -> Option<Indices> {
        // return primitive->indices;
        unimplemented!()
    }

    pub fn mode(&self) -> VerticesMode {
        // _RETURN_VAL_IF_FAIL (is_primitive (primitive), 0);

        // return primitive->mode;
        unimplemented!()
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
    /// To be clear; it doesn't refer to the number of vertices - in
    /// terms of data - associated with the primitive it's just the number
    /// of vertices to read and draw.
    ///
    /// # Returns
    ///
    /// The number of vertices to read when drawing.
    pub fn n_vertices(&self) -> i32 {
        // _RETURN_VAL_IF_FAIL (is_primitive (primitive), 0);

        // return primitive->n_vertices;
        unimplemented!()
    }

    /// Replaces all the attributes of the given `Primitive` object::
    /// ## `attributes`
    /// an array of `Attribute` pointers
    /// ## `n_attributes`
    /// the number of elements in `attributes`
    pub fn set_attributes(&self, attributes: &[&Attribute], n_attributes: i32) {
        // int i;

        // _RETURN_IF_FAIL (is_primitive (primitive));

        // if (G_UNLIKELY (primitive->immutable_ref))
        //     {
        //     warn_about_midscene_changes ();
        //     return;
        //     }

        // NB: we don't unref the previous attributes before refing the new
        // in case we would end up releasing the last reference for an
        // attribute thats actually in the new list too. */
        // for (i = 0; i < n_attributes; i++)
        //     {
        //     _RETURN_IF_FAIL (is_attribute (attributes[i]));
        //     object_ref (attributes[i]);
        //     }

        // for (i = 0; i < primitive->n_attributes; i++)
        //     object_unref (primitive->attributes[i]);

        // First try to use the embedded storage assocated with the
        // primitive, else fallback to slice allocating separate storage for
        // the attribute pointers... */
        // if (n_attributes <= primitive->n_embedded_attributes)
        //     {
        //     if (primitive->attributes != &primitive->embedded_attribute)
        //         g_slice_free1 (sizeof (Attribute *) * primitive->n_attributes,
        //                     primitive->attributes);
        //     primitive->attributes = &primitive->embedded_attribute;
        //     }
        // else
        //     {
        //     if (primitive->attributes != &primitive->embedded_attribute)
        //         g_slice_free1 (sizeof (Attribute *) * primitive->n_attributes,
        //                     primitive->attributes);
        //     primitive->attributes =
        //         g_slice_alloc (sizeof (Attribute *) * n_attributes);
        //     }

        // memcpy (primitive->attributes, attributes,
        //         sizeof (Attribute *) * n_attributes);

        // primitive->n_attributes = n_attributes;
        unimplemented!()
    }

    pub fn set_first_vertex(&self, first_vertex: i32) {
        // _RETURN_IF_FAIL (is_primitive (primitive));

        // if (G_UNLIKELY (primitive->immutable_ref))
        //     {
        //     warn_about_midscene_changes ();
        //     return;
        //     }

        // primitive->first_vertex = first_vertex;
        unimplemented!()
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
    /// The `Primitive` `<structfield>`first_vertex`</structfield>` property
    /// also affects drawing with indices by defining the first entry of the
    /// indices to start drawing from.
    /// ## `indices`
    /// A `Indices` array
    /// ## `n_indices`
    /// The number of indices to reference when drawing
    pub fn set_indices(&self, indices: &Indices, n_indices: i32) {
        // _RETURN_IF_FAIL (is_primitive (primitive));

        // if (G_UNLIKELY (primitive->immutable_ref))
        //     {
        //     warn_about_midscene_changes ();
        //     return;
        //     }

        // if (indices)
        //     object_ref (indices);
        // if (primitive->indices)
        //     object_unref (primitive->indices);
        // primitive->indices = indices;
        // primitive->n_vertices = n_indices;
        unimplemented!()
    }

    pub fn set_mode(&self, mode: VerticesMode) {
        // _RETURN_IF_FAIL (is_primitive (primitive));

        // if (G_UNLIKELY (primitive->immutable_ref))
        //     {
        //     warn_about_midscene_changes ();
        //     return;
        //     }

        // primitive->mode = mode;
        unimplemented!()
    }

    /// Specifies how many vertices should be read when drawing the given
    /// `self`.
    ///
    /// Usually this value is set implicitly when associating vertex data
    /// or indices with a `Primitive`.
    ///
    /// To be clear; it doesn't refer to the number of vertices - in
    /// terms of data - associated with the primitive it's just the number
    /// of vertices to read and draw.
    /// ## `n_vertices`
    /// The number of vertices to read when drawing.
    pub fn set_n_vertices(&self, n_vertices: i32) {
        // _RETURN_IF_FAIL (is_primitive (primitive));

        // primitive->n_vertices = n_vertices;
        unimplemented!()
    }

    pub fn texture_set_auto_mipmap(primitive_texture: &PrimitiveTexture, value: bool) {
        // Texture *texture;

        // _RETURN_IF_FAIL (is_primitive_texture (primitive_texture));

        // texture = TEXTURE (primitive_texture);

        // g_assert (texture->vtable->set_auto_mipmap != NULL);

        // texture->vtable->set_auto_mipmap (texture, value);
        unimplemented!()
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Primitive")
    }
}
