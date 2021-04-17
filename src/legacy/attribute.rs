use crate::{AttributeBuffer, AttributeType, Context, Object};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Attribute(Object<ffi::CoglAttribute, AttributeClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_attribute_get_gtype(),
    }
}

impl Attribute {
    /// Describes the layout for a list of vertex attribute values (For
    /// example, a list of texture coordinates or colors).
    ///
    /// The `name` is used to access the attribute inside a GLSL vertex
    /// shader and there are some special names you should use if they are
    /// applicable:
    ///  `<itemizedlist>`
    ///  `<listitem>`"position_in" (used for vertex positions)`</listitem>`
    ///  `<listitem>`"color_in" (used for vertex colors)`</listitem>`
    ///  `<listitem>`"tex_coord0_in", "tex_coord1", ...
    /// (used for vertex texture coordinates)`</listitem>`
    ///  `<listitem>`"normal_in" (used for vertex normals)`</listitem>`
    ///  `<listitem>`"point_size_in" (used to set the size of points
    ///  per-vertex. Note this can only be used if
    ///  `COGL_FEATURE_ID_POINT_SIZE_ATTRIBUTE` is advertised and
    ///  `Pipeline::set_per_vertex_point_size` is called on the pipeline.
    ///  `</listitem>`
    ///  `</itemizedlist>`
    ///
    /// The attribute values corresponding to different vertices can either
    /// be tightly packed or interleaved with other attribute values. For
    /// example it's common to define a structure for a single vertex like:
    ///
    /// ```text
    /// typedef struct
    /// {
    ///   float x, y, z; /<!-- -->* position attribute *<!-- -->/
    ///   float s, t; /<!-- -->* texture coordinate attribute *<!-- -->/
    /// } MyVertex;
    /// ```
    ///
    /// And then create an array of vertex data something like:
    ///
    /// ```text
    /// MyVertex vertices[100] = { .... }
    /// ```
    ///
    /// In this case, to describe either the position or texture coordinate
    /// attribute you have to move `<literal>`sizeof (MyVertex)`</literal>` bytes to
    /// move from one vertex to the next. This is called the attribute
    /// `stride`. If you weren't interleving attributes and you instead had
    /// a packed array of float x, y pairs then the attribute stride would
    /// be `<literal>`(2 * sizeof (float))`</literal>`. So the `stride` is the number of
    /// bytes to move to find the attribute value of the next vertex.
    ///
    /// Normally a list of attributes starts at the beginning of an array.
    /// So for the `<literal>`MyVertex`</literal>` example above the `offset` is the
    /// offset inside the `<literal>`MyVertex`</literal>` structure to the first
    /// component of the attribute. For the texture coordinate attribute
    /// the offset would be `<literal>`offsetof (MyVertex, s)`</literal>` or instead of
    /// using the offsetof macro you could use `<literal>`sizeof (float) *
    /// 3`</literal>`. If you've divided your `array` into blocks of non-interleved
    /// attributes then you will need to calculate the `offset` as the number of
    /// bytes in blocks preceding the attribute you're describing.
    ///
    /// An attribute often has more than one component. For example a color
    /// is often comprised of 4 red, green, blue and alpha `components`, and a
    /// position may be comprised of 2 x and y `components`. You should aim
    /// to keep the number of components to a minimum as more components
    /// means more data needs to be mapped into the GPU which can be a
    /// bottlneck when dealing with a large number of vertices.
    ///
    /// Finally you need to specify the component data type. Here you
    /// should aim to use the smallest type that meets your precision
    /// requirements. Again the larger the type then more data needs to be
    /// mapped into the GPU which can be a bottlneck when dealing with
    /// a large number of vertices.
    /// ## `attribute_buffer`
    /// The `AttributeBuffer` containing the actual
    ///  attribute data
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `stride`
    /// The number of bytes to jump to get to the next attribute
    ///  value for the next vertex. (Usually
    ///  `<literal>`sizeof (MyVertex)`</literal>`)
    /// ## `offset`
    /// The byte offset from the start of `attribute_buffer` for
    ///  the first attribute value. (Usually
    ///  `<literal>`offsetof (MyVertex, component0)`</literal>`
    /// ## `components`
    /// The number of components (e.g. 4 for an rgba color or
    ///  3 for and (x,y,z) position)
    /// ## `type_`
    /// FIXME
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  describing the layout for a list of attribute values
    ///  stored in `array`.
    pub fn new(
        attribute_buffer: &AttributeBuffer,
        name: &str,
        stride: usize,
        offset: usize,
        components: i32,
        type_: AttributeType,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new(
                attribute_buffer.to_glib_none().0,
                name.to_glib_none().0,
                stride,
                offset,
                components,
                type_.to_glib(),
            ))
        }
    }

    /// Creates a new, single component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constant `value` is a single precision floating point scalar
    /// which should have a corresponding declaration in GLSL code like:
    ///
    /// [|
    /// attribute float name;
    /// |]
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// The constant value for the attribute
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant `value`.
    pub fn new_const_1f(context: &Context, name: &str, value: f32) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_1f(
                context.to_glib_none().0,
                name.to_glib_none().0,
                value,
            ))
        }
    }

    /// Creates a new, 2 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (`component0`, `component1`) represent a 2 component
    /// float vector which should have a corresponding declaration in GLSL
    /// code like:
    ///
    /// [|
    /// attribute vec2 name;
    /// |]
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `component0`
    /// The first component of a 2 component vector
    /// ## `component1`
    /// The second component of a 2 component vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_2f(
        context: &Context,
        name: &str,
        component0: f32,
        component1: f32,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_2f(
                context.to_glib_none().0,
                name.to_glib_none().0,
                component0,
                component1,
            ))
        }
    }

    /// Creates a new, 2 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (value[0], value[1]) represent a 2 component float
    /// vector which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute vec2 name;
    /// |]
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// A pointer to a 2 component float vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_2fv(context: &Context, name: &str, value: &[f32; 2]) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_2fv(
                context.to_glib_none().0,
                name.to_glib_none().0,
                value.as_ptr(),
            ))
        }
    }

    /// Creates a new matrix attribute whose value remains constant
    /// across all the vertices of a primitive without needing to duplicate
    /// the value for each vertex.
    ///
    /// `matrix2x2` represent a square 2 by 2 matrix specified in
    /// column-major order (each pair of consecutive numbers represents a
    /// column) which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute mat2 name;
    /// |]
    ///
    /// If `transpose` is `true` then all matrix components are rotated
    /// around the diagonal of the matrix such that the first column
    /// becomes the first row and the second column becomes the second row.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `matrix2x2`
    /// A pointer to a 2 by 2 matrix
    /// ## `transpose`
    /// Whether the matrix should be transposed on upload or
    ///  not
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant matrix.
    pub fn new_const_2x2fv(
        context: &Context,
        name: &str,
        matrix2x2: &[f32; 4],
        transpose: bool,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_2x2fv(
                context.to_glib_none().0,
                name.to_glib_none().0,
                matrix2x2.as_ptr(),
                transpose as i32,
            ))
        }
    }

    /// Creates a new, 3 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (`component0`, `component1`, `component2`) represent a 3
    /// component float vector which should have a corresponding
    /// declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec3 name;
    /// |]
    ///
    /// unless the built in name "normal_in" is being used where no
    /// explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `component0`
    /// The first component of a 3 component vector
    /// ## `component1`
    /// The second component of a 3 component vector
    /// ## `component2`
    /// The third component of a 3 component vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_3f(
        context: &Context,
        name: &str,
        component0: f32,
        component1: f32,
        component2: f32,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_3f(
                context.to_glib_none().0,
                name.to_glib_none().0,
                component0,
                component1,
                component2,
            ))
        }
    }

    /// Creates a new, 3 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (value[0], value[1], value[2]) represent a 3
    /// component float vector which should have a corresponding
    /// declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec3 name;
    /// |]
    ///
    /// unless the built in name "normal_in" is being used where no
    /// explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// A pointer to a 3 component float vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_3fv(context: &Context, name: &str, value: &[f32; 3]) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_3fv(
                context.to_glib_none().0,
                name.to_glib_none().0,
                value.as_ptr(),
            ))
        }
    }

    /// Creates a new matrix attribute whose value remains constant
    /// across all the vertices of a primitive without needing to duplicate
    /// the value for each vertex.
    ///
    /// `matrix3x3` represent a square 3 by 3 matrix specified in
    /// column-major order (each triple of consecutive numbers represents a
    /// column) which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute mat3 name;
    /// |]
    ///
    /// If `transpose` is `true` then all matrix components are rotated
    /// around the diagonal of the matrix such that the first column
    /// becomes the first row and the second column becomes the second row
    /// etc.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `matrix3x3`
    /// A pointer to a 3 by 3 matrix
    /// ## `transpose`
    /// Whether the matrix should be transposed on upload or
    ///  not
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant matrix.
    pub fn new_const_3x3fv(
        context: &Context,
        name: &str,
        matrix3x3: &[f32; 9],
        transpose: bool,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_3x3fv(
                context.to_glib_none().0,
                name.to_glib_none().0,
                matrix3x3.as_ptr(),
                transpose as i32,
            ))
        }
    }

    /// Creates a new, 4 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (`component0`, `component1`, `component2`, `constant3`)
    /// represent a 4 component float vector which should have a
    /// corresponding declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec4 name;
    /// |]
    ///
    /// unless one of the built in names "color_in",
    /// "tex_coord0_in or "tex_coord1_in" etc is being used where
    /// no explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `component0`
    /// The first component of a 4 component vector
    /// ## `component1`
    /// The second component of a 4 component vector
    /// ## `component2`
    /// The third component of a 4 component vector
    /// ## `component3`
    /// The fourth component of a 4 component vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_4f(
        context: &Context,
        name: &str,
        component0: f32,
        component1: f32,
        component2: f32,
        component3: f32,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_4f(
                context.to_glib_none().0,
                name.to_glib_none().0,
                component0,
                component1,
                component2,
                component3,
            ))
        }
    }

    /// Creates a new, 4 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (value[0], value[1], value[2], value[3]) represent a
    /// 4 component float vector which should have a corresponding
    /// declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec4 name;
    /// |]
    ///
    /// unless one of the built in names "color_in",
    /// "tex_coord0_in or "tex_coord1_in" etc is being used where
    /// no explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// A pointer to a 4 component float vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_4fv(context: &Context, name: &str, value: &[f32; 4]) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_4fv(
                context.to_glib_none().0,
                name.to_glib_none().0,
                value.as_ptr(),
            ))
        }
    }

    /// Creates a new matrix attribute whose value remains constant
    /// across all the vertices of a primitive without needing to duplicate
    /// the value for each vertex.
    ///
    /// `matrix4x4` represent a square 4 by 4 matrix specified in
    /// column-major order (each 4-tuple of consecutive numbers represents a
    /// column) which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute mat4 name;
    /// |]
    ///
    /// If `transpose` is `true` then all matrix components are rotated
    /// around the diagonal of the matrix such that the first column
    /// becomes the first row and the second column becomes the second row
    /// etc.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `matrix4x4`
    /// A pointer to a 4 by 4 matrix
    /// ## `transpose`
    /// Whether the matrix should be transposed on upload or
    ///  not
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant matrix.
    pub fn new_const_4x4fv(
        context: &Context,
        name: &str,
        matrix4x4: &[f32; 16],
        transpose: bool,
    ) -> Attribute {
        unsafe {
            from_glib_full(ffi::cogl_attribute_new_const_4x4fv(
                context.to_glib_none().0,
                name.to_glib_none().0,
                matrix4x4.as_ptr(),
                transpose as i32,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the `AttributeBuffer` that was
    ///  set with `Attribute::set_buffer` or `Attribute::new`.
    pub fn get_buffer(&self) -> Option<AttributeBuffer> {
        unsafe { from_glib_none(ffi::cogl_attribute_get_buffer(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the value of the normalized property set with
    /// `Attribute::set_normalized`.
    pub fn get_normalized(&self) -> bool {
        unsafe { ffi::cogl_attribute_get_normalized(self.to_glib_none().0) == crate::TRUE }
    }

    /// Sets a new `AttributeBuffer` for the attribute.
    /// ## `attribute_buffer`
    /// A `AttributeBuffer`
    pub fn set_buffer(&self, attribute_buffer: &AttributeBuffer) {
        unsafe {
            ffi::cogl_attribute_set_buffer(
                self.to_glib_none().0,
                attribute_buffer.to_glib_none().0,
            );
        }
    }

    /// Sets whether fixed point attribute types are mapped to the range
    /// 0→1. For example when this property is TRUE and a
    /// `AttributeType::UnsignedByte` type is used then the value 255
    /// will be mapped to 1.0.
    ///
    /// The default value of this property depends on the name of the
    /// attribute. For the builtin properties color_in and
    /// normal_in it will default to TRUE and for all other names it
    /// will default to FALSE.
    /// ## `normalized`
    /// The new value for the normalized property.
    pub fn set_normalized(&self, normalized: bool) {
        unsafe {
            ffi::cogl_attribute_set_normalized(self.to_glib_none().0, normalized as i32);
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute")
    }
}
